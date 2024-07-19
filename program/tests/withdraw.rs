#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::{
        error::PaladinLockupError,
        state::{get_escrow_authority_address, get_escrow_token_account_address, Lockup},
    },
    setup::{setup, setup_escrow_authority, setup_lockup, setup_mint, setup_token_account},
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
    spl_token_2022::{extension::StateWithExtensions, state::Account as TokenAccount},
};

#[tokio::test]
async fn fail_incorrect_token_account_address() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;
    setup_token_account(
        &mut context,
        &token_account,
        &owner.pubkey(),
        &Pubkey::new_unique(), // Incorrect mint.
        10_000,
    )
    .await;

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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
            InstructionError::Custom(PaladinLockupError::TokenAccountMintMismatch as u32)
        )
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

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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
async fn fail_lockup_already_initialized() {
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

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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
async fn fail_incorrect_escrow_authority_address() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();

    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_lockup(
        &mut context,
        &lockup,
        &token_account,
        10_000,
        clock.unix_timestamp as u64,
        clock.unix_timestamp as u64, // Now (unlocked).
    )
    .await;

    let mut instruction =
        paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);
    instruction.accounts[2].pubkey = Pubkey::new_unique(); // Incorrect escrow authority address.

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();

    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_lockup(
        &mut context,
        &lockup,
        &token_account,
        10_000,
        clock.unix_timestamp as u64,
        clock.unix_timestamp as u64, // Now (unlocked).
    )
    .await;

    let mut instruction =
        paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);
    instruction.accounts[3].pubkey = Pubkey::new_unique(); // Incorrect escrow token account address.

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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

#[tokio::test]
async fn fail_lockup_still_active() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let lockup = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();

    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_lockup(
        &mut context,
        &lockup,
        &token_account,
        10_000,
        clock.unix_timestamp as u64,
        (clock.unix_timestamp as u64).saturating_add(1_000), // NOT unlocked.
    )
    .await;

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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
            InstructionError::Custom(PaladinLockupError::LockupActive as u32)
        )
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

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();

    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_lockup(
        &mut context,
        &lockup,
        &Pubkey::new_unique(), // Incorrect depositor.
        10_000,
        clock.unix_timestamp as u64,
        clock.unix_timestamp as u64, // Now (unlocked).
    )
    .await;

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
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

fn get_token_account_balance(token_account: &Account) -> u64 {
    StateWithExtensions::<TokenAccount>::unpack(&token_account.data)
        .unwrap()
        .base
        .amount
}

#[tokio::test]
async fn success() {
    let mint = Pubkey::new_unique();

    let owner = Keypair::new();
    let token_account =
        get_associated_token_address_with_program_id(&owner.pubkey(), &mint, &spl_token_2022::id());

    let escrow_authority = get_escrow_authority_address(&paladin_lockup_program::id());
    let escrow_token_account =
        get_escrow_token_account_address(&paladin_lockup_program::id(), &mint);

    let lockup = Pubkey::new_unique();

    let amount = 10_000;

    let mut context = setup().start_with_context().await;

    let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();

    setup_escrow_authority(&mut context, &escrow_authority).await;
    setup_lockup(
        &mut context,
        &lockup,
        &token_account,
        amount,
        clock.unix_timestamp as u64,
        clock.unix_timestamp as u64, // Now (unlocked).
    )
    .await;
    setup_token_account(&mut context, &token_account, &owner.pubkey(), &mint, 10_000).await;
    setup_token_account(
        &mut context,
        &escrow_token_account,
        &escrow_authority,
        &mint,
        10_000,
    )
    .await;
    setup_mint(&mut context, &mint, &Pubkey::new_unique(), 1_000_000).await;

    // For checks later.
    let token_account_start_balance = get_token_account_balance(
        &context
            .banks_client
            .get_account(token_account)
            .await
            .unwrap()
            .unwrap(),
    );
    let escrow_token_account_start_balance = get_token_account_balance(
        &context
            .banks_client
            .get_account(escrow_token_account)
            .await
            .unwrap()
            .unwrap(),
    );

    let instruction = paladin_lockup_program::instruction::withdraw(&token_account, &lockup, &mint);

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );

    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();

    // Check the resulting token account balances.
    let token_account_end_balance = get_token_account_balance(
        &context
            .banks_client
            .get_account(token_account)
            .await
            .unwrap()
            .unwrap(),
    );
    let escrow_token_account_end_balance = get_token_account_balance(
        &context
            .banks_client
            .get_account(escrow_token_account)
            .await
            .unwrap()
            .unwrap(),
    );

    assert_eq!(
        token_account_end_balance,
        token_account_start_balance.saturating_add(amount)
    );
    assert_eq!(
        escrow_token_account_end_balance,
        escrow_token_account_start_balance.saturating_sub(amount)
    );
}
