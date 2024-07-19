//! Program processor.

use {
    crate::{
        error::PaladinLockupError,
        instruction::PaladinLockupInstruction,
        state::{
            collect_escrow_authority_signer_seeds, get_escrow_authority_address,
            get_escrow_authority_address_and_bump_seed, Lockup,
        },
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
        sysvar::Sysvar,
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
    spl_discriminator::{ArrayDiscriminator, SplDiscriminate},
    spl_token_2022::{extension::StateWithExtensions, state::Mint},
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

    let authority_info = next_account_info(accounts_iter)?;
    let token_account_info = next_account_info(accounts_iter)?;
    let lockup_info = next_account_info(accounts_iter)?;
    let escrow_authority_info = next_account_info(accounts_iter)?;
    let escrow_token_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?;

    // Note that Token-2022's `TransferChecked` processor will assert the
    // following:
    //
    // * The provided authority account is a signer.
    // * The provided authority is authorized to transfer the tokens.
    // * The provided token account is for the provided mint.

    // Ensure the lockup account is owned by the Paladin Lockup program.
    if lockup_info.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Ensure the lockup account has enough space.
    if lockup_info.data_len() != std::mem::size_of::<Lockup>() {
        return Err(ProgramError::InvalidAccountData);
    }

    // Ensure the lockup account is not initialized.
    if &lockup_info.try_borrow_data()?[0..8] != ArrayDiscriminator::UNINITIALIZED.as_slice() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Ensure the provided escrow authority address is correct.
    if !escrow_authority_info
        .key
        .eq(&get_escrow_authority_address(program_id))
    {
        return Err(PaladinLockupError::IncorrectEscrowAuthorityAddress.into());
    }

    // Ensure the provided escrow token account address is correct.
    if !escrow_token_account_info
        .key
        .eq(&get_associated_token_address_with_program_id(
            escrow_authority_info.key,
            mint_info.key,
            token_program_info.key,
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
            authority_info.key,
            lockup_start_timestamp,
            lockup_end_timestamp,
            mint_info.key,
        );

    // Transfer the tokens to the escrow token account.
    {
        let decimals = {
            let mint_data = mint_info.try_borrow_data()?;
            let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
            mint.base.decimals
        };

        spl_token_2022::onchain::invoke_transfer_checked(
            &spl_token_2022::id(),
            token_account_info.clone(),
            mint_info.clone(),
            escrow_token_account_info.clone(),
            authority_info.clone(),
            accounts_iter.as_slice(),
            amount,
            decimals,
            &[],
        )?;
    }

    Ok(())
}

/// Processes an
/// [Unlock](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_unlock(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let authority_info = next_account_info(accounts_iter)?;
    let lockup_info = next_account_info(accounts_iter)?;

    // Ensure the authority is a signer.
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the lockup account is owned by the Paladin Lockup program.
    if lockup_info.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Ensure the lockup account is initialized.
    if !(lockup_info.data_len() == std::mem::size_of::<Lockup>()
        && &lockup_info.try_borrow_data()?[0..8] == Lockup::SPL_DISCRIMINATOR_SLICE)
    {
        return Err(ProgramError::UninitializedAccount);
    }

    let mut data = lockup_info.try_borrow_mut_data()?;
    let state = bytemuck::try_from_bytes_mut::<Lockup>(&mut data)
        .map_err(|_| ProgramError::InvalidAccountData)?;

    // Ensure the provided authority is the same as the lockup's authority.
    if state.authority != *authority_info.key {
        return Err(ProgramError::IncorrectAuthority);
    }

    // Get the timestamp from the clock sysvar, and use it to override the end
    // timestamp of the lockup, effectively unlocking the funds.
    let clock = <Clock as Sysvar>::get()?;
    let timestamp = clock.unix_timestamp as u64;
    if state.lockup_end_timestamp > timestamp {
        state.lockup_end_timestamp = timestamp;
    }

    Ok(())
}

/// Processes a
/// [Withdraw](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_withdraw(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let authority_info = next_account_info(accounts_iter)?;
    let token_account_info = next_account_info(accounts_iter)?;
    let lockup_info = next_account_info(accounts_iter)?;
    let escrow_authority_info = next_account_info(accounts_iter)?;
    let escrow_token_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let token_program_info = next_account_info(accounts_iter)?;

    // Note that Token-2022's `TransferChecked` processor will assert the
    // provided token account is for the provided mint.

    // Ensure the authority is a signer.
    if !authority_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the lockup account is owned by the Paladin Lockup program.
    if lockup_info.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }

    // Ensure the lockup account is initialized.
    if !(lockup_info.data_len() == std::mem::size_of::<Lockup>()
        && &lockup_info.try_borrow_data()?[0..8] == Lockup::SPL_DISCRIMINATOR_SLICE)
    {
        return Err(ProgramError::UninitializedAccount);
    }

    // Ensure the provided escrow authority address is correct.
    let (escrow_authority_address, bump_seed) =
        get_escrow_authority_address_and_bump_seed(program_id);
    if !escrow_authority_info.key.eq(&escrow_authority_address) {
        return Err(PaladinLockupError::IncorrectEscrowAuthorityAddress.into());
    }

    // Ensure the provided escrow token account address is correct.
    if !escrow_token_account_info
        .key
        .eq(&get_associated_token_address_with_program_id(
            escrow_authority_info.key,
            mint_info.key,
            token_program_info.key,
        ))
    {
        return Err(PaladinLockupError::IncorrectEscrowTokenAccount.into());
    }

    let withdraw_amount = {
        let data = lockup_info.try_borrow_data()?;
        let state = bytemuck::try_from_bytes::<Lockup>(&data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Ensure the provided authority is the same as the lockup's authority.
        if state.authority != *authority_info.key {
            return Err(ProgramError::IncorrectAuthority);
        }

        // Ensure the provided mint is the same as the lockup's mint.
        if state.mint != *mint_info.key {
            return Err(PaladinLockupError::IncorrectMint.into());
        }

        // Ensure the lockup has ended.
        let clock = <Clock as Sysvar>::get()?;
        let timestamp = clock.unix_timestamp as u64;
        if state.lockup_end_timestamp > timestamp {
            msg!(
                "Lockup has not ended yet. {} seconds remaining.",
                state.lockup_end_timestamp.saturating_sub(timestamp)
            );
            return Err(PaladinLockupError::LockupActive.into());
        }

        state.amount
    };

    // Transfer the tokens to the depositor.
    {
        let bump_seed = [bump_seed];
        let escrow_authority_signer_seeds = collect_escrow_authority_signer_seeds(&bump_seed);

        let decimals = {
            let mint_data = mint_info.try_borrow_data()?;
            let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
            mint.base.decimals
        };

        spl_token_2022::onchain::invoke_transfer_checked(
            &spl_token_2022::id(),
            escrow_token_account_info.clone(),
            mint_info.clone(),
            token_account_info.clone(),
            escrow_authority_info.clone(),
            accounts_iter.as_slice(),
            withdraw_amount,
            decimals,
            &[&escrow_authority_signer_seeds],
        )?;
    }

    let new_token_account_lamports = lockup_info
        .lamports()
        .checked_add(token_account_info.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;

    **lockup_info.try_borrow_mut_lamports()? = 0;
    **token_account_info.try_borrow_mut_lamports()? = new_token_account_lamports;

    lockup_info.realloc(0, true)?;
    lockup_info.assign(&system_program::id());

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
    }
}
