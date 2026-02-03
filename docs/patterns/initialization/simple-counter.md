## Pattern: Simple Counter

**Concept:** Program Derived Addresses (PDAs) and state accounts
**Related Code:** `programs/anchortokenstarter/src/lib.rs: increment instruction, Increment accounts struct, Counter account`
**Test:** `tests/anchortokenstarter.ts: "Creates and increments a counter"`
**Prerequisites:** [Hello World](hello-world.md)

---

## What You're Building

A counter that:
- Stores a `count` value in its own account
- Uses a PDA (Program Derived Address) for deterministic addressing
- Increments the count each time the instruction is called

This pattern teaches:
- Creating state accounts with `#[account]`
- PDAs for deterministic addresses
- Account initialization with `init_if_needed`
- The `#[account(init)]` constraint attributes

---

## The Concept

### Program Derived Addresses (PDAs)

A **PDA** is a deterministic address derived from:
1. Your program ID
2. Seed values (bytes you choose)

**Formula:** `PDA = findProgramAddress(seeds, program_id)`

**Key insight:** Same program + same seeds = same address, always.

```rust
// Derivation
let [pda, bump] = PublicKey.findProgramAddress(
    &[b"counter"],  // seed
    program_id       // your program
);
```

**Why PDAs matter:**
- Programs can "sign" for their PDAs (using the bump seed)
- Enables program-owned state without private keys
- Critical for security and composability

### State Accounts in Anchor

In Solidity, state lives in contract storage:
```solidity
contract Counter {
    uint256 public count = 0;
}
```

In Solana, state lives in separate accounts:
```rust
#[account]
pub struct Counter {
    pub count: u64,
}
```

**Key difference:** The account is separate from the program, has its own address, and must be rent-exempt (hold minimum SOL).

### Account Initialization Constraints

```rust
#[account(
    init_if_needed,           // Create if doesn't exist
    payer = payer,            // Who pays rent
    space = 8 + Counter::INIT_SPACE,  // Account size
    seeds = [b"counter"],     // PDA seeds
    bump                      // Store bump in account
)]
pub counter: Account<'info, Counter>,
```

**Attributes explained:**
- `init_if_needed` - Create account if it doesn't exist (idempotent)
- `payer` - Account that pays the rent exemption (must be `mut`)
- `space` - Account size: discriminator (8) + struct size
- `seeds` - PDA derivation seeds
- `bump` - Canonical bump used for PDA signing

---

## The Code

```rust
// programs/anchortokenstarter/src/lib.rs

use anchor_lang::prelude::*;

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Simple counter increment
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous count: {}", counter.count);
        counter.count += 1;
        msg!("Current count: {}", counter.count);
        Ok(())
    }
}

/// Accounts for increment instruction
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

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Counter state account
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
}
```

**Annotations explained:**
- `&mut` - Mutable reference to counter (allows modification)
- `ctx.accounts.counter` - Access validated account from context
- `#[account]` - Marks struct as deserializable account
- `#[derive(InitSpace)]` - Automatically calculates `INIT_SPACE` constant

---

## Running It

```bash
# Run the counter test
anchor test --skip-build -- --grep "Creates and increments a counter"
```

**Expected output:**
```
  AnchorTokenStarter
    Basic Tests
      ✔ Creates and increments a counter (858ms)
```

**Program logs:**
```
Program log:Previous count: 0
Program log:Current count: 1
Program log:Previous count: 1
Program log:Current count: 2
```

---

## If It Fails

### "Account already exists"
**Cause:** Counter was created in previous run, using `init` instead of `init_if_needed`

**Solution:** Use `init_if_needed` for idempotent initialization, or reset local validator:
```bash
solana-test-validator --reset --ledger /tmp/test-ledger
```

### "Account not rent exempt"
**Cause:** Insufficient funds for rent exemption

**Solution:** Airdrop SOL to payer account
```bash
solana airdrop 2 <payer_address>
```

### "Seeds constraint violation"
**Cause:** Test passing wrong PDA address

**Solution:** Derive PDA in test the same way as in program:
```typescript
const [counterPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],
    program.programId
);
```

### See also
[Common Errors](../../reference/common-errors.md)

---

## Solidity Sidebar

**State storage:**
```solidity
// Solidity - State inside contract
contract Counter {
    uint256 public count = 0;
    function increment() external { count++; }
}
```

```rust
// Anchor - State in separate account
#[account]
pub struct Counter { pub count: u64 }

#[account(seeds = [b"counter"], bump)]
pub counter: Account<'info, Counter>,
```

**Key difference:** Solana separates code (program) from data (accounts). The Counter account is a separate address from the program.

---

## You've Learned

- ✅ How to create state accounts with `#[account]`
- ✅ PDAs for deterministic addressing
- ✅ Account initialization with `init_if_needed`
- ✅ Account size calculation (discriminator + struct)
- ✅ The `#[account(init)]` constraint attributes

---

## Next Steps

**Continue learning:** [Pattern: Create Mint](../token-operations/create-mint.md)
- Create an SPL Token mint (compare to ERC20 deployment)
- Learn about token program accounts
- Understand mint authority and PDA signing

**Deep dive:** [Concept: PDAs Explained](../../concepts/pdas-explained.md)
- Full explanation of PDA derivation
- How bump seeds work
- PDA signing for program authority

**Deep dive:** [Concept: Account Model](../../concepts/account-model.md)
- Rent exemption explained
- Account ownership
- Program vs data accounts
