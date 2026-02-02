# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**AnchorTokenStarter** - A Solana/Anchor project demonstrating token operations including mint creation, token minting, and transfers. This starter template provides essential building blocks for Solana token development.

**Program ID**: `AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa`

## Working Versions (Verified)

The following versions are tested and working together:

| Tool | Version | Notes |
|------|---------|-------|
| Rust | 1.93.0 stable | Pinned in `rust-toolchain.toml` |
| Solana CLI | 3.1.8 | Agave client |
| Anchor CLI | 0.32.1 | Via AVM |
| Node.js | v24.10.0 | For TypeScript tests |
| Yarn | 1.22.22 | Package manager |

## WSL/Windows Setup

### Prerequisites

If you're on Windows, you must use WSL (Windows Subsystem for Linux). This project is developed and tested on WSL2 with Ubuntu.

### Initial Setup Commands

Run these commands in your WSL Ubuntu terminal to install all dependencies:

```bash
# Quick install all dependencies (Rust, Solana CLI, Anchor CLI, Node.js, Yarn)
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash

# Reload your shell to apply PATH changes
source ~/.cargo/env
source ~/.bashrc  # or source ~/.zshrc depending on your shell

# Verify installations
rustc --version && solana --version && anchor --version && node --version && yarn --version
```

### Manual Installation (If Quick Install Fails)

If the quick install doesn't work, install dependencies individually:

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source ~/.cargo/env

# Install Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"

# Install AVM (Anchor Version Manager)
cargo install --git https://github.com/coral-xyz/anchor avm --force

# Install and use Anchor CLI 0.32.1
avm install 0.32.1
avm use 0.32.1

# Install Node.js (if not already installed)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
source ~/.bashrc
nvm install 24
nvm use 24

# Install Yarn
npm install -g yarn
```

### Linux Dependencies

If you encounter build errors on WSL/Ubuntu, you may need to install system dependencies:

```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libudev-dev \
    protobuf-compiler
```

## Common Commands

### Building and Testing
```bash
# Build the program
anchor build

# Run all tests (starts local validator, runs tests, stops validator)
anchor test

# Run tests without rebuilding
anchor test --skip-build

# Run a specific test file
anchor test --skip-build my-first-project
```

### Deployment
```bash
# Deploy to configured cluster (localnet by default)
anchor deploy

# Deploy to specific cluster
anchor deploy --provider.cluster devnet
anchor deploy --provider.cluster mainnet-beta
```

#### Local Deployment Flow

For localnet testing, you have two options:

**Option 1: Manual validator start (Recommended for debugging)**
```bash
# Terminal 1: Start local validator
solana-test-validator --ledger /tmp/test-ledger

# Terminal 2: Deploy your program
anchor build
anchor deploy

# Verify deployment
solana program show AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa
```

**Option 2: Auto-managed (anchor test)**
```bash
# anchor test automatically starts/stops validator for you
anchor test
```

#### Program ID Management

Unlike Ethereum where contract addresses are calculated from deployer + nonce, Solana program IDs are baked into the binary:

```bash
# View your program keypairs
anchor keys list

# If you need to sync the declared ID with the keypair:
solana-keygen pubkey target/deploy/anchortokenstarter-keypair.json
# Then update declare_id!() in lib.rs to match
```

#### Foundry vs Anchor Deployment Comparison

| Concept | Foundry/Ethereum | Anchor/Solana |
|---------|------------------|---------------|
| **Binary** | `MyToken.json` (ABI + bytecode) | `anchortokenstarter.so` (BPF bytecode) |
| **Address** | Calculated from sender + nonce | Baked into binary via `declare_id!()` |
| **Key needed** | Just private key for signing | Program keypair must match declared ID |
| **Deployment** | `forge script Deploy.s.s --broadcast` | `anchor deploy` |
| **Local node** | `anvil` | `solana-test-validator` |
| **Upgradeable** | Proxy patterns (Transparent/UUPS) | Native via `BPFLoaderUpgradeable` |

### Other Useful Commands
```bash
# Generate new program keypair
anchor keys list

# Verify program on chain
anchor verify <PROGRAM_ID>

