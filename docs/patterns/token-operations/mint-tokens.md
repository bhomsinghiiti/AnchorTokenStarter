## Pattern: Mint Tokens

**Concept:** Minting to ATAs using Cross-Program Invocation (CPI) and PDA signing
**Related Code:** `programs/anchortokenstarter/src/lib.rs: mint_tokens instruction, MintTokens accounts struct`
**Test:** `tests/anchortokenstarter.ts: "Mints tokens to an associated token account"`
**Prerequisites:** [Create Mint](create-mint.md)

---

## What You're Building

An instruction that:
- Mints tokens to a user's Associated Token Account (ATA)
- Uses CPI (Cross-Program Invocation) to call SPL Token program
- Signs using the mint PDA (program-controlled mint authority)
- Creates the ATA if it doesn't exist

This pattern teaches:
- Cross-Program Invocation (CPI)
- PDA signing for programs
- Associated Token Accounts (ATAs)
- The `init_if_needed` constraint for ATAs

---

## The Concept

### Cross-Program Invocation (CPI)

**CPI** = Your program calling another program.

In Solidity, you call other contracts:
```solidity
IERC20(token).transfer(recipient, amount);
```

In Solana, you use CPI:
```rust
token_interface::mint_to(cpi_context, amount)?;
```

**Key difference:**
- Solidity: Contract calls are implicit in EVM
- Solana: All accounts must be explicitly provided

### Associated Token Accounts (ATA)

An **ATA** is a deterministic token account address:
- Derived from: `owner + mint + ATA program`
- Each user has one ATA per mint
- Standard for holding tokens

**Formula:**
```rust
ata = getAssociatedTokenAddress(mint, owner)
```

**Comparison:**
- ERC20: `balanceOf[user]` is inside contract
- SPL Token: Separate `TokenAccount` for each user+mint pair

### PDA Signing

Since the mint authority is the mint itself (a PDA), your program can sign for it:

```rust
let seeds: &[&[&[u8]]] = &[&[b"mint"], &[ctx.bumps.mint]];
let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(seeds);
```

**How it works:**
1. Derive PDA seeds (`["mint"]`)
2. Get bump from Anchor (`ctx.bumps.mint`)
3. Pass to CPI with `with_signer`
4. SPL Token program verifies PDA signature

---

## The Code

```rust
// programs/anchortokenstarter/src/lib.rs

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, MintTo, TokenAccount, TokenInterface},
};

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Mint tokens to a token account
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        // PDA seeds for signing
        let seeds: &[&[&[u8]]] = &[&[b"mint"], &[ctx.bumps.mint]];

        // CPI accounts
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint.to_account_info(),  // Mint is authority
        };

        // Build CPI context
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(seeds);

        // Call SPL Token program
        token_interface::mint_to(cpi_context, amount)?;
        msg!("Minted {} tokens", amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"mint"],
        bump,
        mint::authority = mint,  // Verify mint is authority
    )]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,  // Create ATA if it doesn't exist
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
```

**CPI components:**
- `MintTo` struct - accounts required by `mint_to` instruction
- `CpiContext::new` - builds the invocation context
- `.with_signer(seeds)` - adds PDA signature
- `token_interface::mint_to` - executes the CPI

---

## Running It

```bash
# Run the mint tokens test
anchor test --skip-build -- --grep "Mints tokens to an associated token account"
```

**Expected output:**
```
  AnchorTokenStarter
    SPL Token Tests
      ✔ Mints tokens to an associated token account (434ms)
```

**Program logs:**
```
Program log:Minted 1000000000 tokens
```

---

## If It Fails

### "MintToInvalidAuthority"
**Cause:** Mint authority check failed

**Solution:** Ensure:
1. Mint PDA seeds match create_mint seeds
2. `mint::authority = mint` constraint in CreateMint
3. Correct bump seed used

### "Account not rent exempt"
**Cause:** ATA creation failed due to insufficient funds

**Solution:** Airdrop SOL to signer:
```bash
solana airdrop 2 <signer_address>
```

### "Associated token account not found"
**Cause:** ATA constraint missing `init_if_needed`

**Solution:** Add `init_if_needed` to ATA account constraint (see code above)

### See also
[Common Errors](../../reference/common-errors.md)

---

## Solidity Sidebar

**ERC20 minting:**
```solidity
// Inside token contract
function mint(address to, uint256 amount) external {
    _mint(to, amount);
}
```

**SPL Token minting:**
```rust
// Your program calls SPL Token program
token_interface::mint_to(cpi_context, amount)?;
```

**Key differences:**
- ERC20: Mint logic inside your token contract
- SPL Token: Your program calls Token Program via CPI
- SPL Token: Need to create ATA first (or use `init_if_needed`)

---

## You've Learned

- ✅ Cross-Program Invocation (CPI) structure
- ✅ PDA signing with `with_signer(seeds)`
- ✅ Associated Token Accounts (ATAs)
- ✅ The `init_if_needed` constraint for ATAs
- ✅ Building CPI contexts and calling external programs

---

## Next Steps

**Continue learning:** [Pattern: Transfer Tokens](transfer-tokens.md)
- Transfer tokens between users
- Multi-account validation
- Authority checks for transfers

**Deep dive:** [Concept: PDAs Explained](../../concepts/pdas-explained.md)
- Full PDA derivation explanation
- Bump seeds and canonical bumps
- PDA signing patterns

**Pattern:** [CPI to Token Program](../cross-program-calls/cpi-to-token.md)
- Detailed CPI patterns and patterns
- Other token operations (burn, freeze, etc.)
