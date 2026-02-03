# Common Errors

**Troubleshooting guide for common Solana/Anchor development errors**

---

## Account Errors

### `AccountAlreadyInitialized`

**Error:** Account already exists

**Cause:** Using `init` on an account that was created previously

**Solution:**
```rust
// Use init_if_needed for idempotent creation
#[account(init_if_needed, ...)]
pub account: Account<'info, MyData>,
```

Or reset local validator:
```bash
solana-test-validator --reset --ledger /tmp/test-ledger
```

---

### `AccountNotInitialized`

**Error:** Account not initialized

**Cause:** Trying to use an account before creation

**Solution:**
```bash
# Ensure account is created first
# Check if test creates account before using it
```

Or in code:
```rust
// Ensure init constraint is present
#[account(init, ...)]
```

---

### `AccountOwnedByWrongProgram`

**Error:** Account owned by different program

**Cause:** Account owner doesn't match expected program

**Example:** Creating a mint but using `init` instead of `mint::` constraints

**Solution:**
```rust
// For mint accounts, use mint:: constraints (not just init)
#[account(
    init,
    mint::decimals = 9,
    mint::authority = mint,
)]
```

---

### `AccountInvalid` / `ConstraintRaw`

**Error:** Account constraint violated

**Cause:** Account doesn't match constraint

**Common issues:**
- Wrong PDA seeds
- Account not mut when needed
- Wrong owner

**Solution:** Check your constraint matches test data
```typescript
// Test must use same seeds as program
const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],  // Must match seeds
    program.programId
);
```

---

## Authority Errors

### `MissingSigner`

**Error:** Account not a signer

**Cause:** Account needs to sign but didn't

**Solution:**
```typescript
// Add signer to transaction
await program.methods
    .myInstruction()
    .accounts({
        authority: signer.publicKey,  // Add to accounts
    })
    .signers([signer])  // Add to signers
    .rpc();
```

---

### `MintToInvalidAuthority` / `TransferCheckedFailed`

**Error:** Invalid mint authority

**Cause:** Mint authority check failed

**Solution:**
```rust
// Ensure mint PDA seeds match
let seeds: &[&[&[u8]]] = &[&[b"mint"], &[ctx.bumps.mint]];

// And create_mint uses same seeds
seeds = [b"mint"]
```

---

### `OwnerMismatch` / `HasOneConstraintViolation`

**Error:** Owner mismatch

**Cause:** Account owner doesn't match expected

**Solution:**
```rust
// Use has_one for equality check
#[account(has_one = authority)]
pub config: Account<'info, Config>,

// Or manual constraint
#[account(constraint = config.authority == authority.key())]
pub config: Account<'info, Config>,
```

---

## PDA Errors

### `SeedsConstraintViolation`

**Error:** Seeds constraint violated

**Cause:** PDA derivation mismatch

**Solution:**
```typescript
// Test must use same seeds as program
const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("counter")],  // Must match b"counter"
    program.programId
);
```

---

### `TryFindProgramAddressFailed`

**Error:** Can't find PDA

**Cause:** Invalid seeds or program ID

**Solution:**
```typescript
// Ensure correct seeds and program ID
const seeds = [Buffer.from("mint")];  // Must match program
const programId = program.programId;   // Correct program

const [pda] = PublicKey.findProgramAddressSync(seeds, programId);
```

---

## Token Errors

### `AssociatedTokenAccountNotFound`

**Error:** ATA not found

**Cause:** Token account doesn't exist

**Solution:**
```rust
// Use init_if_needed to create ATA
#[account(
    init_if_needed,
    associated_token::mint = mint,
    associated_token::authority = owner,
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,
```

Or in test, create ATA first:
```typescript
// Create ATA before using
await program.methods
    .mintTokens(new BN(0))  // Mint 0 to create ATA
    .accounts({ ... })
    .rpc();
```

---

### `IncorrectAuthority`

**Error:** Incorrect authority

**Cause:** Wrong signer for operation

**Solution:**
```typescript
// Ensure authority account is signer
await program.methods
    .transfer(amount)
    .accounts({
        authority: signer.publicKey,  // Must be signer
        // ...
    })
    .signers([signer])  // Must sign
    .rpc();
```

---

## CPI Errors

### `InvalidProgramId`

