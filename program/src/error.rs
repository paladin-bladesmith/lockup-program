//! Program error types.

use spl_program_error::*;

/// Errors that can be returned by the Paladin Lockup program.
#[spl_program_error]
pub enum PaladinLockupError {
    /// Incorrect mint.
    #[error("Incorrect mint.")]
    IncorrectMint,
    /// Incorrect escrow authority address.
    #[error("Incorrect escrow authority address.")]
    IncorrectEscrowAuthorityAddress,
    /// Incorrect escrow token account.
    #[error("Incorrect escrow token account.")]
    IncorrectEscrowTokenAccount,
    /// Lockup is still active.
    #[error("Lockup is still active.")]
    LockupActive,
}
