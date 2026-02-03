# Account Model

**Deep dive:** Understanding Solana's separation of programs and data

---

## Overview

Solana uses an **account-based model** where code (programs) and data (accounts) are completely separate. This is fundamentally different from Ethereum's contract model where code and state live together.

---

## Ethereum vs Solana Comparison

### Ethereum/ERC20 Model

```
┌─────────────────────────────────┐
│   MyToken Contract Address      │
│   ┌─────────────────────────┐   │
│   │ Code (bytecode)         │   │
│   │ - transfer()            │   │
│   │ - balanceOf()           │   │
│   │ - totalSupply()         │   │
│   └─────────────────────────┘   │
│   ┌─────────────────────────┐   │
│   │ Storage                 │   │
│   │ balances[user1]         │   │
│   │ balances[user2]         │   │
│   │ totalSupply             │   │
│   └─────────────────────────┘   │
└─────────────────────────────────┘

One address = Code + State combined
```

### Solana/SPL Token Model

```
┌──────────────────────┐
│  Token Program       │
│  (Code only)         │
│  - transfer()        │
│  - mint_to()         │
│  - burn()            │
└──────────────────────┘
        │
        │ Uses
        ▼
┌──────────────────────┐       ┌──────────────────────┐
│  Mint Account        │       │  Token Account       │
│  (Data)              │       │  (Data)              │
│  - supply            │       │  - amount            │
│  - decimals          │       │  - owner             │
│  - authority         │       │  - mint              │
└──────────────────────┘       └──────────────────────┘
                                       │
                                       │ Many per mint
                                       ▼
                              ┌──────────────────────┐
                              │  User 1's ATA        │
                              │  User 2's ATA        │
                              │  User 3's ATA        │
                              └──────────────────────┘

Separate addresses: Program + Mint + Token Accounts
```

---

## Key Concepts

### Programs are Stateless

**Solana programs:**
- Contain only code (logic)
- Have no storage of their own
- Can be upgraded (if upgrade authority exists)
- One program can operate on infinite accounts

**Implication:** You don't deploy a new program for each token/mint. One Token Program serves all tokens.

### Accounts are Data Storage

**Solana accounts:**
- Store all state/data
- Have their own address and balance
- Owned by a program (that program can modify the data)
- Must be rent-exempt (hold minimum SOL)

**Account types:**
- **Program accounts** - Executable code (your programs)
- **Data accounts** - State storage (your structs)
- **System accounts** - Native accounts (Clock, Config, etc.)
- **Token accounts** - SPL Token mint/token accounts

### Account Ownership

Each account has an **owner** (program ID):
- Only the owning program can modify the data
- Anyone can read the data (if not marked `readonly` in instruction)
- Ownership can be changed (with owner's permission)

```
┌────────────────────┐
│ Counter Account    │
│ Owner: Your Program│
│ Data: {count: 5}   │  ← Only your program can modify
└────────────────────┘
```

### Rent Exemption

Accounts must hold minimum SOL to exist (**rent exemption**):
- Prevents spam on the network
- Amount depends on account size
- Can be reclaimed if account is closed

**Rent calculation:**
```
rent_exempt_minimum = data_size * rent_per_byte_per_year
```

**Practice:** Anchor's `init` constraint automatically handles rent exemption.

---

## PDAs: Program-Owned Addresses

**PDA (Program Derived Address)** = Deterministic address controlled by a program

```rust
// Derivation
let seeds = [b"counter"];
let [pda, bump] = PublicKey.findProgramAddress(seeds, program_id);
```

**Properties:**
- Deterministic: Same seeds + program = same address
- Program-controlled: Program can sign for the PDA
- No private key: Bump seed acts as "signature"

**Uses:**
- Single-instance config accounts
- Token mint authority (program can mint)
- Escrow accounts
- Anonymous auction bids

---

## Account Constraints in Anchor

Anchor uses `#[account]` attributes to validate and manage accounts:

```rust
#[account(
    init,                    // Create account
    payer = payer,           // Who pays rent
    space = 8 + Counter::INIT_SPACE,  // Size
    seeds = [b"counter"],    // PDA seeds
    bump                     // Store bump
)]
pub counter: Account<'info, Counter>,
```

**Common constraints:**
- `init` / `init_if_needed` - Create account
- `mut` - Account will be modified
- `seeds` / `bump` - PDA derivation
- `payer` - Pays rent and fees
- `space` - Account size
- `owner` - Set account owner

---

## See This in Action

**Patterns using account model:**
- [Simple Counter](../patterns/initialization/simple-counter.md) - State account with PDA
- [Create Mint](../patterns/token-operations/create-mint.md) - Token mint as separate account
- [Config Account](../patterns/initialization/config-account.md) - Multi-field state

**Concepts:**
- [PDAs Explained](pdas-explained.md) - Deep dive on PDAs
- [Anchor Macros](anchor-macros.md) - Constraint reference

**Reference:**
- [Solidity Comparison](../reference/solidity-comparison.md) - ERC20 vs SPL Token
