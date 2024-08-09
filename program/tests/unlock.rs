#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::{error::PaladinLockupError, state::Lockup},
    setup::{setup, setup_lockup},
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
    std::num::NonZeroU64,
};

#[tokio::test]
async fn fail_lockup_authority_not_signer() {
    let authority = Keypair::new();
    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let mut instruction = paladin_lockup_program::instruction::unlock(&authority.pubkey(), &lockup);
    instruction.accounts[0].is_signer = false;

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer], // authority not signer.
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
async fn fail_lockup_not_enough_space() {
    let authority = Keypair::new();
    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

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

    let instruction = paladin_lockup_program::instruction::unlock(&authority.pubkey(), &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
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
    let authority = Keypair::new();
    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

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

    let instruction = paladin_lockup_program::instruction::unlock(&authority.pubkey(), &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
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
async fn fail_incorrect_lockup_authority() {
    let authority = Keypair::new();
    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_lockup(
        &mut context,
        &lockup,
        &Pubkey::new_unique(), // Incorrect authority.
        10_000,
        10_000,
        None,
        &Pubkey::new_unique(),
    )
    .await;

    let instruction = paladin_lockup_program::instruction::unlock(&authority.pubkey(), &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
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
async fn fail_lockup_already_unlocked() {
    let authority = Keypair::new();
    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    // Actual timestamp doesn't matter for this test.
    let start = 100_000u64;
    let end = 200_000u64;

    setup_lockup(
        &mut context,
        &lockup,
        &authority.pubkey(),
        10_000, // Amount (unused here).
        start,
        NonZeroU64::new(end), // Already unlocked.
        &Pubkey::new_unique(),
    )
    .await;

    let instruction = paladin_lockup_program::instruction::unlock(&authority.pubkey(), &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
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
            InstructionError::Custom(PaladinLockupError::LockupAlreadyUnlocked as u32)
        )
    );
}

#[tokio::test]
async fn success() {
    let authority = Keypair::new();
    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();
    let start = clock.unix_timestamp as u64;

    setup_lockup(
        &mut context,
        &lockup,
        &authority.pubkey(),
        10_000, // Amount (unused here).
        start,
        None,
        &Pubkey::new_unique(),
    )
    .await;

    let instruction = paladin_lockup_program::instruction::unlock(&authority.pubkey(), &lockup);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer, &authority],
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
    assert_eq!(state.lockup_end_timestamp.unwrap().get(), start);
}
