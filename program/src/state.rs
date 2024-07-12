/// A lockup account.
pub struct Lockup {
    /// Amount of tokens locked up in the escrow.
    pub amount: u64,
    /// The start of the lockup period.
    pub lockup_start_timestamp: u64,
    /// The end of the lockup period.
    pub lockup_end_timestamp: u64,
}
