#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::{
        error::PaladinLockupError,
        state::{get_escrow_authority_address, Lockup},
    },
    setup::{setup, setup_mint, setup_token_account},
    solana_program_test::*,
    solana_sdk::{
        account::{Account, AccountSharedData},
        clock::Clock,
        instruction::InstructionError,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::{Transaction, TransactionError},
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
    spl_discriminator::SplDiscriminate,
    spl_token_2022::{extension::StateWithExtensions, state::Account as TokenAccount},
    test_case::test_case,
};

#[tokio::test]
async fn fail_incorrect_lockup_owner() {
    let lockup_authority = Keypair::new();
    let mint = Pubkey::new_unique();

    let token_owner = Keypair::new();
    let token_account = get_associated_token_address_with_program_id(
        &token_owner.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &token_owner.pubkey(),
        &mint,
        10_000,
    )
    .await;

    // Create the lockup account with the incorrect owner.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>();
        let lamports = rent.minimum_balance(space);
        context.set_account(
            &lockup,
            &AccountSharedData::new(lamports, space, &Pubkey::new_unique()), // Incorrect owner.
        );
    }

    let instruction = paladin_lockup_program::instruction::lockup(
        &lockup_authority.pubkey(),
        &token_owner.pubkey(),
        &token_account,
        &lockup,
        &mint,
        10_000,
        &spl_token_2022::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_owner],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(0, InstructionError::InvalidAccountOwner)
    );
}

#[tokio::test]
async fn fail_lockup_not_enough_space() {
    let lockup_authority = Keypair::new();
    let mint = Pubkey::new_unique();

    let token_owner = Keypair::new();
    let token_account = get_associated_token_address_with_program_id(
        &token_owner.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &token_owner.pubkey(),
        &mint,
        10_000,
    )
    .await;

    // Create the lockup account with not enough space.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>().saturating_sub(6); // Not enough space.
        let lamports = rent.minimum_balance(space);
        context.set_account(
            &lockup,
            &AccountSharedData::new(lamports, space, &paladin_lockup_program::id()),
        );
    }

    let instruction = paladin_lockup_program::instruction::lockup(
        &lockup_authority.pubkey(),
        &token_owner.pubkey(),
        &token_account,
        &lockup,
        &mint,
        10_000,
        &spl_token_2022::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_owner],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(0, InstructionError::InvalidAccountData)
    );
}

#[tokio::test]
async fn fail_lockup_already_initialized() {
    let lockup_authority = Keypair::new();
    let mint = Pubkey::new_unique();

    let token_owner = Keypair::new();
    let token_account = get_associated_token_address_with_program_id(
        &token_owner.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &token_owner.pubkey(),
        &mint,
        10_000,
    )
    .await;

    // Create the lockup account with initialized state.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>(); // Not enough space.
        let lamports = rent.minimum_balance(space);

        let mut data = vec![0u8; space];
        data[..8].copy_from_slice(&Lockup::SPL_DISCRIMINATOR_SLICE);

        let account = Account {
            lamports,
            data,
            owner: paladin_lockup_program::id(),
            ..Default::default()
        };

        context.set_account(&lockup, &AccountSharedData::from(account));
    }

    let instruction = paladin_lockup_program::instruction::lockup(
        &lockup_authority.pubkey(),
        &token_owner.pubkey(),
        &token_account,
        &lockup,
        &mint,
        10_000,
        &spl_token_2022::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_owner],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(0, InstructionError::AccountAlreadyInitialized)
    );
}

#[tokio::test]
async fn fail_incorrect_escrow_authority_address() {
    let lockup_authority = Keypair::new();
    let mint = Pubkey::new_unique();

    let token_owner = Keypair::new();
    let token_account = get_associated_token_address_with_program_id(
        &token_owner.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &token_owner.pubkey(),
        &mint,
        10_000,
    )
    .await;

    // Set up the lockup account correctly.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>();
        let lamports = rent.minimum_balance(space);

        context.set_account(
            &lockup,
            &AccountSharedData::new(lamports, space, &paladin_lockup_program::id()),
        );
    }

    let mut instruction = paladin_lockup_program::instruction::lockup(
        &lockup_authority.pubkey(),
        &token_owner.pubkey(),
        &token_account,
        &lockup,
        &mint,
        10_000,
        &spl_token_2022::id(),
    );
    instruction.accounts[4].pubkey = Pubkey::new_unique(); // Incorrect escrow authority address.

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_owner],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(PaladinLockupError::IncorrectEscrowAuthorityAddress as u32)
        )
    );
}

