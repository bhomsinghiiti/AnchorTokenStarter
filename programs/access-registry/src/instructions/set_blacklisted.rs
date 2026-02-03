use anchor_lang::prelude::*;

use crate::{SetBlacklisted, AccessRegistryError};

pub fn handler(ctx: Context<SetBlacklisted>, account: Pubkey, blacklisted: bool) -> Result<()> {
    let registry = &mut ctx.accounts.registry;

    // Prevent blacklisting special addresses
    if registry.is_special_address(&account) {
        return Err(AccessRegistryError::CannotBlacklistSpecialAddress.into());
    }

    let entry = &mut ctx.accounts.blacklist_entry;

    if blacklisted {
        // Create or update blacklist entry
        if entry.account != Pubkey::default() && entry.blacklisted {
            return Err(AccessRegistryError::AlreadyBlacklisted.into());
        }

        entry.account = account;
        entry.blacklisted = true;
        entry.timestamp = Clock::get()?.unix_timestamp;
        entry.bump = ctx.bumps.blacklist_entry;

        registry.blacklist_count = registry.blacklist_count.saturating_add(1);

        msg!("Blacklisted: {}", account);
    } else {
        // Close blacklist entry (unblacklist)
        if entry.account == Pubkey::default() || !entry.blacklisted {
            return Err(AccessRegistryError::NotBlacklisted.into());
        }

        // Close the account and return lamports to the owner
        ctx.accounts.blacklist_entry.close(ctx.accounts.authority.to_account_info())?;

        registry.blacklist_count = registry.blacklist_count.saturating_sub(1);

        msg!("Unblacklisted: {}", account);
    }

    Ok(())
}
