// Instruction handlers for the Access Registry program
//
// Each instruction has its own module with:
// - A Context struct defining the accounts
// - A handler function implementing the logic

pub mod accept_ownership;
pub mod get_approved_batch;
pub mod initialize;
pub mod is_approved;
pub mod is_sanctioned_by_chainalysis;
pub mod set_blacklisted;
pub mod set_blacklisted_batch;
pub mod transfer_ownership;
