use {
    bytemuck::{Pod, Zeroable},
    solana_program::pubkey::Pubkey,
    spl_discriminator::SplDiscriminate,
};

/// The seed prefix (`"escrow_authority"`) in bytes used to derive the address
/// of the Paladin Lockup program's escrow authority.
/// Seeds: `"escrow_authority"`.
pub const SEED_PREFIX_ESCROW_AUTHORITY: &[u8] = b"escrow_authority";
/// The seed prefix (`"escrow_token_account"`) in bytes used to derive the
/// address of the Paladin Lockup program's escrow token account.
/// Seeds: `"escrow_token_account" + mint_address`.
pub const SEED_PREFIX_ESCROW_TOKEN_ACCOUNT: &[u8] = b"escrow_token_account";

/// Derive the address of the escrow authority.
pub fn get_escrow_authority_address(program_id: &Pubkey) -> Pubkey {
    get_escrow_authority_address_and_bump_seed(program_id).0
}

/// Derive the address of the escrow authority, with bump seed.
pub fn get_escrow_authority_address_and_bump_seed(program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&collect_escrow_authority_seeds(), program_id)
}

pub(crate) fn collect_escrow_authority_seeds<'a>() -> [&'a [u8]; 1] {
    [SEED_PREFIX_ESCROW_AUTHORITY]
}

pub(crate) fn collect_escrow_authority_signer_seeds(bump_seed: &[u8]) -> [&[u8]; 2] {
    [SEED_PREFIX_ESCROW_AUTHORITY, bump_seed]
}

/// Derive the address of the escrow token account.
pub fn get_escrow_token_account_address(program_id: &Pubkey, mint: &Pubkey) -> Pubkey {
    get_escrow_token_account_address_and_bump_seed(program_id, mint).0
}

/// Derive the address of the escrow token account, with bump seed.
pub fn get_escrow_token_account_address_and_bump_seed(
    program_id: &Pubkey,
    mint: &Pubkey,
) -> (Pubkey, u8) {
    Pubkey::find_program_address(&collect_escrow_token_account_seeds(mint), program_id)
}

pub(crate) fn collect_escrow_token_account_seeds(mint: &Pubkey) -> [&[u8]; 2] {
    [SEED_PREFIX_ESCROW_TOKEN_ACCOUNT, mint.as_ref()]
}

pub(crate) fn collect_escrow_token_account_signer_seeds<'a>(
    mint: &'a Pubkey,
    bump_seed: &'a [u8],
) -> [&'a [u8]; 3] {
    [SEED_PREFIX_ESCROW_TOKEN_ACCOUNT, mint.as_ref(), bump_seed]
}

/// A lockup account.
#[derive(Clone, Copy, Debug, PartialEq, Pod, SplDiscriminate, Zeroable)]
#[discriminator_hash_input("lockup::state::lockup")]
#[repr(C)]
pub struct Lockup {
    discriminator: [u8; 8],
    /// Amount of tokens locked up in the escrow.
    pub amount: u64,
    /// The depositor who created the lockup.
    pub depositor: Pubkey,
    /// The start of the lockup period.
    pub lockup_start_timestamp: u64,
    /// The end of the lockup period.
    pub lockup_end_timestamp: u64,
    /// The address of the mint this lockup supports.
    pub mint: Pubkey,
}

impl Lockup {
    /// Create a new lockup account.
    pub fn new(
        amount: u64,
        depositor: &Pubkey,
        lockup_start_timestamp: u64,
        lockup_end_timestamp: u64,
        mint: &Pubkey,
    ) -> Self {
        Self {
            discriminator: Self::SPL_DISCRIMINATOR.into(),
            amount,
            depositor: *depositor,
            lockup_start_timestamp,
            lockup_end_timestamp,
            mint: *mint,
        }
    }
}
