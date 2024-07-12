//! Program processor.

use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

/// Processes a
/// [PaladinLockupInstruction](enum.PaladinLockupInstruction.html).
pub fn process(_program_id: &Pubkey, _accounts: &[AccountInfo], _input: &[u8]) -> ProgramResult {
    Ok(())
}
