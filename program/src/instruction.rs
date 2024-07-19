//! Program instruction types.

use {
    crate::state::get_escrow_authority_address,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
    },
    spl_associated_token_account::get_associated_token_address_with_program_id,
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
    /// 0. `[s]` Lockup authority.
    /// 1. `[s]` Token owner.
    /// 2. `[w]` Depositor token account.
    /// 3. `[w]` Lockup account.
    /// 4. `[ ]` Escrow authority.
    /// 5. `[w]` Escrow token account.
    /// 6. `[ ]` Token mint.
    /// 7. `[ ]` Token program.
    Lockup { amount: u64, period_seconds: u64 },
    /// Unlock a token lockup, enabling the tokens for withdrawal.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[s]` Lockup authority.
    /// 1. `[w]` Lockup account.
    Unlock,
    /// Withdraw tokens from a lockup account.
    ///
    /// Note this instruction accepts a destination account for both lamports
    /// (from the closed lockup account's rent lamports) and tokens.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[s]` Lockup authority.
    /// 1. `[w]` Lamport destination.
    /// 2. `[w]` Token destination.
    /// 3. `[w]` Lockup account.
    /// 4. `[ ]` Escrow authority.
    /// 5. `[w]` Escrow token account.
    /// 6. `[ ]` Token mint.
    /// 7. `[ ]` Token program.
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
#[allow(clippy::too_many_arguments)]
pub fn lockup(
    lockup_authority_address: &Pubkey,
    token_owner_address: &Pubkey,
    token_account_address: &Pubkey,
    lockup_address: &Pubkey,
    mint_address: &Pubkey,
    amount: u64,
    period_seconds: u64,
    token_program_id: &Pubkey,
) -> Instruction {
    let escrow_authority_address = get_escrow_authority_address(&crate::id());
    let escrow_token_account_address = get_associated_token_address_with_program_id(
        &escrow_authority_address,
        mint_address,
        token_program_id,
    );
    let accounts = vec![
        AccountMeta::new_readonly(*lockup_authority_address, true),
        AccountMeta::new_readonly(*token_owner_address, true),
        AccountMeta::new(*token_account_address, false),
        AccountMeta::new(*lockup_address, false),
        AccountMeta::new_readonly(escrow_authority_address, false),
        AccountMeta::new(escrow_token_account_address, false),
        AccountMeta::new_readonly(*mint_address, false),
        AccountMeta::new_readonly(*token_program_id, false),
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
pub fn unlock(lockup_authority_address: &Pubkey, lockup_address: &Pubkey) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*lockup_authority_address, true),
        AccountMeta::new(*lockup_address, false),
    ];
    let data = PaladinLockupInstruction::Unlock.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

/// Creates a
/// [Withdraw](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn withdraw(
    lockup_authority_address: &Pubkey,
    lamport_destination_address: &Pubkey,
    token_destination_address: &Pubkey,
    lockup_address: &Pubkey,
    mint_address: &Pubkey,
    token_program_id: &Pubkey,
) -> Instruction {
    let escrow_authority_address = get_escrow_authority_address(&crate::id());
    let escrow_token_account_address = get_associated_token_address_with_program_id(
        &escrow_authority_address,
        mint_address,
        token_program_id,
    );
    let accounts = vec![
        AccountMeta::new_readonly(*lockup_authority_address, true),
        AccountMeta::new(*lamport_destination_address, false),
        AccountMeta::new(*token_destination_address, false),
        AccountMeta::new(*lockup_address, false),
        AccountMeta::new_readonly(escrow_authority_address, false),
        AccountMeta::new(escrow_token_account_address, false),
        AccountMeta::new_readonly(*mint_address, false),
        AccountMeta::new_readonly(*token_program_id, false),
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
