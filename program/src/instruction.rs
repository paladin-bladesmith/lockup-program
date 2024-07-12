//! Program instruction types.

use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Instructions supported by the Paladin Lockup program.
#[derive(Clone, Copy, Debug, PartialEq)]
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

impl PaladinLockupInstruction {
    /// Packs a
    /// [PaladinLockupInstruction](enum.PaladinLockupInstruction.html)
    /// into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        match self {
            Self::Lockup {
                amount,
                period_seconds,
            } => {
                let mut buf = Vec::with_capacity(1 + 8 + 8);
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&period_seconds.to_le_bytes());
                buf
            }
            Self::Unlock => vec![1],
            Self::Withdraw => vec![2],
        }
    }

    /// Unpacks a byte buffer into a
    /// [PaladinLockupInstruction](enum.PaladinLockupInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        match input.split_first() {
            Some((&0, rest)) if rest.len() == 16 => {
                let amount = u64::from_le_bytes(rest[..8].try_into().unwrap());
                let period_seconds = u64::from_le_bytes(rest[8..].try_into().unwrap());
                Ok(Self::Lockup {
                    amount,
                    period_seconds,
                })
            }
            Some((&1, _)) => Ok(Self::Unlock),
            Some((&2, _)) => Ok(Self::Withdraw),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Creates a
/// [Lockup](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn lockup(
    depository_address: &Pubkey,
    lockup_address: &Pubkey,
    amount: u64,
    period_seconds: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*depository_address, true),
        AccountMeta::new(*lockup_address, false),
    ];
    let data = PaladinLockupInstruction::Lockup {
        amount,
        period_seconds,
    }
    .pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

/// Creates an
/// [Unlock](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn unlock(depository_address: &Pubkey, lockup_address: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*depository_address, true),
        AccountMeta::new(*lockup_address, false),
    ];
    let data = PaladinLockupInstruction::Unlock.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

/// Creates a
/// [Withdraw](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn withdraw(depository_address: &Pubkey, lockup_address: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*depository_address, true),
        AccountMeta::new(*lockup_address, false),
    ];
    let data = PaladinLockupInstruction::Withdraw.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_pack_unpack(instruction: PaladinLockupInstruction) {
        let packed = instruction.pack();
        let unpacked = PaladinLockupInstruction::unpack(&packed).unwrap();
        assert_eq!(instruction, unpacked);
    }

    #[test]
    fn test_pack_unpack_lockup() {
        test_pack_unpack(PaladinLockupInstruction::Lockup {
            amount: 42,
            period_seconds: 123,
        });
    }

    #[test]
    fn test_pack_unpack_unlock() {
        test_pack_unpack(PaladinLockupInstruction::Unlock);
    }

    #[test]
    fn test_pack_unpack_withdraw() {
        test_pack_unpack(PaladinLockupInstruction::Withdraw);
    }
}
