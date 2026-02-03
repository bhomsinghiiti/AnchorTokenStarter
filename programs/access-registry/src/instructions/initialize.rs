use anchor_lang::prelude::*;

use crate::{Initialize, AccessRegistryError, MAX_BATCH_SIZE};

pub fn handler(
    ctx: Context<Initialize>,
    chainalysis_oracle: Pubkey,
    pool_factory_owner: Pubkey,
    initial_blacklist: Vec<Pubkey>,
) -> Result<()> {
    let registry = &mut ctx.accounts.registry;

    // Validate addresses
    require!(
        pool_factory_owner != Pubkey::default(),
        AccessRegistryError::InvalidPoolFactoryAddress
    );

    // Initialize registry
    registry.owner = ctx.accounts.payer.key();
    registry.pending_owner = Pubkey::default();
    registry.chainalysis_oracle = chainalysis_oracle;
    registry.pool_factory_owner = pool_factory_owner;
    registry.blacklist_count = 0;
    registry.bump = ctx.bumps.registry;

    msg!("AccessRegistry initialized");
    msg!("Owner: {}", registry.owner);
    msg!("Chainalysis Oracle: {}", registry.chainalysis_oracle);
    msg!("Pool Factory Owner: {}", registry.pool_factory_owner);

    // Process initial blacklist if provided
    if !initial_blacklist.is_empty() {
        require!(
            initial_blacklist.len() <= MAX_BATCH_SIZE,
            AccessRegistryError::InvalidBatchSize
        );

        // Validate no special addresses in initial blacklist
        for address in &initial_blacklist {
            if registry.is_special_address(address) {
                return Err(AccessRegistryError::CannotBlacklistSpecialAddress.into());
            }
        }

        msg!("Pre-populating blacklist with {} addresses", initial_blacklist.len());
        // Note: In full implementation, would create BlacklistEntry PDAs here
        // For now, just set the count
        registry.blacklist_count = initial_blacklist.len() as u32;
    }

    Ok(())
}
