# Rust Survival for Solidity Developers

**Essential Rust concepts for Solana development**

---

## Overview

You know Solidity. Here's what you need to know about Rust for Solana smart contract development.

---

## Ownership in Solana Context

### Rust Ownership (Quick Refresher)

```rust
// Ownership = "who is responsible for cleanup"
let owner = Account::new();  // owner owns the account
let borrower = &owner;        // borrower borrows (read-only)
// owner.drop();              // ERROR: owner still owns it
```

### In Solana Programs

**Good news:** You rarely think about ownership in Solana!

```rust
// Accounts are passed by Anchor, not created by you
pub fn increment(ctx: Context<Increment>) -> Result<()> {
    let counter = &mut ctx.accounts.counter;  // Borrow from context
    counter.count += 1;                       // Modify
    Ok(())                                     // Anchor handles cleanup
}
```

**Key point:** `ctx.accounts` owns everything, you just borrow.

---

## Borrowing: `&` vs `&mut`

### Immutable Borrow (`&`)

```rust
let counter = &ctx.accounts.counter;  // Read only
msg!("Count: {}", counter.count);
// counter.count += 1;  // ERROR: can't modify
```

### Mutable Borrow (`&mut`)

```rust
let counter = &mut ctx.accounts.counter;  // Read and write
counter.count += 1;                        // Can modify
```

### In Account Structs

```rust
#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(mut)]  // ← Tells Anchor: will modify
    pub counter: Account<'info, Counter>,
}
```

**Solidity comparison:**
- Solidity: No concept, everything is mutable by default
- Rust: Explicit `mut` makes intentions clear

---

## Result Types and Error Handling

### `Result<T, E>`

Rust's way of handling success/failure:

```rust
pub fn my_function() -> Result<()> {
    // Do something
    Ok(())  // Success (empty tuple)
}

pub fn my_function_with_value() -> Result<u64> {
    // Do something
    Ok(42)  // Success with value
}
```

### Errors with `?`

The `?` operator propagates errors:

```rust
pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    token_interface::transfer_checked(cpi_context, amount, decimals)?;  // ? propagates errors
    Ok(())
}
```

**Equivalent to:**
```solidity
function transfer(uint256 amount) external {
    try tokenProgram.transfer(amount) {
        revert("Transfer failed");
    }
}
```

### Custom Errors

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds")]
    InsufficientFunds,

    #[msg("Unauthorized")]
    Unauthorized,
}

// Using errors
require!(amount <= balance, ErrorCode::InsufficientFunds);
```

---

## Macros

### `msg!` for Logging

```rust
msg!("Hello, Solana!");
msg!("Count is now: {}", count);
```

**Solidity equivalent:**
```solidity
emit Log("Count is now");
// or
console.log("Count is now");
```

### `require!` for Validation

```rust
require!(amount > 0, ErrorCode::InvalidAmount);
```

**Solidity equivalent:**
```solidity
require(amount > 0, "Invalid amount");
```

### Derive Macros

```rust
#[account]              // Serialize/deserialize
#[derive(InitSpace)]     // Calculate size
#[derive(Accounts)]      // Validate accounts
pub struct MyData {
    pub value: u64,
}
```

---

## Types

### Primitives

| Solidity | Rust | Solana Usage |
|----------|------|--------------|
| `uint256` | `u64` | Balances, amounts |
| `uint8` | `u8` | Decimals |
| `bool` | `bool` | Flags |
| `address` | `Pubkey` | Account addresses |
| `bytes` | `Vec<u8>` | Arbitrary data |
| `bytes32` | `[u8; 32]` | Fixed-size byte arrays |
| `string` | `String` | Text (rare on-chain) |

### Common Solana Types

```rust
Pubkey              // Account address
u64                 // Balances, amounts
i64                 // Signed integers (timestamps)
u8                  // Decimals, small values
bool                // Flags
Vec<u8>             // Arbitrary data
```

---

## Pattern Matching

### Basic Match

```rust
match my_option {
    Some(value) => msg!("Value: {}", value),
    None => msg!("No value"),
}
```

### In Solana Programs

You'll see this less often, but useful for:
- Processing optional values
- Handling different account states

---

## Lifetimes (The Basics)

**Good news:** Anchor handles most lifetime stuff!

**What you'll see:**
```rust
pub struct MyAccounts<'info> {
    pub counter: Account<'info, Counter>,
}
```

**What it means:** `'info` is a lifetime that says "these accounts are valid for this instruction."

**You don't need to:**
- Understand lifetime elision
- Write complex lifetime annotations
- Worry about lifetime errors (usually)

Anchor's macros handle the heavy lifting.

---

## Common Patterns

### Context Parameter

Every instruction takes a Context:

```rust
pub fn my_instruction(ctx: Context<MyAccounts>) -> Result<()> {
    // Access accounts via ctx.accounts
    let account = &ctx.accounts.my_account;
    Ok(())
}
```

### Account Structs

Define what accounts are required:

```rust
#[derive(Accounts)]
pub struct MyAccounts<'info> {
    #[account(mut)]
    pub my_account: Account<'info, MyData>,
}
```

### CPI Context

For calling other programs:

```rust
let cpi_context = CpiContext::new(program, accounts)
    .with_signer(seeds);

other_program::instruction(cpi_context, params)?;
```

---

## Further Reading

**Learn more Rust:**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

**Solana-specific:**
- [Anchor Documentation](https://www.anchor-lang.com/docs)
- [Solana Cookbook](https://solanacookbook.com/)

**Practice:**
- [Pattern: Simple Counter](../patterns/initialization/simple-counter.md) - Basic Rust in action
- [Pattern: Mint Tokens](../patterns/token-operations/mint-tokens.md) - Advanced patterns
- [Pattern: Transfer Tokens](../patterns/token-operations/transfer-tokens.md) - CPI and borrowing
