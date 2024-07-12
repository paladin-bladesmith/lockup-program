//! Program error types.

use spl_program_error::*;

/// Errors that can be returned by the Paladin Lockup program.
#[spl_program_error]
pub enum PaladinLockupError {
    /// This is a placeholder error.
    #[error("This is a placeholder error.")]
    Placeholder,
}
