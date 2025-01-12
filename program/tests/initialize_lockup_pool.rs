#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::state::LockupPool,
    setup::setup,
    solana_program_test::*,
    solana_sdk::{
        instruction::InstructionError,
        rent::Rent,
        signature::Keypair,
        signer::Signer,
        system_instruction,
        transaction::{Transaction, TransactionError},
    },
};

#[tokio::test]
async fn err_duplicate_initialize() {
    let mut context = setup().start_with_context().await;
    let pool = Keypair::new();

    // Initialize the pool once.
    let rent = Rent::default().minimum_balance(LockupPool::LEN);
    let fund = system_instruction::transfer(&context.payer.pubkey(), &pool.pubkey(), rent);
    let allocate = system_instruction::allocate(&pool.pubkey(), LockupPool::LEN as u64);
    let assign = system_instruction::assign(&pool.pubkey(), &paladin_lockup_program::ID);
    let initialize_lockup_pool =
        paladin_lockup_program::instruction::initialize_lockup_pool(pool.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[fund, allocate, assign, initialize_lockup_pool.clone()],
        Some(&context.payer.pubkey()),
        &[&context.payer, &pool],
        context.last_blockhash,
    );
    context.banks_client.process_transaction(tx).await.unwrap();

    // Attempt to initialize the pool again.
    let tx = Transaction::new_signed_with_payer(
        &[initialize_lockup_pool],
        Some(&context.payer.pubkey()),
        &[&context.payer],
        context.last_blockhash,
    );
    let err = context
        .banks_client
        .process_transaction(tx)
        .await
        .unwrap_err()
        .unwrap();

    // Initialize should panic.
    assert_eq!(
        err,
        TransactionError::InstructionError(0, InstructionError::ProgramFailedToComplete)
    );
}
