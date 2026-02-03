# Anchor Macros

**Reference:** Complete guide to `#[account]` attributes and constraints

---

## Overview

Anchor uses procedural macros to:
- Validate accounts before your instruction runs
- Deserialize account data automatically
- Handle common Solana patterns (PDAs, CPI, etc.)
- Reduce boilerplate code

---

## `#[derive(Accounts)]`

The main macro for account validation structs:

```rust
#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(...)]
    pub my_account: Account<'info, MyData>,
}
```

**What it does:**
1. Validates each account against constraints
2. Deserializes data into Rust structs
3. Generates helpful error messages
4. Checks account permissions (mut, signer, etc.)

---

## Constraint Attributes

### Account Creation

#### `init`

Create a new account (fails if exists):

```rust
#[account(
    init,
    payer = payer,
    space = 8 + MyData::INIT_SPACE,
)]
pub my_account: Account<'info, MyData>,
```

#### `init_if_needed`

Create account or use existing (idempotent):

```rust
#[account(
    init_if_needed,
    payer = payer,
    space = 8 + MyData::INIT_SPACE,
)]
pub my_account: Account<'info, MyData>,
```

**Use case:** Idempotent operations, tests that can be re-run.

### PDA Constraints

#### `seeds` / `bump`

Validate and derive PDAs:

```rust
#[account(
    seeds = [b"config"],
    bump
)]
pub config: Account<'info, Config>,
```

**Multi-seed:**
```rust
#[account(
    seeds = [b"user", user.key().as_ref()],
    bump
)]
pub user_account: Account<'info, UserAccount>,
```

### Ownership

#### `owner`

Set account owner (must match program that will modify it):

```rust
#[account(
    init,
    payer = payer,
    owner = token_program,  // Token Program owns this
)]
pub mint: InterfaceAccount<'info, Mint>,
```

### Mutability

#### `mut`

Account will be modified (must be mutable in transaction):

```rust
#[account(mut)]
pub payer: Signer<'info>,
```

### Signer

#### `signer`

Account must sign the transaction:

```rust
#[account(signer)]
pub authority: Signer<'info>,
```

**Note:** `Signer<'info>` implies `signer` constraint.

### Token-Specific Constraints

#### `mint::decimals`

Set mint decimals (requires `init`):

```rust
#[account(
    init,
    mint::decimals = 9,
)]
pub mint: InterfaceAccount<'info, Mint>,
```

#### `mint::authority`

Set mint authority:

```rust
#[account(
    init,
    mint::authority = mint,  // Mint is its own authority
)]
pub mint: InterfaceAccount<'info, Mint>,
```

#### `mint::freeze_authority`

Set freeze authority (or `None`):

```rust
#[account(
    init,
    mint::freeze_authority = None,
)]
pub mint: InterfaceAccount<'info, Mint>,
```

#### `associated_token::mint` / `associated_token::authority`

Validate Associated Token Account (ATA):

```rust
#[account(
    associated_token::mint = mint,
    associated_token::authority = owner,
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,
```

### Constraint Groups

#### `has_one`

Shorthand for checking account equality:

```rust
#[account(has_one = authority)]
pub config: Account<'info, Config>,

pub authority: Signer<'info>,

// Equivalent to:
// require!(config.authority == authority.key())
```

#### `address`

Check exact address match:

```rust
#[account(address = expected_address)]
pub specific_account: Account<'info, MyData>,
```

### Space Calculation

#### `space`

Set account size in bytes:

```rust
#[account(
    init,
    space = 8 + MyData::INIT_SPACE,
)]
pub my_account: Account<'info, MyData>,
```

**Components:**
- `8` bytes for account discriminator
- `MyData::INIT_SPACE` from `#[derive(InitSpace)]`

### Rent & Payer

#### `payer`

Account paying for rent/fees (must be `mut`):

```rust
#[account(
    init,
    payer = payer,
    space = 100,
)]
pub new_account: Account<'info, MyData>,

#[account(mut)]
pub payer: Signer<'info>,
```

#### `close`

Close account and return lamports to payer:

```rust
#[account(
    mut,
    close = payer,
)]
pub closing_account: Account<'info, MyData>,

#[account(mut)]
pub payer: Signer<'info>,
```

### Special Constraints

#### `constraint`

Custom validation logic:

```rust
#[account(
    constraint = amount < MAX_AMOUNT @ ErrorCode::AmountTooHigh
)]
pub config: Account<'info, Config>,
```

#### `realloc`

Reallocate account size:

```rust
#[account(
    mut,
    realloc = 200,  // New size
    realloc::payer = payer,
    realloc::zero = true,  // Zero new memory
)]
pub growing_account: Account<'info, MyData>,
```

---

## Account Types

### `Account<'info, T>`

Standard deserializable account:

```rust
#[account]
pub struct MyData {
    pub value: u64,
}

pub my_account: Account<'info, MyData>,
```

**Provides:**
- `.data` - direct field access
- `.key()` - account address
- `.reload()` - refresh from chain

### `InterfaceAccount<'info, T>`

For token program accounts (Token & Token-2022):

```rust
pub mint: InterfaceAccount<'info, Mint>,
pub token: InterfaceAccount<'info, TokenAccount>,
```

**Use for:** Token program accounts when using `token_interface`.

### `Signer<'info>`

Account that signs transaction:

```rust
pub authority: Signer<'info>,
```

**Provides:**
- `.key()` - signer's public key

### `Sysvar<'info, T>`

System variables:

```rust
pub clock: Sysvar<'info, Clock>,
pub rent: Sysvar<'info, Rent>,
pub instructions: Sysvar<'info, Instructions>,
```

### `Program<'info, T>`

External program:

```rust
pub token_program: Program<'info, Token>,
pub system_program: Program<'info, System>,
```

### `Interface<'info, T>`

For program interfaces (Token & Token-2022):

```rust
pub token_program: Interface<'info, TokenInterface>,
```

---

## Combining Constraints

### Example: Mint with PDA Authority

```rust
#[account(
    init,
    payer = payer,
    mint::decimals = 9,
    mint::authority = mint,  // Self-authority
    mint::freeze_authority = None,
    seeds = [b"mint"],
    bump
)]
pub mint: InterfaceAccount<'info, Mint>,
```

### Example: Token Account with ATA

```rust
#[account(
    init_if_needed,
    payer = payer,
    associated_token::mint = mint,
    associated_token::authority = authority,
    associated_token::token_program = token_program,
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,
```

---

## See This in Action

**Patterns using these constraints:**
- [Simple Counter](../patterns/initialization/simple-counter.md) - PDA, init, space
- [Create Mint](../patterns/token-operations/create-mint.md) - mint:: constraints
- [Mint Tokens](../patterns/token-operations/mint-tokens.md) - ATA, mut

**Concepts:**
- [Account Model](account-model.md) - Why these constraints exist
- [PDAs Explained](pdas-explained.md) - PDA-specific constraints

**Reference:**
- [Common Errors](../reference/common-errors.md) - Constraint error messages
