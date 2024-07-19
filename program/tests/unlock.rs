#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::state::Lockup,
    setup::{setup, setup_lockup, setup_token_account},
    solana_program_test::*,
    solana_sdk::{
        account::AccountSharedData,
        clock::Clock,
        instruction::InstructionError,
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        transaction::{Transaction, TransactionError},
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
};

#[tokio::test]
async fn fail_owner_not_signer() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let mut instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);
    instruction.accounts[0].is_signer = false;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer], // Owner not signer.
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
        TransactionError::InstructionError(0, InstructionError::MissingRequiredSignature)
    );
}

#[tokio::test]
async fn fail_wrong_owner() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &Pubkey::new_unique(), // Wrong owner.
        &mint,
        10_000,
    )
    .await;

    let instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &owner],
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
        TransactionError::InstructionError(0, InstructionError::IncorrectAuthority)
    );
}

#[tokio::test]
async fn fail_incorrect_lockup_owner() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;

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

    let instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &owner],
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
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;

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

    let instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &owner],
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
        TransactionError::InstructionError(0, InstructionError::UninitializedAccount)
    );
}

#[tokio::test]
async fn fail_lockup_uninitialized() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;

    // Create the lockup account with uninitialized state.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let space = std::mem::size_of::<Lockup>(); // Not enough space.
        let lamports = rent.minimum_balance(space);
        context.set_account(
            &lockup,
            &AccountSharedData::new(lamports, space, &paladin_lockup_program::id()),
        );
    }

    let instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &owner],
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
        TransactionError::InstructionError(0, InstructionError::UninitializedAccount)
    );
}

#[tokio::test]
async fn fail_incorrect_depositor() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_lockup(
        &mut context,
        &lockup,
        &Pubkey::new_unique(), // Incorrect depositor.
        10_000,
        10_000,
        10_000,
        &mint,
    )
    .await;

    let instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &owner],
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
        TransactionError::InstructionError(0, InstructionError::IncorrectAuthority)
    );
}

#[tokio::test]
async fn success() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();
    let start = clock.unix_timestamp as u64;
    let end = clock.unix_timestamp.saturating_add(10_000) as u64;

    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_lockup(
        &mut context,
        &lockup,
        &token_account,
        10_000, // Amount (unused here).
        start,
        end,
        &mint,
    )
    .await;

    let instruction =
        paladin_lockup_program::instruction::unlock(&owner.pubkey(), &token_account, &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &owner],
        context.last_blockhash,
    );

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
    let state = bytemuck::from_bytes::<Lockup>(&lockup_account.data);
    assert_eq!(state.lockup_end_timestamp, start); // Unlocked in current
                                                   // timestamp.
}
