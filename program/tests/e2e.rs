//! End-to-end test.

#![cfg(feature = "test-sbf")]

mod setup;

use {
    paladin_lockup_program::{
        error::PaladinLockupError,
        state::{get_escrow_authority_address, get_escrow_token_account_address, Lockup},
    },
    setup::{setup, setup_mint, setup_token_account},
    solana_program_test::*,
    solana_sdk::{
        clock::Clock,
        instruction::{Instruction, InstructionError},
        pubkey::Pubkey,
        signature::Keypair,
        signer::Signer,
        system_instruction,
        transaction::{Transaction, TransactionError},
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
    spl_token_2022::{extension::StateWithExtensions, state::Account as TokenAccount},
};

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

async fn check_lockup_state(
    context: &mut ProgramTestContext,
    lockup_address: &Pubkey,
    check_lockup: &Lockup,
) {
    let account = context
        .banks_client
        .get_account(*lockup_address)
        .await
        .expect("get_account")
        .expect("account not found");
    let actual_lockup = bytemuck::from_bytes::<Lockup>(&account.data);
    assert_eq!(actual_lockup, check_lockup);
}

async fn send_transaction(
    context: &mut ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
) {
    let blockhash = context.banks_client.get_latest_blockhash().await.unwrap();
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&context.payer.pubkey()),
        signers,
        blockhash,
    );
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}

async fn send_transaction_with_expected_err(
    context: &mut ProgramTestContext,
    instructions: &[Instruction],
    signers: &[&Keypair],
    expected_err: TransactionError,
) {
    let transaction = Transaction::new_signed_with_payer(
        instructions,
        Some(&context.payer.pubkey()),
        signers,
        context.last_blockhash,
    );
    let err = context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap_err()
        .unwrap();
    assert_eq!(err, expected_err);
}

