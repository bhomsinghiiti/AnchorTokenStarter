use anchor_lang::prelude::*;

/// Access Registry Error Codes
///
/// These errors cover all failure scenarios in the AccessRegistry program.
/// Error messages are designed to be clear and actionable for clients.
#[error_code]
pub enum AccessRegistryError {
    #[msg("Registry already initialized")]
    AlreadyInitialized,

    #[msg("Unauthorized: caller is not the owner")]
    Unauthorized,

    #[msg("Batch size must be between 1 and 15")]
    InvalidBatchSize,

    #[msg("Chainalysis oracle call failed")]
    OracleFailure,

    #[msg("Cannot blacklist registry owner or pool factory owner")]
    CannotBlacklistSpecialAddress,

    #[msg("Address is already blacklisted")]
    AlreadyBlacklisted,

    #[msg("Address is not blacklisted")]
    NotBlacklisted,

    #[msg("Invalid pending owner address")]
    InvalidPendingOwner,

    #[msg("Not the pending owner")]
    NotPendingOwner,

    #[msg("No pending owner transfer")]
    NoPendingTransfer,

    #[msg("Invalid oracle program address")]
    InvalidOracleAddress,

    #[msg("Invalid pool factory address")]
    InvalidPoolFactoryAddress,

    #[msg("Array length mismatch")]
    ArrayLengthMismatch,
}
