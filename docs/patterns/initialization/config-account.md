## Pattern: Config Account

**Concept:** Multi-field state accounts for configuration
**Prerequisites:** [Simple Counter](simple-counter.md)
**Status:** 📝 Exercise

---

## Exercise: Create a Config Account

Implement a configuration account that stores multiple fields for program settings.

### Requirements

Create a `Config` account with:
- `authority: Pubkey` - The admin who can update config
- `fee_rate: u64` - Fee percentage in basis points (100 = 1%)
- `is_paused: bool` - Whether the program is paused

Implement two instructions:
1. `initialize_config` - Create the config account (only once)
2. `update_config` - Update config fields (only by authority)

### Hints

**Account struct:**
```rust
#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub fee_rate: u64,
    pub is_paused: bool,
}
```

**PDA seeds:** Use `b"config"` for deterministic address

**Initialization constraint:**
```rust
#[account(
    init,
    payer = payer,
    space = 8 + Config::INIT_SPACE,
    seeds = [b"config"],
    bump
)]
pub config: Account<'info, Config>,
```

**Authority validation:**
```rust
require!(ctx.accounts.authority.key() == ctx.accounts.config.authority, ErrorCode::Unauthorized);
```

### Expected Tests

```typescript
describe("Config Account", () => {
  const [configPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  it("Initializes config", async () => {
    await program.methods
      .initializeConfig()
      .accounts({
        config: configPda,
        payer: provider.wallet.publicKey,
        authority: provider.wallet.publicKey,
      })
      .rpc();
  });

  it("Updates config as authority", async () => {
    await program.methods
      .updateConfig(new BN(500), true)  // 5% fee, paused
      .accounts({
        config: configPda,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const config = await program.account.config.fetch(configPda);
    expect(config.feeRate.toNumber()).to.equal(500);
    expect(config.isPaused).to.be.true;
  });

  it("Fails when non-authority tries to update", async () => {
    const nonAuthority = Keypair.generate();

    await expect(program.methods
      .updateConfig(new BN(100), false)
      .accounts({
        config: configPda,
        authority: nonAuthority.publicKey,
      })
      .signers([nonAuthority])
      .rpc()
    ).to.be.rejected;
  });
});
```

### Solution

<details>
<summary>Click to reveal solution</summary>

```rust
// programs/anchortokenstarter/src/lib.rs

#[program]
pub mod anchortokenstarter {
    use super::*;

    /// Initialize config account
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        authority: Pubkey,
        fee_rate: u64,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.authority = authority;
        config.fee_rate = fee_rate;
        config.is_paused = false;
        msg!("Config initialized: authority={}, fee_rate={}", authority, fee_rate);
        Ok(())
    }

    /// Update config (authority only)
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        fee_rate: u64,
        is_paused: bool,
    ) -> Result<()> {
        let config = &mut ctx.accounts.config;

        // Validate authority
        require!(
            ctx.accounts.authority.key() == config.authority,
            ErrorCode::Unauthorized
        );

        config.fee_rate = fee_rate;
        config.is_paused = is_paused;
        msg!("Config updated: fee_rate={}, is_paused={}", fee_rate, is_paused);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut)]
    pub config: Account<'info, Config>,

    pub authority: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub fee_rate: u64,
    pub is_paused: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Only config authority can perform this action")]
    Unauthorized,
}
```

</details>

---

## What You'll Learn

- ✅ Multi-field account structures
- ✅ Authority validation patterns
- ✅ Difference between `init` (once) and `init_if_needed` (idempotent)
- ✅ Using `Pubkey` type in account structs
- ✅ Bool types in Solana accounts

---

## Common Patterns

### Immutable Config

Some configs should be immutable after creation:

```rust
// Omit update instruction, only allow initialization
pub fn initialize_config(ctx: Context<InitializeConfig>, ...) -> Result<()> {
    // Set once, never change
}
```

### Time-Based Pausing

Add `pause_until` timestamp instead of bool:

```rust
#[account]
pub struct Config {
    pub pause_until: i64,  // Unix timestamp, 0 = not paused
}

// Check in instructions
if config.pause_until > Clock::get()?.unix_timestamp {
    return Err(ErrorCode::Paused.into());
}
```

---

## Next Steps

**Continue learning:** [Pattern: Create Mint](../token-operations/create-mint.md)
- Create an SPL Token mint
- Learn about token-specific account structures
- Compare to ERC20 deployment

**See in action:** [Transfer Tokens](../token-operations/transfer-tokens.md)
- Multi-field validation in token transfers
- Complex account structures
