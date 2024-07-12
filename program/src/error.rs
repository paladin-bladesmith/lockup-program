//! Program error types.

use spl_program_error::*;

/// Errors that can be returned by the Paladin Lockup program.
#[spl_program_error]
pub enum PaladinLockupError {
    /// Incorrect token account.
    #[error("Incorrect token account.")]
    IncorrectTokenAccount,
    /// Incorrect escrow address.
    #[error("Incorrect escrow address.")]
    IncorrectEscrowAddress,
    /// Incorrect escrow token account.
    #[error("Incorrect escrow token account.")]
    IncorrectEscrowTokenAccount,
    /// Token account mint mismatch.
    #[error("Token account mint mismatch.")]
    TokenAccountMintMismatch,
    /// Lockup is still active.
    #[error("Lockup is still active.")]
    LockupActive,
}
