## Pattern: Create Mint

**Concept:** SPL Token mint creation (compare to ERC20 deployment)
**Related Code:** `programs/anchortokenstarter/src/lib.rs: create_mint instruction, CreateMint accounts struct`
**Test:** `tests/anchortokenstarter.ts: "Creates a new token mint"`
**Prerequisites:** [Simple Counter](../initialization/simple-counter.md)

---

## What You're Building

An SPL Token mint that:
- Has 9 decimal places (standard for tokens)
- Uses a PDA as mint authority (program can mint)
- Has no freeze authority (no one can freeze accounts)
- Is created at a deterministic address using seeds

This pattern teaches:
- SPL Token mint account structure
- Using `mint::` constraint attributes
- Mint authority and freeze authority concepts
- How this differs from ERC20 deployment

---

## The Concept

### SPL Token vs ERC20

**Ethereum/ERC20:**
```solidity
// ERC20 deployment - code + data combined
contract MyToken is ERC20 {
    constructor() ERC20("My Token", "MTK") {
        _mint(msg.sender, 1000000 * 10**18);
    }
}
```

**Solana/SPL Token:**
```rust
// SPL Token - separate mint account
#[account(
    init,
    payer = payer,
    mint::decimals = 9,
    mint::authority = mint,  // Mint itself (PDA) can mint
    mint::freeze_authority = mint,  // Same for freeze (or None)
    seeds = [b"mint"],
    bump
)]
pub mint: InterfaceAccount<'info, Mint>,
```

**Key difference:**
- ERC20: Contract IS the token (code + deployment = new token)
- SPL Token: Mint account is the token (separate from your program)

### Mint Authorities

**Mint Authority:** Who can create new tokens
- Can be a user wallet (`Pubkey`)
- Can be the mint itself (PDA) - program controls minting
- Can be `None` - fixed supply, no more can be minted

**Freeze Authority:** Who can freeze token accounts
- Can be a user wallet
- Can be the mint itself (PDA)
- Can be `None` - no one can freeze (recommended)

### Using `InterfaceAccount<'info, Mint>`

The `InterfaceAccount` type supports:
- Original Token Program
- Token-2022 Program

This future-proofs your program for Token-2022 features.

---

## The Code

```rust
// programs/anchortokenstarter/src/lib.rs

use anchor_spl::token_interface::{Mint, TokenInterface};

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Create a new token mint
    pub fn create_mint(_ctx: Context<CreateMint>, decimals: u8) -> Result<()> {
        // Validate decimals (this implementation uses 9)
        require!(decimals == 9, ErrorCode::InvalidDecimals);
        msg!("Created token mint with {} decimals", decimals);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        payer = payer,
        mint::decimals = 9,           // Standard for tokens
        mint::authority = mint,       // Mint itself (PDA) is authority
        mint::freeze_authority = mint, // Same for freeze (or None)
        seeds = [b"mint"],            // PDA derivation
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Decimals must be 9 for this mint implementation")]
    InvalidDecimals,
}
```

**Constraint attributes:**
- `mint::decimals = 9` - Fixed decimal places (standard)
- `mint::authority = mint` - Mint can sign for itself via PDA
- `mint::freeze_authority = mint` - Same for freeze (use `None` to disable)

---

## Running It

```bash
# Run the create mint test
anchor test --skip-build -- --grep "Creates a new token mint"
```

**Expected output:**
```
  AnchorTokenStarter
    SPL Token Tests
      ✔ Creates a new token mint (417ms)
```

**Mint address (PDA):**
```
Seed: ["mint"]
Program: AT3LVeEomGSHCdD1vASqsnFvwm8Y4KmY9Z9BLbEt3jEa
```

---

## If It Fails

### "Invalid decimals"
**Cause:** Passing decimals != 9

**Solution:** This implementation requires 9 decimals (standard for tokens). Other common values: 6 (USDC), 8 (BTC-like).

### "Mint already exists"
**Cause:** Mint account was created in previous run

**Solution:** Use `init` (not `init_if_needed`) for one-time creation, or reset local validator:
```bash
solana-test-validator --reset --ledger /tmp/test-ledger
```

### "Account owned by wrong program"
**Cause:** Using wrong token program version

**Solution:** Ensure `token_program` matches your Anchor version:
```typescript
tokenProgram: TOKEN_PROGRAM_ID  // Original
// or
tokenProgram: TOKEN_2022_PROGRAM_ID  // Token-2022
```

### See also
[Common Errors](../../reference/common-errors.md)

---

## Solidity Sidebar

**ERC20 deployment:**
```solidity
// One transaction = contract + token
MyToken token = new MyToken();
// token.address() IS the token
```

**SPL Token creation:**
```rust
// Two concepts: your program + mint account
// Mint address is the token
// Your program controls the mint
```

**Key difference:**
- ERC20: Deploying creates the token
- SPL Token: Creating mint account creates the token
- Your program is separate (and can control multiple mints)

---

## You've Learned

- ✅ SPL Token mint account structure
- ✅ `mint::` constraint attributes (`decimals`, `authority`, `freeze_authority`)
- ✅ Using `InterfaceAccount` for token program compatibility
- ✅ Mint authority and freeze authority concepts
- ✅ How SPL Token creation differs from ERC20 deployment

---

## Next Steps

**Continue learning:** [Pattern: Mint Tokens](mint-tokens.md)
- Mint tokens to user accounts
- Use CPI to call SPL Token program
- PDA signing for mint authority

**Deep dive:** [Concept: Account Model](../../concepts/account-model.md)
- Understand why tokens are accounts
- Learn about rent exemption for mints

**Reference:** [Solidity Comparison](../../reference/solidity-comparison.md)
- Full ERC20 vs SPL Token comparison table