#[tokio::test]
async fn test_e2e() {
    let mint = Pubkey::new_unique();

    let alice = Keypair::new();
    let alice_token_account =
        get_associated_token_address_with_program_id(&alice.pubkey(), &mint, &spl_token_2022::id());
    let alice_token_account_starting_token_balance = 10_000;

    let bob = Keypair::new();
    let bob_token_account =
        get_associated_token_address_with_program_id(&bob.pubkey(), &mint, &spl_token_2022::id());
    let bob_token_account_starting_token_balance = 10_000;

    let escrow_authority = get_escrow_authority_address(&paladin_lockup_program::id());
    let escrow_token_account =
        get_escrow_token_account_address(&paladin_lockup_program::id(), &mint);

    let mut context = setup().start_with_context().await;
    let payer = context.payer.insecure_clone();

    // Setup.
    {
        setup_token_account(
            &mut context,
            &alice_token_account,
            &alice.pubkey(),
            &mint,
            alice_token_account_starting_token_balance,
        )
        .await;
        setup_token_account(
            &mut context,
            &bob_token_account,
            &bob.pubkey(),
            &mint,
            bob_token_account_starting_token_balance,
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
    }

    // Create a lockup for Alice.
    let alice_lockup = Keypair::new();
    let alice_lockup_amount = 1_000;
    let alice_lockup_period_seconds = 60;
    {
        let clock = context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .expect("get_sysvar");

        let rent = context.banks_client.get_rent().await.expect("get_rent");
        let space = std::mem::size_of::<Lockup>();

        send_transaction(
            &mut context,
            &[
                system_instruction::transfer(
                    &payer.pubkey(),
                    &alice_lockup.pubkey(),
                    rent.minimum_balance(space),
                ),
                system_instruction::allocate(&alice_lockup.pubkey(), space as u64),
                system_instruction::assign(&alice_lockup.pubkey(), &paladin_lockup_program::id()),
                paladin_lockup_program::instruction::lockup(
                    &alice.pubkey(),
                    &alice_token_account,
                    &alice_lockup.pubkey(),
                    &mint,
                    alice_lockup_amount,
                    alice_lockup_period_seconds,
                ),
            ],
            &[&payer, &alice, &alice_lockup],
        )
        .await;

        let expected_lockup_start = clock.unix_timestamp as u64;
        let expected_lockup_end = expected_lockup_start.saturating_add(alice_lockup_period_seconds);

        // Validate the lockup was created and tokens were transferred to the escrow.
        check_lockup_state(
            &mut context,
            &alice_lockup.pubkey(),
            &Lockup::new(
                alice_lockup_amount,
                &alice.pubkey(),
                expected_lockup_start,
                expected_lockup_end,
                &mint,
            ),
        )
        .await;
        check_token_account_balance(
            &mut context,
            &alice_token_account,
            alice_token_account_starting_token_balance.saturating_sub(alice_lockup_amount),
        )
        .await;
        check_token_account_balance(&mut context, &escrow_token_account, alice_lockup_amount).await;
    }

    // Warp the clock 30 seconds.
    // Alice can't withdraw until the period ends.
    {
        let mut clock = context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .expect("get_sysvar");
        clock.unix_timestamp = clock.unix_timestamp.saturating_add(30);
        context.set_sysvar(&clock);

        send_transaction_with_expected_err(
            &mut context,
            &[paladin_lockup_program::instruction::withdraw(
                &alice.pubkey(),
                &alice_token_account,
                &alice_lockup.pubkey(),
                &mint,
            )],
            &[&payer, &alice],
            TransactionError::InstructionError(
                0,
                InstructionError::Custom(PaladinLockupError::LockupActive as u32),
            ),
        )
        .await;
    }

    // Warp the clock 30 more seconds.
    // Alice can now withdraw.
    {
        let mut clock = context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .expect("get_sysvar");
        clock.unix_timestamp = clock.unix_timestamp.saturating_add(30);
        context.set_sysvar(&clock);

        send_transaction(
            &mut context,
            &[paladin_lockup_program::instruction::withdraw(
                &alice.pubkey(),
                &alice_token_account,
                &alice_lockup.pubkey(),
                &mint,
            )],
            &[&payer, &alice],
        )
        .await;

        // Validate the lockup was closed and tokens were transferred back to Alice.
        assert!(context
            .banks_client
            .get_account(alice_lockup.pubkey())
            .await
            .expect("get_account")
            .is_none());
        check_token_account_balance(
            &mut context,
            &alice_token_account,
            alice_token_account_starting_token_balance,
        )
        .await;
        check_token_account_balance(&mut context, &escrow_token_account, 0).await;
    }

    // Create a lockup for Bob.
    let bob_lockup = Keypair::new();
    let bob_lockup_amount = 2_000;
    let bob_lockup_period_seconds = 120;
    {
        let clock = context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .expect("get_sysvar");

        let rent = context.banks_client.get_rent().await.expect("get_rent");
        let space = std::mem::size_of::<Lockup>();

        send_transaction(
            &mut context,
            &[
                system_instruction::transfer(
                    &payer.pubkey(),
                    &bob_lockup.pubkey(),
                    rent.minimum_balance(space),
                ),
                system_instruction::allocate(&bob_lockup.pubkey(), space as u64),
                system_instruction::assign(&bob_lockup.pubkey(), &paladin_lockup_program::id()),
                paladin_lockup_program::instruction::lockup(
                    &bob.pubkey(),
                    &bob_token_account,
                    &bob_lockup.pubkey(),
                    &mint,
                    bob_lockup_amount,
                    bob_lockup_period_seconds,
                ),
            ],
            &[&payer, &bob, &bob_lockup],
        )
        .await;

        let expected_lockup_start = clock.unix_timestamp as u64;
        let expected_lockup_end = expected_lockup_start.saturating_add(bob_lockup_period_seconds);

        // Validate the lockup was created and tokens were transferred to the escrow.
        check_lockup_state(
            &mut context,
            &bob_lockup.pubkey(),
            &Lockup::new(
                bob_lockup_amount,
                &bob.pubkey(),
                expected_lockup_start,
                expected_lockup_end,
                &mint,
            ),
        )
        .await;
        check_token_account_balance(
            &mut context,
            &bob_token_account,
            bob_token_account_starting_token_balance.saturating_sub(bob_lockup_amount),
        )
        .await;
        check_token_account_balance(&mut context, &escrow_token_account, bob_lockup_amount).await;
    }

    // Warp the clock 60 seconds.
    // Bob can't withdraw until the period ends.
    {
        let mut clock = context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .expect("get_sysvar");
        clock.unix_timestamp = clock.unix_timestamp.saturating_add(60);
        context.set_sysvar(&clock);

        send_transaction_with_expected_err(
            &mut context,
            &[paladin_lockup_program::instruction::withdraw(
                &bob.pubkey(),
                &bob_token_account,
                &bob_lockup.pubkey(),
                &mint,
            )],
            &[&payer, &bob],
            TransactionError::InstructionError(
                0,
                InstructionError::Custom(PaladinLockupError::LockupActive as u32),
            ),
        )
        .await;
    }

    // Bob unlocks the tokens.
    {
        let clock = context
            .banks_client
            .get_sysvar::<Clock>()
            .await
            .expect("get_sysvar");

        send_transaction(
            &mut context,
            &[paladin_lockup_program::instruction::unlock(
                &bob.pubkey(),
                &bob_lockup.pubkey(),
            )],
            &[&payer, &bob],
        )
        .await;

        let expected_lockup_start = (clock.unix_timestamp as u64).saturating_sub(60);
        let expected_lockup_end = clock.unix_timestamp as u64; // Now.

        // Validate the lockup was unlocked.
        check_lockup_state(
            &mut context,
            &bob_lockup.pubkey(),
            &Lockup::new(
                bob_lockup_amount,
                &bob.pubkey(),
                expected_lockup_start,
                expected_lockup_end,
                &mint,
            ),
        )
        .await;
    }

    // Bob can now withdraw tokens.
    {
        send_transaction(
            &mut context,
            &[paladin_lockup_program::instruction::withdraw(
                &bob.pubkey(),
                &bob_token_account,
                &bob_lockup.pubkey(),
                &mint,
            )],
            &[&payer, &bob],
        )
        .await;

        // Validate the lockup was closed and tokens were transferred back to Bob.
        assert!(context
            .banks_client
            .get_account(bob_lockup.pubkey())
            .await
            .expect("get_account")
            .is_none());
        check_token_account_balance(
            &mut context,
            &bob_token_account,
            bob_token_account_starting_token_balance,
        )
        .await;
        check_token_account_balance(&mut context, &escrow_token_account, 0).await;
    }
}
