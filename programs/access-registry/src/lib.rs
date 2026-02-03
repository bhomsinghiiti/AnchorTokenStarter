// Access Registry Program
//
// Centralized blacklist registry with Chainalysis Oracle integration.
// Converted from GYLD's AccessRegistry.sol to Anchor/Rust.
//
// This program maintains an internal blacklist and integrates with the
// Chainalysis Oracle for OFAC SDN list compliance.
//
// Key Features:
// - Internal blacklist (owner-controlled)
// - Chainalysis Oracle integration (fail-closed design)
// - Auto-approval for registry owner and pool factory owner
// - Batch operations (up to 15 addresses)
// - Two-step ownership transfer
// - Re-initialization attack protection

declare_id!("25fGver7srxMVBXA8H7eMXMUqXAkxiHLF1w7V91t9Zfw");

pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use error::AccessRegistryError;
pub use state::*;

// =============================================================================
// ACCOUNT STRUCTS
// =============================================================================

/// Accounts for initialize instruction
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + AccessRegistry::INIT_SPACE,
        seeds = [b"access_registry"],
        bump
    )]
    pub registry: Account<'info, AccessRegistry>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts for set_blacklisted instruction
#[derive(Accounts)]
pub struct SetBlacklisted<'info> {
    #[account(
        mut,
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + BlacklistEntry::INIT_SPACE,
        seeds = [b"blacklist", account.key().as_ref()],
        bump
    )]
    pub blacklist_entry: Account<'info, BlacklistEntry>,

    /// CHECK: account is only used as seed
    pub account: UncheckedAccount<'info>,

    #[account(
        mut,
        constraint = registry.owner == authority.key() @ AccessRegistryError::Unauthorized
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts for set_blacklisted_batch instruction
#[derive(Accounts)]
pub struct SetBlacklistedBatch<'info> {
    #[account(
        mut,
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,

    #[account(
        constraint = registry.owner == authority.key() @ AccessRegistryError::Unauthorized
    )]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

/// Accounts for is_approved instruction
#[derive(Accounts)]
pub struct IsApproved<'info> {
    #[account(
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,
}

/// Accounts for is_sanctioned_by_chainalysis instruction
#[derive(Accounts)]
pub struct IsSanctionedByChainalysis<'info> {
    #[account(
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,
}

/// Accounts for get_approved_batch instruction
#[derive(Accounts)]
pub struct GetApprovedBatch<'info> {
    #[account(
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,
}

/// Accounts for transfer_ownership instruction
#[derive(Accounts)]
pub struct TransferOwnership<'info> {
    #[account(
        mut,
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,

    /// CHECK: pending_owner is only stored, not validated beyond being non-zero
    pub pending_owner: UncheckedAccount<'info>,

    #[account(
        constraint = registry.owner == authority.key() @ AccessRegistryError::Unauthorized
    )]
    pub authority: Signer<'info>,
}

/// Accounts for accept_ownership instruction
#[derive(Accounts)]
pub struct AcceptOwnership<'info> {
    #[account(
        mut,
        seeds = [b"access_registry"],
        bump = registry.bump
    )]
    pub registry: Account<'info, AccessRegistry>,

    pub signer: Signer<'info>,
}

// =============================================================================
// PROGRAM
// =============================================================================

#[program]
pub mod access_registry {
    use super::*;

