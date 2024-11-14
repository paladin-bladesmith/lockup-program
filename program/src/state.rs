use {
    bytemuck::{Pod, Zeroable},
    shank::ShankAccount,
    solana_program::pubkey::Pubkey,
    spl_discriminator::SplDiscriminate,
    std::num::NonZeroU64,
};

/// The seed prefix (`"escrow_authority"`) in bytes used to derive the address
/// of the Paladin Lockup program's escrow authority.
/// Seeds: `"escrow_authority"`.
pub const SEED_PREFIX_ESCROW_AUTHORITY: &[u8] = b"escrow_authority";

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

/// Lockup pool account.
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, SplDiscriminate, Zeroable)]
#[discriminator_hash_input("lockup::state::lockup_pool")]
#[repr(C)]
pub struct LockupPool {
    pub discriminator: [u8; 8],
    pub entries: [LockupPoolEntry; 256],
    pub entries_len: usize,
}

impl LockupPool {
    pub const LEN: usize = std::mem::size_of::<LockupPool>();
}

/// Lockup entry in the lockup pool.
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable)]
#[repr(C)]
pub struct LockupPoolEntry {
    pub lockup: Pubkey,
    pub amount: u64,
}

/// A lockup account.
#[derive(Clone, Copy, Debug, PartialEq, Pod, ShankAccount, SplDiscriminate, Zeroable)]
#[discriminator_hash_input("lockup::state::lockup")]
#[repr(C)]
pub struct Lockup {
    pub discriminator: [u8; 8],
    /// Amount of tokens locked up in the escrow.
    pub amount: u64,
    /// The lockup's authority.
    pub authority: Pubkey,
    /// The start of the lockup period.
    pub lockup_start_timestamp: u64,
    /// The end of the lockup period.
    pub lockup_end_timestamp: Option<NonZeroU64>,
    /// The address of the mint this lockup supports.
    pub mint: Pubkey,
    /// The pool this lockup participates in.
    ///
    /// # Note
    ///
    /// Pools enable storing top lockups for easy off-chain lookup.
    pub pool: Pubkey,
    /// Address of any additional metadata (stored in another account).
    pub metadata: Pubkey,
}

impl Lockup {
    pub const LEN: usize = std::mem::size_of::<Lockup>();
}
