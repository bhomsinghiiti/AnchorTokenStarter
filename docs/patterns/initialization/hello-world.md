## Pattern: Hello World

**Concept:** Basic Solana program structure and logging
**Related Code:** `programs/anchortokenstarter/src/lib.rs: initialize instruction, Initialize accounts struct`
**Test:** `tests/anchortokenstarter.ts: "Initializes the program"`
**Prerequisites:** [Quick Start](../../00-quick-start.md)

---

## What You're Building

The simplest possible Anchor program - one that logs messages when called. No state, no accounts, just pure function execution.

This pattern teaches:
- Program structure with `#[program]`
- Instruction handler functions
- Using `msg!` for logging
- Empty accounts structs

---

## The Concept

### Anchor Program Structure

Every Anchor program starts with the same structure:

```rust
use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID_HERE");

#[program]
pub mod my_program {
    use super::*;

    pub fn my_instruction(ctx: Context<MyAccounts>) -> Result<()> {
        // Your logic here
        msg!("Hello, Solana!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MyAccounts {}
```

**Key components:**
- `declare_id!` - Your program's on-chain address (baked into the binary)
- `#[program]` - Marks the module containing instruction handlers
- `pub fn` - Each public function is a callable instruction
- `Context<T>` - Wrapper containing accounts and program info
- `Result<()>` - Return type (Ok or Error)
- `#[derive(Accounts)]` - Validates and deserializes accounts

### Logging with `msg!`

The `msg!` macro logs to the program output (visible in tests and explorers):

```rust
msg!("Greetings from: {:?}", ctx.program_id);
msg!("Hello, Solana!");
```

**Difference from Solidity:**
- Solidity `emit Event()` - indexed events for frontend
- Solana `msg!()` - program logs (visible but not indexed)

### Empty Accounts Struct

The `Initialize` struct is empty because hello world needs no accounts:

```rust
#[derive(Accounts)]
pub struct Initialize {}
```

**Why this works:**
- No state to read/write
- No signer to validate
- No other programs to call
- Just pure function execution

---

## The Code

```rust
// programs/anchortokenstarter/src/lib.rs

use anchor_lang::prelude::*;

declare_id!("AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa");

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Initialize - Basic hello world
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        msg!("Hello, Solana!");
        Ok(())
    }
}

/// Initialize accounts (empty for hello world)
#[derive(Accounts)]
pub struct Initialize {}
```

**Annotations explained:**
- `///` - Rust doc comment (shows in IDE docs)
- `pub fn` - Public function (callable as instruction)
- `Context<Initialize>` - Wrapper containing validated accounts
- `Result<()>` - Returns empty result on success

---

## Running It

```bash
# Run just the hello world test
anchor test --skip-build -- --grep "Initializes the program"
```

**Expected output:**
```
  AnchorTokenStarter
    Basic Tests
Initialize transaction signature 4waPXJpYqVXXfHRhfC8ybye9immPgk99TYVKPdCLE4YtSV8Stt2Lf7Cv4onLPYCbPGXPHZtceNK3z3Z4ztohK618
      ✔ Initializes the program (121ms)
```

**Program logs** (visible in test output):
```
Program log:Greetings from: AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa
Program log:Hello, Solana!
```

---

## If It Fails

### "Program not found"
**Cause:** Program hasn't been deployed or program ID mismatch

**Solution:**
```bash
anchor build
anchor test
```

### "Account invalid" or similar
**Cause:** Accounts struct has accounts but test doesn't provide them

**Solution:** Ensure test `.accounts()` call matches your `#[derive(Accounts)]` struct

### See also
[Common Errors](../../reference/common-errors.md)

---

## You've Learned

- ✅ Basic Anchor program structure
- ✅ How `#[program]` and `declare_id!` work
- ✅ Instruction handler functions return `Result<()>`
- ✅ Using `msg!` for program logging
- ✅ Empty accounts structs for stateless instructions

---

## Next Steps

**Continue learning:** [Pattern: Simple Counter](simple-counter.md)
- Adds state with a Counter account
- Introduces PDAs (Program Derived Addresses)
- Shows account initialization with `#[account(init)]`

**Deep dive:** [Concept: Account Model](../../concepts/account-model.md)
- Understand Solana's separation of code and data
- Learn about rent exemption
- Compare with Solidity's contract storage

**Deep dive:** [Concept: Anchor Macros](../../concepts/anchor-macros.md)
- Full reference of `#[account]` attributes
- Constraint types and their meanings
