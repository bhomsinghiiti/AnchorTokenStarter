# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Solana/Anchor project - a starter template for Solana program development. The project is currently a minimal "Hello World" program with a single `initialize` instruction.

**Program ID**: `4axna8Sy83cq21JRGeJnEunDDPZNGiiF1NGuoT6WZrBJ`

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
