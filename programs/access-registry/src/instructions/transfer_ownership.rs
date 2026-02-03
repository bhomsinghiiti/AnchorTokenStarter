use anchor_lang::prelude::*;

use crate::{TransferOwnership, AccessRegistryError};

pub fn handler(ctx: Context<TransferOwnership>, pending_owner: Pubkey) -> Result<()> {
    let registry = &mut ctx.accounts.registry;

    // Validate pending owner is not zero address
    require!(
        pending_owner != Pubkey::default(),
        AccessRegistryError::InvalidPendingOwner
    );

    // Validate pending owner is not special address
    require!(
        !registry.is_special_address(&pending_owner),
        AccessRegistryError::InvalidPendingOwner
    );

    registry.pending_owner = pending_owner;

    msg!("Ownership transfer initiated to {}", pending_owner);

    Ok(())
}