    /// Initialize the Access Registry
    ///
    /// Creates the main registry PDA with owner, oracle, and pool factory configuration.
    /// Optionally accepts an initial blacklist of addresses.
    ///
    /// # Arguments
    /// * `chainalysis_oracle` - Chainalysis Oracle program ID (use default() to disable)
    /// * `pool_factory_owner` - PoolFactory owner address (auto-approved)
    /// * `initial_blacklist` - Optional array of addresses to blacklist on initialization
    ///
    /// # Security
    /// This function can only be called once. Subsequent calls will fail with
    /// AccessRegistryError::AlreadyInitialized to prevent re-initialization attacks.
    pub fn initialize(
        ctx: Context<Initialize>,
        chainalysis_oracle: Pubkey,
        pool_factory_owner: Pubkey,
        initial_blacklist: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, chainalysis_oracle, pool_factory_owner, initial_blacklist)
    }

    /// Set blacklist status for a single address
    ///
    /// Creates or closes a BlacklistEntry PDA for the specified address.
    ///
    /// # Arguments
    /// * `account` - The address to blacklist or unblacklist
    /// * `blacklisted` - true to blacklist, false to unblacklist
    ///
    /// # Security
    /// - Only the registry owner can call this function
    /// - Cannot blacklist the registry owner or pool factory owner
    /// - When unblacklisting, the PDA is closed and rent is returned to the owner
    pub fn set_blacklisted(
        ctx: Context<SetBlacklisted>,
        account: Pubkey,
        blacklisted: bool,
    ) -> Result<()> {
        instructions::set_blacklisted::handler(ctx, account, blacklisted)
    }

    /// Set blacklist status for multiple addresses
    ///
    /// Atomically updates blacklist status for up to 15 addresses.
    /// If any operation fails, the entire transaction is reverted.
    ///
    /// # Arguments
    /// * `accounts` - Array of addresses to update
    /// * `blacklisted` - true to blacklist, false to unblacklist all
    ///
    /// # Security
    /// - Only the registry owner can call this function
    /// - Cannot blacklist registry owner or pool factory owner (transaction fails)
    /// - Maximum batch size is 15 addresses
    pub fn set_blacklisted_batch(
        ctx: Context<SetBlacklistedBatch>,
        accounts: Vec<Pubkey>,
        blacklisted: bool,
    ) -> Result<()> {
        instructions::set_blacklisted_batch::handler(ctx, accounts, blacklisted)
    }

    /// Check if an address is approved
    ///
    /// Returns true if the address passes all checks:
    /// 1. Auto-approved if registry owner
    /// 2. Auto-approved if pool factory owner
    /// 3. Rejected if internally blacklisted
    /// 4. Rejected if Chainalysis Oracle returns sanctioned (fail-closed)
    ///
    /// # Arguments
    /// * `account` - The address to check
    ///
    /// # Note
    /// This is an on-chain instruction that costs compute units.
    /// For optimization, clients should first check internal blacklist via RPC
    /// and only call this for oracle verification.
    pub fn is_approved(
        ctx: Context<IsApproved>,
        account: Pubkey,
    ) -> Result<()> {
        instructions::is_approved::handler(ctx, account)
    }

    /// Check if an address is sanctioned by Chainalysis Oracle only
    ///
    /// This function only checks the Chainalysis Oracle, skipping internal
    /// blacklist and auto-approval logic.
    ///
    /// # Arguments
    /// * `account` - The address to check
    ///
    /// # Fail-Closed
    /// If the oracle call fails for any reason, this function reverts.
    pub fn is_sanctioned_by_chainalysis(
        ctx: Context<IsSanctionedByChainalysis>,
        account: Pubkey,
    ) -> Result<()> {
        instructions::is_sanctioned_by_chainalysis::handler(ctx, account)
    }

    /// Check approval status for multiple addresses
    ///
    /// Returns approval status for up to 15 addresses in a single call.
    ///
    /// # Arguments
    /// * `accounts` - Array of addresses to check
    ///
    /// # Returns
    /// Writes results to the results account (one bool per input address)
    pub fn get_approved_batch(
        ctx: Context<GetApprovedBatch>,
        accounts: Vec<Pubkey>,
    ) -> Result<()> {
        instructions::get_approved_batch::handler(ctx, accounts)
    }

    /// Initiate ownership transfer
    ///
    /// Sets the pending_owner field. The pending owner must call
    /// accept_ownership to complete the transfer.
    ///
    /// # Arguments
    /// * `pending_owner` - The address to transfer ownership to
    ///
    /// # Security
    /// Only the current owner can call this function.
    pub fn transfer_ownership(
        ctx: Context<TransferOwnership>,
        pending_owner: Pubkey,
    ) -> Result<()> {
        instructions::transfer_ownership::handler(ctx, pending_owner)
    }

    /// Accept ownership transfer
    ///
    /// Completes the ownership transfer initiated by transfer_ownership.
    /// Only the pending owner can call this function.
    pub fn accept_ownership(ctx: Context<AcceptOwnership>) -> Result<()> {
        instructions::accept_ownership::handler(ctx)
    }
}
