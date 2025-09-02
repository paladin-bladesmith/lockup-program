#![allow(non_local_definitions)]
//! Program error types.

use {num_derive::FromPrimitive, solana_program::program_error::ProgramError, thiserror::Error};

/// Errors that can be returned by the Paladin Lockup program.
// Note: Shank does not export the type when we use `spl_program_error`.
#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
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
    /// Lockup already unlocked.
    #[error("Lockup already unlocked.")]
    LockupAlreadyUnlocked,
    /// The amount was too low to be entered into the pool.
    #[error("Amount too low.")]
    AmountTooLow,
    /// The provided pool account did not match the lockup.
    #[error("Incorrect pool account")]
    IncorrectPool,
}

impl From<PaladinLockupError> for ProgramError {
    fn from(e: PaladinLockupError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
