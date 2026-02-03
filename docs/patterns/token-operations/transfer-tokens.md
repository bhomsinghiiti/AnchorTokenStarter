## Pattern: Transfer Tokens

**Concept:** Multi-account validation and authority-checked transfers
**Related Code:** `programs/anchortokenstarter/src/lib.rs: transfer_tokens instruction, TransferTokens accounts struct`
**Test:** `tests/anchortokenstarter.ts: "Transfers tokens between accounts"`
**Prerequisites:** [Mint Tokens](mint-tokens.md)

---

## What You're Building

An instruction that:
- Transfers tokens from sender to recipient
- Validates both ATAs exist
- Checks sender authority (signer owns sender ATA)
- Uses `transfer_checked` to validate mint decimals

This pattern teaches:
- Multi-account validation
- Authority checks with ATA ownership
- Using `transfer_checked` vs `transfer`
- Handling recipient accounts

---

## The Concept

### Token Transfer Components

A transfer requires:
1. **Mint account** - To verify decimals and get metadata
2. **Sender ATA** - Source token account
3. **Recipient ATA** - Destination token account
4. **Authority** - Signer who owns sender ATA

**Account flow:**
```
Mint ──┬── Sender ATA ──┬── Authority (signer)
       │                └── tokens transferred here
       └── Recipient ATA ─── tokens arrive here
```

### transfer_checked vs transfer

**`transfer_checked`:** Requires mint account, validates decimals
```rust
token_interface::transfer_checked(cpi_context, amount, decimals)?;
```

**`transfer` (legacy):** Doesn't validate decimals (deprecated)

**Why use `transfer_checked`:**
- Ensures amount doesn't exceed decimal precision
- Catches precision mismatches early
- Recommended for all new code

### ATA Ownership Validation

Anchor's `associated_token::authority` constraint verifies ownership:

```rust
#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,  // signer owns this ATA
)]
pub sender_token_account: InterfaceAccount<'info, TokenAccount>,
```

**What this validates:**
- Account is an ATA for `mint`
- Owner is `signer` (transaction signer)
- Prevents transferring from someone else's account

---

## The Code

```rust
// programs/anchortokenstarter/src/lib.rs

use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked};

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Transfer tokens between accounts
    pub fn transfer_tokens(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        let decimals = ctx.accounts.mint.decimals;

        // CPI accounts for transfer
        let cpi_accounts = TransferChecked {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.sender_token_account.to_account_info(),
            to: ctx.accounts.recipient_token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        // Build CPI context
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        // Execute transfer
        token_interface::transfer_checked(cpi_context, amount, decimals)?;
        msg!("Transferred {} tokens", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,  // signer owns sender ATA
    )]
    pub sender_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: recipient is only used as a token authority
    pub recipient: UncheckedAccount<'info>,

    pub token_program: Interface<'info, TokenInterface>,
}
```

**Key annotations:**
- `unchecked_account` for recipient - only used for ATA derivation, no signing
- `mut` on token accounts - balances change
- `associated_token::authority` - validates ownership

---

## Running It

```bash
# Run the transfer tokens test
anchor test --skip-build -- --grep "Transfers tokens between accounts"
```

**Expected output:**
```
  AnchorTokenStarter
    SPL Token Tests
      ✔ Transfers tokens between accounts (1268ms)
```

**Program logs:**
```
Program log:Transferred 100000000 tokens
```

---

## If It Fails

### "OwnerMismatch"
**Cause:** Signer doesn't own sender ATA

**Solution:** Ensure signer is the owner of the sender token account. Check ATA derivation:
```typescript
const senderATA = await getAssociatedTokenAddress(mint, signer);
```

### "Account not found"
**Cause:** Recipient ATA doesn't exist

**Solution:** Create recipient ATA first (often done in a separate step or using CPI):
```typescript
// Create ATA before transfer
await program.methods
    .mintTokens(new BN(0))  // Mint 0 to create ATA
    .accounts({ ... })
    .rpc();
```

### "Insufficient funds"
**Cause:** Sender ATA balance < transfer amount

**Solution:** Check balance before transfer or handle in client code

### See also
[Common Errors](../../reference/common-errors.md)

---

## Solidity Sidebar

**ERC20 transfer:**
```solidity
// Inside token contract
function transfer(address to, uint256 amount) external returns (bool) {
    _transfer(msg.sender, to, amount);
    return true;
}
```

**SPL Token transfer:**
```rust
// Your program calls Token Program
token_interface::transfer_checked(cpi_context, amount, decimals)?;
```

**Key differences:**
- ERC20: `msg.sender` is implicit sender
- SPL Token: All accounts explicit (sender ATA, recipient ATA, mint)
- ERC20: Function validates ownership
- SPL Token: Account constraints validate ownership

---

## You've Learned

- ✅ Multi-account validation patterns
- ✅ ATA ownership validation with `associated_token::authority`
- ✅ Using `transfer_checked` for safe transfers
- ✅ Using `unchecked_account` for non-signing authorities
- ✅ The recipient account pattern

---

## Next Steps

**Continue learning:** [Pattern: Burn Tokens](burn-tokens.md)
- Implement token burning
- Reduce circulating supply
- Exercise: Complete the implementation

**Deep dive:** [Concept: Anchor Macros](../../concepts/anchor-macros.md)
- Full reference of `#[account]` attributes
- Constraint combinations and patterns

**Pattern:** [Signer Validation](../security/signer-validation.md)
- Advanced authority patterns
- Multi-sig approvals
- Role-based access control
