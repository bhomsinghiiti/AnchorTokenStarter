# AnchorTokenStarter

A Solana/Anchor project demonstrating token operations including mint creation, token minting, and transfers. This starter template provides essential building blocks for Solana token development.

**Program ID**: `AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa`

## For Solidity Developers

If you're coming from Ethereum/Solidity development, this project will help you understand the key differences between Foundry and Anchor frameworks.

## Quick Start

### Prerequisites

This project requires WSL2 on Windows. Install dependencies:

```bash
# Quick install all dependencies
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
source ~/.cargo/env
source ~/.bashrc

# Verify installations
rustc --version  # 1.93.0
solana --version  # 3.1.8
anchor --version  # 0.32.1
node --version    # v24.10.0
yarn --version    # 1.22.22
```

### Build & Test

```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run all tests (auto-starts validator)
anchor test

# Run tests without rebuilding
anchor test --skip-build
```

## Deployment

### Local Deployment

**Option 1: Manual validator (Recommended for learning)**

```bash
# Terminal 1: Start local validator
solana-test-validator --ledger test-ledger

# Expected output:
# Identity: AiUMAEvPB3ksoPyJFbJng3KUW5NzJ32mo5PU9rEnq6bN
# Genesis Hash: 9YewJuCeQdFRg42zyLvjjUeLN7Kt4YavALroxNgoHsm7
# JSON RPC URL: http://127.0.0.1:8899

# Terminal 2: Deploy
anchor build
anchor deploy

# Verify deployment
solana program show AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa
```

**Option 2: Auto-managed (anchor test)**

```bash
# anchor test automatically starts/stops validator
anchor test
```

### Deployment Output Explained

```
Deploying cluster: http://127.0.0.1:8899
```
→ RPC endpoint (like `http://localhost:8545` in Foundry)

```
Upgrade authority: /home/bhom/.config/solana/id.json
```
→ Your wallet controls program upgrades (like deployer key in Foundry)

