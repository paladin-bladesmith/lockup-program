//! Program processor.

use {
    crate::{
        error::PaladinLockupError,
        instruction::PaladinLockupInstruction,
        state::{
            collect_escrow_authority_signer_seeds, collect_escrow_token_account_signer_seeds,
            get_escrow_authority_address, get_escrow_authority_address_and_bump_seed,
            get_escrow_token_account_address, get_escrow_token_account_address_and_bump_seed,
            Lockup,
        },
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        clock::Clock,
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction, system_program,
        sysvar::Sysvar,
    },
    spl_discriminator::SplDiscriminate,
    spl_token_2022::{
        extension::StateWithExtensions,
        state::{Account, Mint},
    },
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
    let escrow_authority_info = next_account_info(accounts_iter)?;
    let escrow_token_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let _token_program_info = next_account_info(accounts_iter)?;

    // Note that Token-2022's `TransferChecked` processor will assert the
    // following:
    //
    // * The provided owner account is a signer.
    // * The provided owner is authorized to transfer the tokens.
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
    if &lockup_info.try_borrow_data()?[0..8] == Lockup::SPL_DISCRIMINATOR_SLICE {
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
        .eq(&get_escrow_token_account_address(program_id, mint_info.key))
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
            mint_info.key,
        );

    // Transfer the tokens to the escrow token account.
    {
        let decimals = {
            let mint_data = mint_info.try_borrow_data()?;
            let mint = StateWithExtensions::<Mint>::unpack(&mint_data)?;
            mint.base.decimals
        };

        invoke(
            &spl_token_2022::instruction::transfer_checked(
                &spl_token_2022::id(),
                token_account_info.key,
                mint_info.key,
                escrow_token_account_info.key,
                owner_info.key,
                &[],
                amount,
                decimals,
            )?,
            &[
                token_account_info.clone(),
                mint_info.clone(),
                escrow_token_account_info.clone(),
                owner_info.clone(),
            ],
        )?;
    }

    Ok(())
}

/// Processes an
/// [Unlock](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_unlock(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let owner_info = next_account_info(accounts_iter)?;
    let token_account_info = next_account_info(accounts_iter)?;
    let lockup_info = next_account_info(accounts_iter)?;

    // Ensure the owner account is a signer.
    if !owner_info.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Ensure the provided owner is the owner of the depositor token account.
    {
        let token_account_data = token_account_info.try_borrow_data()?;
        let token_account = StateWithExtensions::<Account>::unpack(&token_account_data)?;
        if &token_account.base.owner != owner_info.key {
            return Err(ProgramError::IncorrectAuthority);
        }
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

    // Ensure the provided depositor account is the same as the lockup's
    // depositor.
    if state.depositor != *token_account_info.key {
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

    let token_account_info = next_account_info(accounts_iter)?;
    let lockup_info = next_account_info(accounts_iter)?;
    let escrow_authority_info = next_account_info(accounts_iter)?;
    let escrow_token_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let _token_program_info = next_account_info(accounts_iter)?;

    // Note that Token-2022's `TransferChecked` processor will assert the
    // provided token account is for the provided mint.

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
        .eq(&get_escrow_token_account_address(program_id, mint_info.key))
    {
        return Err(PaladinLockupError::IncorrectEscrowTokenAccount.into());
    }

    let withdraw_amount = {
        let data = lockup_info.try_borrow_data()?;
        let state = bytemuck::try_from_bytes::<Lockup>(&data)
            .map_err(|_| ProgramError::InvalidAccountData)?;

        // Ensure the provided depositor account is the same as the lockup's
        // depositor.
        if state.depositor != *token_account_info.key {
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

        invoke_signed(
            &spl_token_2022::instruction::transfer_checked(
                &spl_token_2022::id(),
                escrow_token_account_info.key,
                mint_info.key,
                token_account_info.key,
                escrow_authority_info.key,
                &[],
                withdraw_amount,
                decimals,
            )?,
            &[
                escrow_token_account_info.clone(),
                mint_info.clone(),
                token_account_info.clone(),
                escrow_authority_info.clone(),
            ],
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

/// Processes an
/// [InitializeEscrow](enum.PaladinLockupInstruction.html)
/// instruction.
fn process_initialize_escrow(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let escrow_authority_info = next_account_info(accounts_iter)?;
    let escrow_token_account_info = next_account_info(accounts_iter)?;
    let mint_info = next_account_info(accounts_iter)?;
    let _token_program_info = next_account_info(accounts_iter)?;

    let (escrow_authority_address, bump_seed) =
        get_escrow_authority_address_and_bump_seed(program_id);
    let bump_seed = [bump_seed];
    let escrow_authority_signer_seeds = collect_escrow_authority_signer_seeds(&bump_seed);

    // Ensure the provided escrow authority address is correct.
    if !escrow_authority_info.key.eq(&escrow_authority_address) {
        return Err(PaladinLockupError::IncorrectEscrowAuthorityAddress.into());
    }

    // Assign the escrow authority (no state to allocate).
    invoke_signed(
        &system_instruction::assign(escrow_authority_info.key, program_id),
        &[escrow_authority_info.clone()],
        &[&escrow_authority_signer_seeds],
    )?;

    let (escrow_token_account_address, bump_seed) =
        get_escrow_token_account_address_and_bump_seed(program_id, mint_info.key);
    let bump_seed = [bump_seed];
    let escrow_token_account_signer_seeds =
        collect_escrow_token_account_signer_seeds(mint_info.key, &bump_seed);

    // Ensure the provided escrow token account address is correct.
    if !escrow_token_account_info
        .key
        .eq(&escrow_token_account_address)
    {
        return Err(PaladinLockupError::IncorrectEscrowTokenAccount.into());
    }

    // Allocate & assign the escrow token account.
    invoke_signed(
        &system_instruction::allocate(&escrow_token_account_address, Account::LEN as u64),
        &[escrow_token_account_info.clone()],
        &[&escrow_token_account_signer_seeds],
    )?;
    invoke_signed(
        &system_instruction::assign(&escrow_token_account_address, &spl_token_2022::id()),
        &[escrow_token_account_info.clone()],
        &[&escrow_token_account_signer_seeds],
    )?;

    // Create the escrow token account.
    invoke(
        &spl_token_2022::instruction::initialize_account3(
            &spl_token_2022::id(),
            escrow_token_account_info.key,
            mint_info.key,
            escrow_authority_info.key,
        )?,
        &[escrow_token_account_info.clone(), mint_info.clone()],
    )?;

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
