//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use {num_derive::FromPrimitive, thiserror::Error};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum PaladinLockupError {
    /// 0 - Incorrect mint.
    #[error("Incorrect mint.")]
    IncorrectMint = 0x0,
    /// 1 - Incorrect escrow authority address.
    #[error("Incorrect escrow authority address.")]
    IncorrectEscrowAuthorityAddress = 0x1,
    /// 2 - Incorrect escrow token account.
    #[error("Incorrect escrow token account.")]
    IncorrectEscrowTokenAccount = 0x2,
    /// 3 - Lockup is still active.
    #[error("Lockup is still active.")]
    LockupActive = 0x3,
}

impl solana_program::program_error::PrintProgramError for PaladinLockupError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
