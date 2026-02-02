# AnchorTokenStarter

A Solana/Anchor project demonstrating token operations including mint creation, token minting, and transfers.

## Features

- **Initialize**: Basic program initialization
- **Counter**: Simple counter with PDA (Program Derived Address)
- **Token Mint**: Create new SPL token mints
- **Mint Tokens**: Mint tokens to associated token accounts
- **Transfer Tokens**: Transfer tokens between accounts

## Tech Stack

| Tool | Version |
|------|---------|
| Rust | 1.93.0 stable |
| Solana CLI | 3.1.8 |
| Anchor CLI | 0.32.1 |
| Node.js | v24.10.0 |
| Yarn | 1.22.22 |

## Quick Start

### Prerequisites

This project requires WSL2 on Windows. Install dependencies:

```bash
curl --proto '=https' --tlsv1.2 -sSfL https://solana-install.solana.workers.dev | bash
source ~/.cargo/env
```

### Installation

```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test
```

## Project Structure

```
AnchorTokenStarter/
├── programs/
│   └── anchortokenstarter/
│       └── src/
│           └── lib.rs          # Main program logic
├── tests/
│   └── anchortokenstarter.ts    # TypeScript tests
├── migrations/
│   └── deploy.ts                # Deployment script
├── Anchor.toml                  # Anchor configuration
├── Cargo.toml                   # Rust workspace config
└── rust-toolchain.toml          # Rust version pin
```

## Usage

### Build

```bash
anchor build
```

### Test

```bash
anchor test
```

### Deploy

```bash
# To localnet
anchor deploy

# To devnet
anchor deploy --provider.cluster devnet
```

## Program Instructions

| Instruction | Description |
|-------------|-------------|
| `initialize` | Initialize the program |
| `increment` | Increment a counter |
| `create_mint` | Create a new token mint |
| `mint_tokens` | Mint tokens to an ATA |
| `transfer_tokens` | Transfer tokens between accounts |

## License

ISC
