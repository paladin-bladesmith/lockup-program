//! Program error types.
use {
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

/// Errors that can be returned by the Paladin Lockup program.
// Note: Shank does not export the type when we use `spl_program_error`.
#[derive(Error, Clone, Debug, Eq, PartialEq)]
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
}

impl PrintProgramError for PaladinLockupError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<PaladinLockupError> for ProgramError {
    fn from(e: PaladinLockupError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for PaladinLockupError {
    fn type_of() -> &'static str {
        "PaladinLockupError"
    }
}
