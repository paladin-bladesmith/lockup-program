//! Program instruction types.

use {
    crate::state::get_escrow_address,
    solana_program::{
        instruction::{AccountMeta, Instruction},
        program_error::ProgramError,
        pubkey::Pubkey,
        system_program,
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
    /// 0. `[s]` Depositor owner.
    /// 1. `[w]` Depositor token account.
    /// 2. `[w]` Lockup account.
    /// 3. `[ ]` Escrow account.
    /// 4. `[w]` Escrow token account.
    /// 5. `[ ]` Token mint.
    /// 6. `[ ]` Token program.
    Lockup { amount: u64, period_seconds: u64 },
    /// Unlock a token lockup, enabling the tokens for withdrawal.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[s]` Depositor owner.
    /// 1. `[ ]` Depositor token account.
    /// 2. `[w]` Lockup account.
    Unlock,
    /// Withdraw tokens from a lockup account.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Depositor token account.
    /// 1. `[w]` Lockup account.
    /// 2. `[ ]` Escrow account.
    /// 3. `[w]` Escrow token account.
    /// 4. `[ ]` Token mint.
    /// 5. `[ ]` Token program.
    Withdraw,
    /// Initialize the escrow account.
    ///
    /// Expects an uninitialized escrow account with enough rent-exempt
    /// lamports to store escrow state, owned by the System program.
    ///
    /// This instruction is permissionless, but can only be invoked once.
    ///
    /// Accounts expected by this instruction:
    ///
    /// 0. `[w]` Escrow account.
    /// 1. `[w]` Escrow token account.
    /// 2. `[ ]` Token mint.
    /// 3. `[ ]` Token program.
    InitializeEscrow,
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
            Self::InitializeEscrow => vec![3],
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
            Some((&3, _)) => Ok(Self::InitializeEscrow),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}

/// Creates a
/// [Lockup](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn lockup(
    owner_address: &Pubkey,
    token_account_address: &Pubkey,
    lockup_address: &Pubkey,
    mint_address: &Pubkey,
    amount: u64,
    period_seconds: u64,
) -> Instruction {
    let escrow_address = get_escrow_address(&crate::id());
    let escrow_token_account_address = get_associated_token_address_with_program_id(
        &escrow_address,
        mint_address,
        &spl_token_2022::id(),
    );
    let accounts = vec![
        AccountMeta::new_readonly(*owner_address, true),
        AccountMeta::new(*token_account_address, false),
        AccountMeta::new(*lockup_address, false),
        AccountMeta::new_readonly(escrow_address, false),
        AccountMeta::new(escrow_token_account_address, false),
        AccountMeta::new_readonly(*mint_address, false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
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
pub fn unlock(
    owner_address: &Pubkey,
    token_account_address: &Pubkey,
    lockup_address: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new_readonly(*owner_address, true),
        AccountMeta::new_readonly(*token_account_address, false),
        AccountMeta::new(*lockup_address, false),
    ];
    let data = PaladinLockupInstruction::Unlock.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

/// Creates a
/// [Withdraw](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn withdraw(
    token_account_address: &Pubkey,
    lockup_address: &Pubkey,
    mint_address: &Pubkey,
) -> Instruction {
    let escrow_address = get_escrow_address(&crate::id());
    let escrow_token_account_address = get_associated_token_address_with_program_id(
        &escrow_address,
        mint_address,
        &spl_token_2022::id(),
    );
    let accounts = vec![
        AccountMeta::new(*token_account_address, false),
        AccountMeta::new(*lockup_address, false),
        AccountMeta::new_readonly(escrow_address, false),
        AccountMeta::new(escrow_token_account_address, false),
        AccountMeta::new_readonly(*mint_address, false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
    ];
    let data = PaladinLockupInstruction::Withdraw.pack();
    Instruction::new_with_bytes(crate::id(), &data, accounts)
}

/// Creates an
/// [InitializeEscrow](enum.PaladinLockupInstruction.html)
/// instruction.
pub fn initialize_escrow(mint_address: &Pubkey) -> Instruction {
    let escrow_address = get_escrow_address(&crate::id());
    let escrow_token_account_address = get_associated_token_address_with_program_id(
        &escrow_address,
        mint_address,
        &spl_token_2022::id(),
    );
    let accounts = vec![
        AccountMeta::new(escrow_address, false),
        AccountMeta::new(escrow_token_account_address, false),
        AccountMeta::new_readonly(*mint_address, false),
        AccountMeta::new_readonly(spl_token_2022::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    let data = PaladinLockupInstruction::InitializeEscrow.pack();
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

    #[test]
    fn test_pack_unpack_initialize_escrow() {
        test_pack_unpack(PaladinLockupInstruction::InitializeEscrow);
    }
}
