use anchor_lang::prelude::*;

use crate::IsApproved;

pub fn handler(ctx: Context<IsApproved>, account: Pubkey) -> Result<()> {
    let registry = &ctx.accounts.registry;

    // Auto-approve registry owner
    if registry.is_registry_owner(&account) {
        msg!("Auto-approved: registry owner");
        return Ok(());
    }

    // Auto-approve pool factory owner
    if registry.is_pool_factory_owner(&account) {
        msg!("Auto-approved: pool factory owner");
        return Ok(());
    }

    // Check if Chainalysis oracle is configured
    if registry.has_oracle() {
        msg!("Checking Chainalysis oracle for {}", account);
        // TODO: Implement CPI to oracle
        msg!("Oracle CPI not yet implemented");
    }

    // TODO: Check internal blacklist via RPC optimization
    msg!("is_approved check complete for {}", account);

    Ok(())
}
