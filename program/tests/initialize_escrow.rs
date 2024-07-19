#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::{
        error::PaladinLockupError,
        instruction::initialize_escrow,
        state::{get_escrow_authority_address, get_escrow_token_account_address},
    },
    setup::{setup, setup_mint},
    solana_program_test::*,
    solana_sdk::{
        account::AccountSharedData,
        instruction::InstructionError,
        program_pack::Pack,
        pubkey::Pubkey,
        signer::Signer,
        system_program,
        transaction::{Transaction, TransactionError},
    },
    spl_token_2022::{extension::StateWithExtensions, state::Account as TokenAccount},
};

#[tokio::test]
async fn fail_incorrect_escrow_authority_address() {
    let mint = Pubkey::new_unique();

    let mut context = setup().start_with_context().await;

    let mut instruction = initialize_escrow(&mint);
    instruction.accounts[0].pubkey = Pubkey::new_unique(); // Incorrect escrow authority address.

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

    let mut context = setup().start_with_context().await;

    let mut instruction = initialize_escrow(&mint);
    instruction.accounts[1].pubkey = Pubkey::new_unique(); // Incorrect escrow token account address.

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
async fn success() {
    let mint = Pubkey::new_unique();

    let escrow_authority = get_escrow_authority_address(&paladin_lockup_program::id());
    let escrow_token_account =
        get_escrow_token_account_address(&paladin_lockup_program::id(), &mint);

    let mut context = setup().start_with_context().await;
    setup_mint(&mut context, &mint, &Pubkey::new_unique(), 1_000_000).await;

    // Fund/allocate the escrow account and escrow token account.
    {
        let rent = context.banks_client.get_rent().await.unwrap();
        let lamports = rent.minimum_balance(0);
        context.set_account(
            &escrow_authority,
            &AccountSharedData::new(lamports, 0, &system_program::id()),
        );
        let lamports = rent.minimum_balance(TokenAccount::LEN);
        context.set_account(
            &escrow_token_account,
            &AccountSharedData::new(lamports, 0, &system_program::id()),
        );
    }

    let instruction = initialize_escrow(&mint);

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

    // Check the escrow authority account.
    let escrow_authority_account = context
        .banks_client
        .get_account(escrow_authority)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(escrow_authority_account.owner, paladin_lockup_program::id());

    // Check the escrow token account.
    let escrow_token_account = context
        .banks_client
        .get_account(escrow_token_account)
        .await
        .unwrap()
        .unwrap();
    let state = StateWithExtensions::<TokenAccount>::unpack(&escrow_token_account.data).unwrap();
    assert_eq!(state.base.mint, mint);
    assert_eq!(state.base.owner, escrow_authority);
}
