use anchor_lang::prelude::*;

use crate::AcceptOwnership;

pub fn handler(ctx: Context<AcceptOwnership>) -> Result<()> {
    let registry = &mut ctx.accounts.registry;

    // Validate caller is the pending owner
    require!(
        registry.pending_owner == ctx.accounts.signer.key(),
        crate::AccessRegistryError::NotPendingOwner
    );

    // Transfer ownership
    registry.owner = registry.pending_owner;
    registry.pending_owner = Pubkey::default();

    msg!("Ownership transferred to {}", registry.owner);

    Ok(())
}
