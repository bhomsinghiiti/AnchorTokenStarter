## Pattern: Burn Tokens

**Concept:** Reducing token supply by burning tokens
**Prerequisites:** [Transfer Tokens](transfer-tokens.md)
**Status:** 📝 Exercise

---

## Exercise: Implement Token Burning

Implement a `burn_tokens` instruction that allows token holders to burn (destroy) their tokens.

### Requirements

Create a `burn_tokens` instruction that:
- Burns tokens from the signer's token account
- Validates the signer owns the token account
- Reduces the total supply tracked by the mint
- Uses CPI to call the SPL Token program's `burn` instruction

### Hints

**CPI Accounts for Burn:**
```rust
use anchor_spl::token_interface::{Burn, TokenAccount, TokenInterface};

let cpi_accounts = Burn {
    mint: ctx.accounts.mint.to_account_info(),
    from: ctx.accounts.token_account.to_account_info(),
    authority: ctx.accounts.signer.to_account_info(),
};
```

**CPI Call:**
```rust
token_interface::burn(cpi_context, amount)?;
```

**Account validation:**
```rust
#[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = signer,
)]
pub token_account: InterfaceAccount<'info, TokenAccount>,
```

### Expected Tests

```typescript
describe("Burn Tokens", () => {
  const [mintPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  );

  const signerATA = await getAssociatedTokenAddress(
    mintPda,
    provider.wallet.publicKey
  );

  it("Burns tokens from signer's account", async () => {
    // First, get initial balance
    const beforeBalance = (await getAccount(
      provider.connection,
      signerATA
    )).amount;

    const burnAmount = new anchor.BN(100_000_000); // 0.1 tokens

    await program.methods
      .burnTokens(burnAmount)
      .accounts({
        signer: provider.wallet.publicKey,
        mint: mintPda,
        tokenAccount: signerATA,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Verify balance decreased
    const afterBalance = (await getAccount(
      provider.connection,
      signerATA
    )).amount;

    expect(Number(afterBalance)).to.equal(
      Number(beforeBalance) - burnAmount.toNumber()
    );
  });

  it("Fails when burning more than balance", async () => {
    const hugeAmount = new anchor.BN(1_000_000_000_000); // Way more than exists

    await expect(program.methods
      .burnTokens(hugeAmount)
      .accounts({
        signer: provider.wallet.publicKey,
        mint: mintPda,
        tokenAccount: signerATA,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc()
    ).to.be.rejected;
  });

  it("Fails when non-owner tries to burn", async () => {
    const nonOwner = Keypair.generate();
    const nonOwnerATA = await getAssociatedTokenAddress(
      mintPda,
      nonOwner.publicKey
    );

    // Create and fund non-owner ATA first
    await program.methods
      .mintTokens(new anchor.BN(100_000_000))
      .accounts({
        signer: nonOwner.publicKey,
        mint: mintPda,
        tokenAccount: nonOwnerATA,
        // ... other accounts
      })
      .signers([nonOwner])
      .rpc();

    // Try to burn from non-owner's ATA (signed by non-owner)
    await expect(program.methods
      .burnTokens(new anchor.BN(50_000_000))
      .accounts({
        signer: nonOwner.publicKey,  // Actual owner
        tokenAccount: signerATA,     // WRONG: someone else's ATA
        // ...
      })
      .rpc()
    ).to.be.rejected;  // Should fail - ATA doesn't match signer
  });
});
```

### Solution

<details>
<summary>Click to reveal solution</summary>

```rust
// programs/anchortokenstarter/src/lib.rs

use anchor_spl::token_interface::{Burn, Mint, TokenAccount, TokenInterface};

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Burn tokens from signer's account
    pub fn burn_tokens(ctx: Context<BurnTokens>, amount: u64) -> Result<()> {
        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.signer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        token_interface::burn(cpi_context, amount)?;
        msg!("Burned {} tokens", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct BurnTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}
```

</details>

---

## What You'll Learn

- ✅ How burning reduces token supply
- ✅ Burn CPI structure
- ✅ Account ownership validation
- ✅ Similarities with mint_tokens (both use CPI to token program)

---

## Common Patterns

### Burn-and-Mint (Rebase)

Some protocols burn and re-mint for supply adjustments:
```rust
// Burn old tokens
token_interface::burn(burn_context, old_amount)?;

// Mint new tokens at different ratio
token_interface::mint_to(mint_context, new_amount)?;
```

### Fee-on-Transfer

Burn a portion as fee during transfer:
```rust
pub fn transfer_with_fee(ctx: Context<TransferWithFee>, amount: u64) -> Result<()> {
    let fee = amount / 100; // 1% fee

    // Transfer to recipient
    token_interface::transfer_checked(transfer_context, amount - fee, decimals)?;

    // Burn the fee
    token_interface::burn(burn_context, fee)?;

    Ok(())
}
```

---

## Comparison: Burn vs Transfer

| Aspect | Burn | Transfer |
|--------|------|----------|
| **Token account** | Decrements balance | Decrements balance |
| **Mint supply** | Decrements total supply | No change |
| **Recipient** | None (tokens destroyed) | Required |
| **Authority** | Token account owner | Token account owner |
| **CPI call** | `token_interface::burn` | `token_interface::transfer_checked` |

---

## Next Steps

**Continue learning:** [Pattern: CPI to Token Program](../cross-program-calls/cpi-to-token.md)
- More token operations (freeze, revoke, etc.)
- Advanced CPI patterns
- Error handling in CPI calls

**Pattern:** [Signer Validation](../security/signer-validation.md)
- Advanced authority patterns
- Multi-signature burns
- Role-based burning permissions

**Deep dive:** [Concept: Account Model](../../concepts/account-model.md)
- Understanding token supply tracking
- How mint supply changes with burn/mint