**Error:** Invalid program ID

**Cause:** Wrong program in CPI

**Solution:**
```rust
// Ensure correct program in CPI
let cpi_program = ctx.accounts.token_program.to_account_info();
token_interface::mint_to(cpi_context, amount)?;
```

---

### `CpiContext/WrongAccountSize`

**Error:** Wrong account size for CPI

**Cause:** Missing required account in CPI

**Solution:**
```rust
// Ensure all required accounts are included
let cpi_accounts = MintTo {
    mint: ctx.accounts.mint.to_account_info(),
    to: ctx.accounts.token_account.to_account_info(),
    authority: ctx.accounts.mint.to_account_info(),
    // Missing accounts cause this error
};
```

---

## Rent Errors

### `AccountNotRentExempt`

**Error:** Account not rent exempt

**Cause:** Insufficient funds for rent

**Solution:**
```bash
# Airdrop SOL to payer
solana airdrop 2 <payer_address>
```

Or ensure sufficient balance before test:
```typescript
// Ensure payer has SOL
await connection.requestAirdrop(payer, 2 * LAMPORTS_PER_SOL);
```

---

## Build Errors

### `cannot find value ... in this scope`

**Error:** Variable not found

**Cause:** Typo or import missing

**Solution:**
```rust
// Check imports
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, MintTo};

// Check variable names
let counter = &mut ctx.accounts.counter;  // Correct name
```

---

### `type ... not provided`

**Error:** Type inference failed

**Cause:** Missing type annotation

**Solution:**
```rust
// Add explicit type
let amount: u64 = 1_000_000_000;

// Or use turbofish
let cpi_context: CpiContext<'_, '_, '_, '_, MintTo<'_>> = CpiContext::new(...);
```

---

## Test Errors

### `Timeout awaiting transaction confirmation`

**Error:** Transaction timeout

**Cause:** Test taking too long or transaction failing

**Solution:**
```typescript
// Increase timeout
await program.methods
    .myInstruction()
    .rpc({ commitment: 'confirmed' });  // Wait for confirmation
```

---

### `Blockhash expired`

**Error:** Blockhash expired

**Cause:** Test running too long

**Solution:**
```typescript
// Re-fetch blockhash
const { blockhash } = await connection.getLatestBlockhash();
// Or use shorter-running tests
```

---

## Running Tests

### `anchor test` hangs

**Cause:** Validator not starting or test waiting

**Solution:**
```bash
# Kill existing validators
pkill solana-test-validator

# Or use skip-local with manually started validator
solana-test-validator --ledger /tmp/test-ledger
anchor test --skip-local
```

---

### `anchor build` fails

**Cause:** Compilation error

**Solution:**
```bash
# Check Rust version
rustc --version  # Should be 1.93.0

# Check for errors in output
anchor build 2>&1 | grep error

# Fix errors and rebuild
```

---

## Getting Help

### Debug Mode

Enable debug output:
```typescript
// In test
const tx = await program.methods
    .myInstruction()
    .accounts({ ... })
    .rpc({ skipPreflight: false });  // Get more info
```

### Program Logs

View program logs:
```bash
# In test
console.log("Logs:", tx);

# Or use explorer
solana confirm -v <tx_signature>
```

### Explorer

Use Solana explorer to inspect transactions:
```
https://explorer.solana.com/tx/<signature>?cluster=devnet
```

---

## Prevention

### Common Practices

1. **Use `init_if_needed`** for idempotent account creation
2. **Validate in tests** that accounts exist before use
3. **Use descriptive errors** with `#[error_code]`
4. **Log extensively** with `msg!` for debugging
5. **Test locally** before deploying to devnet/mainnet

### Development Tips

1. **Run tests frequently** - Catch errors early
2. **Use git** - Revert breaking changes quickly
3. **Keep tests simple** - One thing per test
4. **Check types** - Ensure TypeScript types match Rust types
5. **Read error messages** - They usually tell you what's wrong

---

## Further Reading

**Concepts:**
- [Account Model](../concepts/account-model.md) - Understanding account errors
- [Anchor Macros](../concepts/anchor-macros.md) - Constraint errors
- [PDAs Explained](../concepts/pdas-explained.md) - PDA errors

**Patterns:**
- [Quick Start](../00-quick-start.md) - Troubleshooting section