# Upgrade existing program
anchor upgrade <PROGRAM_SO_FILE> --program-id <PROGRAM_ID>
```

### Code Quality
```bash
# Format code
yarn run lint:fix

# Check code formatting
yarn run lint
```

## Solidity to Rust Reference

For Solidity developers transitioning to Anchor/Solana:

### Basic Syntax Mapping

| Solidity | Anchor/Rust | This Project |
|----------|-------------|--------------|
| `contract MyToken` | `#[program] pub mod my_token` | `anchortokenstarter` |
| `constructor()` | `initialize()` | Initialize instruction |
| `function mint()` | `pub fn mint()` | `create_mint`, `mint_tokens` |
| `msg.sender` | `ctx.accounts.signer` | Signer accounts |
| `public/external` | `pub fn` | All instruction handlers |
| `require()` | `require!()` macro | Validation |
| `emit Event()` | `msg!()` macro | Logging |
| `mapping(address => uint)` | `#[account]` struct | `Counter` account |

### Account Validation vs Function Parameters

```solidity
// Solidity - Simple parameters
function mint(address to, uint256 amount) external {
    balances[to] += amount;
}
```

```rust
// Anchor - Accounts struct (validated BEFORE execution)
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)] pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
}

pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    // CPI to SPL Token program
    token_interface::mint_to(cpi_context, amount)?;
    Ok(())
}
```

### State Storage Model

```solidity
// Solidity - Contract storage
contract Counter {
    uint256 public count = 0;
    function increment() external { count++; }
}
```

```rust
// Anchor - Separate account (rent-exempt)
#[account]
pub struct Counter {
    pub count: u64,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        init_if_needed,
        payer = payer,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter"],
        bump
    )]
    pub counter: Account<'info, Counter>,
    pub payer: Signer<'info>,
}
```

### Key Differences

1. **Accounts vs Storage**: Solana uses separate accounts (PDAs) instead of contract storage
2. **Account Validation**: Anchor validates all accounts before instruction executes
3. **Rent**: Accounts must hold minimum SOL balance to exist (rent-exempt)
4. **PDAs**: Program Derived Addresses for deterministic account addresses
5. **CPI**: Cross-Program Invocation to call other programs (like SPL Token)

## Architecture

### Program Structure

The Solana program is located in `programs/my-first-project/src/lib.rs`:

- **`declare_id!` macro**: Defines the on-chain program ID
- **`#[program]` mod**: Contains all instruction handlers
- **Instruction handlers**: Functions that take a `Context` and return `Result<()>`
- **`#[derive(Accounts)]` structs**: Validate and deserialize accounts passed to instructions

### Adding Instructions

To add a new instruction:

1. Add a function inside the `#[program]` mod:
```rust
pub fn my_instruction(ctx: Context<MyAccounts>, param: u64) -> Result<()> {
    // Your logic here
    Ok(())
}
```

2. Create the corresponding accounts struct:
```rust
#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    // Add more accounts here
}
```

3. Rebuild: `anchor build`

### Testing

Tests are written in TypeScript in the `tests/` directory using Mocha:

- Tests automatically get the program from `anchor.workspace.myFirstProject`
- Call instructions via `program.methods.<instruction_name>(<params>).rpc()`
- The test runner uses a very long timeout (1M ms) configured in Anchor.toml

### State Management Patterns

When adding state, you'll typically use:
- **`#[account]`** attribute on account structs for automatic serialization
- **`#[account(init)]`** for creating new program-owned accounts
- **`#[account(mut)]`` for accounts that will be modified
- **PDAs (Program Derived Addresses)** using `pubkey.findProgramAddress()` for deterministic addresses

### Configuration Files

- **`Anchor.toml`**: Main configuration - defines clusters, programs, test scripts
- **`programs/*/Cargo.toml`**: Rust dependencies for the program
- **`rust-toolchain.toml`**: Pins Rust version to 1.93.0
- **`tsconfig.json`**: TypeScript configuration for tests

## Rust Toolchain

This project uses Rust 1.93.0 stable with rustfmt and clippy components (profile: minimal).
