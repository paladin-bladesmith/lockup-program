//! Program instruction types.

/// Instructions supported by the Paladin Lockup program.
pub enum PaladinLockupInstruction {
    /// Lock up tokens in a lockup account for a specified period of time.
    ///
    /// Expects an uninitialized lockup account with enough rent-exempt
    /// lamports to store lockup state, owned by the Paladin Lockup program.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w, s]` Depositor account.
    /// 1. `[w]` Lockup account.
    Lockup { amount: u64, period_seconds: u64 },
    /// Unlock a token lockup, enabling the tokens for withdrawal.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w, s]` Depositor account.
    /// 1. `[w]` Lockup account.
    Unlock,
    /// Withdraw tokens from a lockup account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w, s]` Depositor account.
    /// 1. `[w]` Lockup account.
    Withdraw,
}
