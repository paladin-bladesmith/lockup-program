#![cfg(feature = "test-sbf")]
#![allow(dead_code)]

use {
    paladin_lockup_program::state::Lockup,
    solana_program_test::*,
    solana_sdk::{
        account::{Account, AccountSharedData},
        clock::Clock,
        program_option::COption,
        pubkey::Pubkey,
        system_program,
    },
    spl_token_2022::{
        extension::{BaseStateWithExtensionsMut, ExtensionType, StateWithExtensionsMut},
        state::{Account as TokenAccount, AccountState, Mint},
    },
    std::num::NonZeroU64,
};

pub fn setup() -> ProgramTest {
    ProgramTest::new(
        "paladin_lockup_program",
        paladin_lockup_program::id(),
        processor!(paladin_lockup_program::processor::process),
    )
}

pub async fn setup_mint(
    context: &mut ProgramTestContext,
    mint: &Pubkey,
    mint_authority: &Pubkey,
    supply: u64,
) {
    let account_size = ExtensionType::try_calculate_account_len::<Mint>(&[]).unwrap();

    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(account_size);

    let mut data = vec![0; account_size];
    {
        let mut state = StateWithExtensionsMut::<Mint>::unpack_uninitialized(&mut data).unwrap();
        state.base = Mint {
            mint_authority: COption::Some(*mint_authority),
            is_initialized: true,
            supply,
            ..Mint::default()
        };
        state.pack_base();
        state.init_account_type().unwrap();
    }

    context.set_account(
        mint,
        &AccountSharedData::from(Account {
            lamports,
            data,
            owner: spl_token_2022::id(),
            ..Account::default()
        }),
    );
}

pub async fn setup_token_account(
    context: &mut ProgramTestContext,
    token_account: &Pubkey,
    owner: &Pubkey,
    mint: &Pubkey,
    amount: u64,
) {
    let account_size = ExtensionType::try_calculate_account_len::<TokenAccount>(&[]).unwrap();

    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(account_size);

    let mut data = vec![0; account_size];
    {
        let mut state =
            StateWithExtensionsMut::<TokenAccount>::unpack_uninitialized(&mut data).unwrap();
        state.base = TokenAccount {
            amount,
            mint: *mint,
            owner: *owner,
            state: AccountState::Initialized,
            ..TokenAccount::default()
        };
        state.pack_base();
        state.init_account_type().unwrap();
    }

    context.set_account(
        token_account,
        &AccountSharedData::from(Account {
            lamports,
            data,
            owner: spl_token_2022::id(),
            ..Account::default()
        }),
    );
}

#[allow(clippy::arithmetic_side_effects)]
pub async fn setup_system_account(
    context: &mut ProgramTestContext,
    address: &Pubkey,
    excess_lamports: u64,
) {
    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(0) + excess_lamports;

    context.set_account(
        address,
        &AccountSharedData::new(lamports, 0, &system_program::id()),
    );
}

pub async fn setup_lockup(
    context: &mut ProgramTestContext,
    address: &Pubkey,
    authority: &Pubkey,
    amount: u64,
    lockup_start_timestamp: u64,
    lockup_end_timestamp: Option<NonZeroU64>,
    mint_address: &Pubkey,
) {
    let mut state = Lockup::new(amount, authority, lockup_start_timestamp, mint_address);
    state.lockup_end_timestamp = lockup_end_timestamp;
    let data = bytemuck::bytes_of(&state).to_vec();

    let rent = context.banks_client.get_rent().await.unwrap();
    let lamports = rent.minimum_balance(data.len());

    context.set_account(
        address,
        &AccountSharedData::from(Account {
            lamports,
            data,
            owner: paladin_lockup_program::id(),
            ..Account::default()
        }),
    );
}

pub async fn add_seconds_to_clock(context: &mut ProgramTestContext, seconds: u64) {
    let mut clock = context
        .banks_client
        .get_sysvar::<Clock>()
        .await
        .expect("get_sysvar");
    clock.unix_timestamp = clock.unix_timestamp.saturating_add(seconds as i64);
    context.set_sysvar(&clock);
}
