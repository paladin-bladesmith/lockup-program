//! Program processor.

use {
    crate::instruction::PaladinLockupInstruction,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey},
};

/// Processes a
/// [Lockup](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_lockup(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _amount: u64,
    _period_seconds: u64,
) -> ProgramResult {
    Ok(())
}

/// Processes an
/// [Unlock](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_unlock(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [Withdraw](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_withdraw(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes an
/// [InitializeEscrow](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_initialize_escrow(_program_id: &Pubkey, _accounts: &[AccountInfo]) -> ProgramResult {
    Ok(())
}

/// Processes a
/// [PaladinLockupInstruction](enum.PaladinLockupInstruction.html).
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
    let instruction = PaladinLockupInstruction::unpack(input)?;
    match instruction {
        PaladinLockupInstruction::Lockup {
            amount,
            period_seconds,
        } => {
            msg!("Instruction: Lockup");
            process_lockup(program_id, accounts, amount, period_seconds)
        }
        PaladinLockupInstruction::Unlock => {
            msg!("Instruction: Unlock");
            process_unlock(program_id, accounts)
        }
        PaladinLockupInstruction::Withdraw => {
            msg!("Instruction: Withdraw");
            process_withdraw(program_id, accounts)
        }
        PaladinLockupInstruction::InitializeEscrow => {
            msg!("Instruction: InitializeEscrow");
            process_initialize_escrow(program_id, accounts)
        }
    }
}