#[tokio::test]
async fn fail_incorrect_escrow_token_account_address() {
    let lockup_authority = Keypair::new();
    let mint = Pubkey::new_unique();

    let token_owner = Keypair::new();
    let token_account = get_associated_token_address_with_program_id(
        &token_owner.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &token_owner.pubkey(),
        &mint,
        10_000,
    )
    .await;

    // Set up the lockup account correctly.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>();
        let lamports = rent.minimum_balance(space);

        context.set_account(
            &lockup,
            &AccountSharedData::new(lamports, space, &paladin_lockup_program::id()),
        );
    }

    let mut instruction = paladin_lockup_program::instruction::lockup(
        &lockup_authority.pubkey(),
        &token_owner.pubkey(),
        &token_account,
        &lockup,
        &mint,
        10_000,
        &spl_token_2022::id(),
    );
    instruction.accounts[5].pubkey = Pubkey::new_unique(); // Incorrect escrow token account address.

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_owner],
        context.last_blockhash,
    );

    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();

    assert_eq!(
        err,
        TransactionError::InstructionError(
            0,
            InstructionError::Custom(PaladinLockupError::IncorrectEscrowTokenAccount as u32)
        )
    );
}

async fn check_token_account_balance(
    context: &mut ProgramTestContext,
    token_account_address: &Pubkey,
    check_amount: u64,
) {
    let account = context
        .banks_client
        .get_account(*token_account_address)
        .await
        .expect("get_account")
        .expect("account not found");
    let actual_amount = StateWithExtensions::<TokenAccount>::unpack(&account.data)
        .unwrap()
        .base
        .amount;
    assert_eq!(actual_amount, check_amount);
}

#[test_case(1)]
#[test_case(1_000_000_000)]
#[test_case(1_000_000_000_000_000)]
#[tokio::test]
async fn success(amount: u64) {
    let lockup_authority = Keypair::new();
    let mint = Pubkey::new_unique();

    let token_owner = Keypair::new();
    let token_account = get_associated_token_address_with_program_id(
        &token_owner.pubkey(),
        &mint,
        &spl_token_2022::id(),
    );
    let token_account_starting_token_balance = amount;

    let escrow_authority = get_escrow_authority_address(&paladin_lockup_program::id());
    let escrow_token_account = get_associated_token_address_with_program_id(
        &escrow_authority,
        &mint,
        &spl_token_2022::id(),
    );

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &token_owner.pubkey(),
        &mint,
        token_account_starting_token_balance,
    )
    .await;
    setup_token_account(
        &mut context,
        &escrow_token_account,
        &escrow_authority,
        &mint,
        0,
    )
    .await;
    setup_mint(&mut context, &mint, &Pubkey::new_unique(), 1_000_000).await;

    // Set up the lockup account correctly.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>();
        let lamports = rent.minimum_balance(space);

        context.set_account(
            &lockup,
            &AccountSharedData::new(lamports, space, &paladin_lockup_program::id()),
        );
    }

    let instruction = paladin_lockup_program::instruction::lockup(
        &lockup_authority.pubkey(),
        &token_owner.pubkey(),
        &token_account,
        &lockup,
        &mint,
        amount,
        &spl_token_2022::id(),
    );

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &token_owner],
        context.last_blockhash,
    );

    // For checks later.
    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Check the lockup account.
    let lockup_account = context
        .banks_client
        .get_account(lockup)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        bytemuck::from_bytes::<Lockup>(&lockup_account.data),
        &Lockup::new(
            amount,
            &lockup_authority.pubkey(),
            clock.unix_timestamp as u64,
            &mint,
        )
    );

    // Validate tokens were transferred from the token account to the escrow.
    check_token_account_balance(
        &mut context,
        &token_account,
        token_account_starting_token_balance.saturating_sub(amount),
    )
    .await;
    check_token_account_balance(&mut context, &escrow_token_account, amount).await;
}
