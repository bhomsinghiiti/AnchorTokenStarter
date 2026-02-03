# PDAs Explained

**Concept:** Program Derived Addresses - deterministic addresses controlled by programs

---

## Overview

A **PDA (Program Derived Address)** is a special Solana address that:
- Is deterministically derived from seeds and a program ID
- Has no private key
- Can be signed for by its owning program
- Acts as a program-controlled account

---

## Why PDAs Matter

**Problem:** How can a program control an account without a private key?

**Solution:** PDAs allow programs to sign for addresses they control.

**Use cases:**
- Token mint authority (program-controlled minting)
- Single-instance config accounts
- Escrow accounts
- Auction bids
- Liquidity pool positions
- Any program-owned state

---

## PDA Derivation

### Formula

```rust
PDA = findProgramAddress(seeds, program_id)
```

**Components:**
- **seeds** - Arbitrary bytes you choose (e.g., `b"config"`, user pubkey)
- **program_id** - Your program's address
- **bump** - Value that makes the address fall off Ed25519 curve

### How It Works

```
1. Start with: program_id + seeds
2. Try: HASH(program_id || seeds || 0)
3. If on Ed25519 curve: FAIL (would have private key)
4. Try: HASH(program_id || seeds || 1)
5. If on Ed25519 curve: FAIL
6. ...
7. Try: HASH(program_id || seeds || 255)
8. NOT on curve: SUCCESS! This is the PDA
```

The "bump" (0-255) is the value that makes the address NOT a valid Ed25519 public key (no private key exists).

### In Anchor

**Deriving in tests:**
```typescript
const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
);
```

**Validating in program:**
```rust
#[account(
    seeds = [b"config"],
    bump
)]
pub config: Account<'info, Config>,
```

Anchor automatically:
1. Derives the PDA from seeds
2. Verifies the passed account matches
3. Checks the bump
4. Stores bump in `ctx.bumps.config`

---

## PDA Signing

### The Magic: Programs Can Sign!

Since PDAs have no private key, how do we sign transactions?

**Answer:** The program signs using the seeds and bump.

### Example: Mint Tokens

```rust
// PDA seeds
let seeds: &[&[&[u8]]] = &[&[b"mint"], &[ctx.bumps.mint]];

// CPI to SPL Token
let cpi_context = CpiContext::new(cpi_program, cpi_accounts)
    .with_signer(seeds);  // ← Program signs for PDA!

token_interface::mint_to(cpi_context, amount)?;
```

**What happens:**
1. Your program provides the seeds and bump
2. SPL Token program verifies the seeds+bump derive the PDA
3. SPL Token program checks that PDA is the mint authority
4. SPL Token program accepts this as a valid "signature"

### In Solidity Terms

**Solidity:**
```solidity
// Only external accounts can sign
function mint(address to, uint256 amount) external {
    require(msg.sender == mintAuthority, "Not authorized");
    _mint(to, amount);
}
```

**Solana with PDA:**
```rust
// Program can sign for its own PDA
pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
    let seeds = &[&[b"mint"], &[ctx.bumps.mint]];
    let cpi = CpiContext::new(...).with_signer(seeds);
    token_interface::mint_to(cpi, amount)?;  // Program signed!
}
```

---

## Common Seed Patterns

### Single Instance (Global Config)

```rust
// One config per program
seeds = [b"config"]
```

### Per-User Accounts

```rust
// Separate account for each user
seeds = [b"user", user.key().as_ref()]
```

### Per-Mint Accounts

```rust
// Account for each token mint
seeds = [b"vault", mint.key().as_ref()]
```

### Multi-Seeds

```rust
// Complex derivation with multiple seeds
seeds = [
    b"auction",
    auction_id.as_ref(),
    mint.as_ref()
]
```

---

## Bump Seeds

### Canonical Bump

The "canonical bump" is the first bump (starting from 255) that produces a valid PDA.

**Finding it:**
```rust
// Anchor does this automatically
let (pda, bump) = Pubkey::find_program_address(&[b"config"], program_id);
// bump = 255 (usually)
```

**Storing it:**
```rust
#[account(
    seeds = [b"config"],
    bump  // ← Store bump in account data
)]
pub config: Account<'info, Config>,
```

**Using it:**
```rust
let seeds = &[&[b"config"], &[ctx.bumps.config]];
```

### Why Store the Bump?

1. **Efficiency:** Don't need to re-derive
2. **Verification:** Ensures the account was created by your program
3. **Security:** Prevents passing arbitrary accounts

---

## See This in Action

**Patterns using PDAs:**
- [Simple Counter](../patterns/initialization/simple-counter.md) - PDA for counter account
- [Create Mint](../patterns/token-operations/create-mint.md) - PDA as mint authority
- [Mint Tokens](../patterns/token-operations/mint-tokens.md) - PDA signing in action

**Concepts:**
- [Account Model](account-model.md) - How PDAs fit into Solana's model
- [Anchor Macros](anchor-macros.md) - PDA constraint attributes

**Reference:**
- [Solidity Comparison](../reference/solidity-comparison.md) - Contract storage vs PDAs
