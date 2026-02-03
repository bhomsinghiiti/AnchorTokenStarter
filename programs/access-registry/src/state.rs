use anchor_lang::prelude::*;

/// Maximum batch size for batch operations
/// Solana transaction size limits constrain this to ~15 addresses
pub const MAX_BATCH_SIZE: usize = 15;

/// Main Access Registry Account
///
/// This PDA stores the registry configuration and is the central authority
/// for blacklist management in the GYLD protocol.
///
/// # Seeds
/// `["access_registry"]`
///
/// # Space Calculation
/// - discriminator: 8 bytes
/// - owner: 32 bytes
/// - pending_owner: 32 bytes
/// - chainalysis_oracle: 32 bytes
/// - pool_factory_owner: 32 bytes
/// - blacklist_count: 4 bytes
/// - bump: 1 byte
/// - Total: 141 bytes
#[account]
#[derive(InitSpace)]
pub struct AccessRegistry {
    /// Registry owner (typically a multisig wallet)
    /// Can initialize, update blacklist, and transfer ownership
    pub owner: Pubkey,

    /// Pending owner for two-step ownership transfer
    /// If set, this address can call accept_ownership to become the new owner
    pub pending_owner: Pubkey,

    /// Chainalysis Oracle program ID
    /// If set, all approval checks will CPI to this program
    /// If Pubkey::default(), oracle checks are skipped
    pub chainalysis_oracle: Pubkey,

    /// PoolFactory owner address
    /// This address is auto-approved (bypasses all blacklist and oracle checks)
    pub pool_factory_owner: Pubkey,

    /// Count of blacklisted addresses
    /// Used for tracking and validation
    pub blacklist_count: u32,

    /// PDA bump for this account
    /// Used for validation and signing
    pub bump: u8,
}

/// Per-Address Blacklist Entry
///
/// Each blacklisted address has its own PDA account.
/// This design is rent-efficient (only pay for what you use) and
/// allows enumeration via getProgramAccounts RPC.
///
/// # Seeds
/// `["blacklist", account.as_ref()]`
///
/// # Space Calculation
/// - discriminator: 8 bytes
/// - account: 32 bytes
/// - blacklisted: 1 byte
/// - timestamp: 8 bytes
/// - bump: 1 byte
/// - Total: 50 bytes
#[account]
#[derive(InitSpace)]
pub struct BlacklistEntry {
    /// The address being blacklisted
    pub account: Pubkey,

    /// Blacklist status
    /// If true, this address is blacklisted
    /// If false but account exists, entry is stale and should be closed
    pub blacklisted: bool,

    /// Unix timestamp when this entry was created
    pub timestamp: i64,

    /// PDA bump for this account
    pub bump: u8,
}

impl AccessRegistry {
    /// Check if an address is the registry owner
    pub fn is_registry_owner(&self, address: &Pubkey) -> bool {
        &self.owner == address
    }

    /// Check if an address is the pool factory owner
    pub fn is_pool_factory_owner(&self, address: &Pubkey) -> bool {
        &self.pool_factory_owner == address
    }

    /// Check if an address is a special (auto-approved) address
    pub fn is_special_address(&self, address: &Pubkey) -> bool {
        self.is_registry_owner(address) || self.is_pool_factory_owner(address)
    }

    /// Check if the Chainalysis oracle is configured
    pub fn has_oracle(&self) -> bool {
        self.chainalysis_oracle != Pubkey::default()
    }

    /// Check if there's a pending owner transfer
    pub fn has_pending_owner(&self) -> bool {
        self.pending_owner != Pubkey::default()
    }
}
