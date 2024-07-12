//! Program processor.

use {
    crate::{
        error::PaladinLockupError,
        instruction::PaladinLockupInstruction,
        state::{get_escrow_address, Lockup},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        sysvar::Sysvar,
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
    spl_discriminator::SplDiscriminate,
};

/// Processes a
/// [Lockup](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_lockup(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
    period_seconds: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let owner_info = next_account_info(accounts_iter)?;
    let token_account_info = next_account_info(accounts_iter)?;
    let lockup_info = next_account_info(accounts_iter)?;
    let escrow_info = next_account_info(accounts_iter)?;
    let escrow_token_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let _token_program_info = next_account_info(accounts_iter)?;

    // Ensure the owner account is a signer.
    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the provided token account address is correct.
    if !token_account_info
        .key
        .eq(&get_associated_token_address_with_program_id(
            owner_info.key,
            mint_info.key,
            &spl_token_2022::id(),
        ))
    {
        return Err(PaladinLockupError::IncorrectTokenAccount.into());
    }

    // Ensure the lockup account is owned by the Paladin Lockup program.
    if lockup_info.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Ensure the lockup account has enough space.
    if lockup_info.data_len() != std::mem::size_of::<Lockup>() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the lockup account is not initialized.
    if &lockup_info.try_borrow_data()?[0..8] == Lockup::SPL_DISCRIMINATOR_SLICE {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Ensure the provided escrow address is correct.
    if !escrow_info.key.eq(&get_escrow_address(program_id)) {
        return Err(PaladinLockupError::IncorrectEscrowAddress.into());
    }

    // Ensure the provided escrow token account address is correct.
    if !escrow_token_account_info
        .key
        .eq(&get_associated_token_address_with_program_id(
            escrow_info.key,
            mint_info.key,
            &spl_token_2022::id(),
        ))
    {
        return Err(PaladinLockupError::IncorrectEscrowTokenAccount.into());
    }

    // Get the timestamp from the clock sysvar, and use it to determine the
    // lockup start and end timestamp, using the provided period.
    let clock = <Clock as Sysvar>::get()?;
    let lockup_start_timestamp = clock.unix_timestamp as u64;
    let lockup_end_timestamp = lockup_start_timestamp
        .checked_add(period_seconds)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Write the data.
    let mut data = lockup_info.try_borrow_mut_data()?;
    *bytemuck::try_from_bytes_mut(&mut data).map_err(|_| ProgramError::InvalidAccountData)? =
        Lockup::new(
            amount,
            token_account_info.key,
            lockup_start_timestamp,
            lockup_end_timestamp,
        );

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