```
Program path: target/deploy/anchortokenstarter.so
```
→ Compiled BPF bytecode (~278KB, vs Foundry's ~200KB JSON)

```
Program Id: AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa
```
→ **Key difference**: Address baked into binary at compile time via `declare_id!()`
→ In Foundry: Address calculated as `keccak256(deployer, nonce)`

```
Signature: 2QRpy54b7xkyiM61wygLdoGJh7ejvJ5ihUm1zTSaVSUe9MdqmRWbKspfD6M8A7hgk...
```
→ Transaction signature (like tx hash in Foundry)

```
Waiting for program to be confirmed...
Program confirmed on-chain
```
→ Transaction included in a slot (like block confirmation in Foundry)

```
Idl data length: 890 bytes
Idl account created: 4FXGm6hhHiSfSjtQtrjooaagZSzpraMWzDDJ7MVLzbP3
```
→ **Anchor-specific**: Interface Definition Language stored on-chain
→ No Foundry equivalent (similar to ABI, but stored on-chain)

```
Deploy success
```

### On-Chain Structure

```
┌──────────────────────────────────────┐
│  Program Account (AT3LVe...)         │
│  ├─ Owner: BPFLoaderUpgradeable      │
│  ├─ Data: [BPF bytecode ~278KB]      │
│  ├─ Upgrade Authority: Your Wallet   │
│  └─ Balance: ~1.94 SOL (rent exempt) │
└──────────────────────────────────────┘
       │
       │ References
       ▼
┌──────────────────────────────────────┐
│  IDL Account (4FXGm6hh...)           │
│  ├─ Owner: Your Program              │
│  ├─ Data: [IDL JSON ~890 bytes]      │
│  └─ Contains: Function signatures,   │
│               account types, errors  │
└──────────────────────────────────────┘
```

## Foundry vs Anchor Comparison

| Concept | Foundry (Ethereum) | Anchor (Solana) |
|---------|-------------------|-----------------|
| **Binary** | `MyToken.json` (ABI + bytecode) | `anchortokenstarter.so` (BPF bytecode) |
| **Address** | Calculated from sender + nonce | Baked into binary via `declare_id!()` |
| **Key needed** | Just private key for signing | Program keypair must match declared ID |
| **Deployment** | `forge script Deploy.s.s --broadcast` | `anchor deploy` |
| **Local node** | `anvil` | `solana-test-validator` |
| **State** | Contract storage | Separate accounts (PDAs) |
| **Upgradeable** | Proxy patterns (Transparent/UUPS) | Native via `BPFLoaderUpgradeable` |
| **Interface** | ABI (off-chain JSON) | IDL (on-chain account) |
| **Gas** | Ethereum gas + gas price | Compute units + lamports |
| **Account model** | EOA/Contract accounts | Everything is an account |

### Deployment Flow Comparison

```
┌─────────────────────────────────────────────────────────────────────┐
│                        FOUNDRY DEPLOYMENT                           │
├─────────────────────────────────────────────────────────────────────┤
│ 1. anvil                      # Start local node                    │
│    -> Chain ID: 31337                                         │
│    -> RPC: http://localhost:8545                                 │
│                                                                     │
│ 2. forge script Deploy.s.s --broadcast                              │
│    -> Compiles contract                                           │
│    -> Calculates address: keccak256(deployer, nonce)              │
│    -> Sends transaction                                           │
│    -> Returns: 0xabc... (contract address)                        │
│    -> Returns: 0xdef... (tx hash)                                 │
│                                                                     │
│ 3. Contract storage lives IN the contract account                  │
│                                                                     │
│ 4. cast call 0xabc... "function()" # Interact                      │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                        ANCHOR DEPLOYMENT                            │
├─────────────────────────────────────────────────────────────────────┤
│ 1. solana-test-validator        # Start local validator             │
│    -> Genesis Hash: 9YewJu...                                  │
│    -> RPC: http://127.0.0.1:8899                                 │
│                                                                     │
│ 2. anchor deploy               # Deploy program                     │
│    -> Compiles to .so (BPF bytecode)                              │
│    -> Address baked in: declare_id!("AT3LVe...")                  │
│    -> Creates Program account                                     │
│    -> Uploads bytecode to account                                 │
│    -> Creates IDL account (4FXGm6hh...)                           │
│    -> Returns: AT3LVe... (program ID)                             │
│    -> Returns: 2QRpy... (tx signature)                            │
│                                                                     │
│ 3. State lives in SEPARATE accounts (PDAs), not in program         │
│                                                                     │
│ 4. anchor test / solana program show AT3LVe... # Interact          │
└─────────────────────────────────────────────────────────────────────┘
```

## Solidity to Rust Syntax Reference

| Solidity | Anchor/Rust |
|----------|-------------|
| `contract MyToken` | `#[program] pub mod my_token` |
| `constructor()` | `initialize()` |
| `function mint()` | `pub fn mint()` |
| `msg.sender` | `ctx.accounts.signer` |
| `public/external` | `pub fn` |
| `require()` | `require!()` macro |
| `emit Event()` | `msg!()` macro |
| `mapping(address => uint)` | `#[account]` struct |

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

## Program Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Basic program initialization |
| `increment` | Simple counter with PDA |
| `createMint` | Create a new token mint |
| `mintTokens` | Mint tokens to associated token account |
| `transferTokens` | Transfer tokens between accounts |

## Useful Commands

```bash
# Build
anchor build

# Test
anchor test

# Deploy to localnet
anchor deploy

# Deploy to devnet
anchor deploy --provider.cluster devnet

# View program on-chain
solana program show AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa

# View all programs on localnet
solana program show --programs

# Check program keypairs
anchor keys list

# Get program keypair pubkey
solana-keygen pubkey target/deploy/anchortokenstarter-keypair.json

# Upgrade program
anchor upgrade target/deploy/anchortokenstarter.so --program-id AT3LVe...

# View account balance
solana balance

# Request airdrop (devnet/testnet only)
solana airdrop 2

# Configure Solana CLI
solana config set --url devnet
solana config set --keypair ~/.config/solana/id.json
```

## Project Structure

```
.
├── programs/
│   └── anchortokenstarter/
│       └── src/
│           └── lib.rs              # Main program logic
├── tests/
│   └── anchortokenstarter.ts       # TypeScript tests
├── Anchor.toml                      # Anchor configuration
├── Cargo.toml                       # Rust workspace config
├── rust-toolchain.toml              # Rust version pin
├── CLAUDE.md                        # Claude Code instructions
└── target/
    └── deploy/
        ├── anchortokenstarter.so   # Compiled program
        └── anchortokenstarter-keypair.json
```

## Key Concepts

### PDAs (Program Derived Addresses)
Deterministic addresses derived from program ID and seeds:
```rust
#[account(
    seeds = [b"counter"],
    bump
)]
pub counter: Account<'info, Counter>,
```

### Account Validation
All accounts validated BEFORE instruction executes:
```rust
#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    // ...
}
```

### CPI (Cross-Program Invocation)
Calling other programs (like SPL Token):
```rust
token_interface::mint_to(cpi_context, amount)?;
```

### State Storage Model
```solidity
// Solidity - Contract storage
contract Counter {
    uint256 public count = 0;
}
```

```rust
// Anchor - Separate account (rent-exempt)
#[account]
pub struct Counter {
    pub count: u64,
}
```

## Tech Stack

| Tool | Version | Notes |
|------|---------|-------|
| Rust | 1.93.0 stable | Pinned in `rust-toolchain.toml` |
| Solana CLI | 3.1.8 | Agave client |
| Anchor CLI | 0.32.1 | Via AVM |
| Node.js | v24.10.0 | For TypeScript tests |
| Yarn | 1.22.22 | Package manager |

## License

ISC
