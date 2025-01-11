#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::state::LockupPool,
    setup::setup,
    solana_program_test::*,
    solana_sdk::{
        rent::Rent, signature::Keypair, signer::Signer, system_instruction,
        transaction::Transaction,
    },
};

#[tokio::test]
async fn err_duplicate_initialize() {
    let mut context = setup().start_with_context().await;
    let pool = Keypair::new();

    let rent = Rent::default().minimum_balance(LockupPool::LEN);
    let fund = system_instruction::transfer(&context.payer.pubkey(), &pool.pubkey(), rent);
    let allocate = system_instruction::allocate(&pool.pubkey(), LockupPool::LEN as u64);
    let initialize_lockup_pool =
        paladin_lockup_program::instruction::initialize_lockup_pool(pool.pubkey());
    let tx = Transaction::new_signed_with_payer(
        &[fund, allocate, initialize_lockup_pool],
        Some(&pool.pubkey()),
        &[&context.payer, &pool],
        context.last_blockhash,
    );

    println!("B");
    let res = context.banks_client.process_transaction(tx).await;
    println!("A");

    println!("{res:?}");
}
