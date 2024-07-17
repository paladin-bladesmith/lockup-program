//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct Unlock {
    pub depositor_owner_account: solana_program::pubkey::Pubkey,

    pub depositor_token_account: solana_program::pubkey::Pubkey,

    pub lockup_account: solana_program::pubkey::Pubkey,
}

impl Unlock {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.depositor_owner_account,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.depositor_token_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lockup_account,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = UnlockInstructionData::new().try_to_vec().unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::PALADIN_LOCKUP_PROGRAM_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct UnlockInstructionData {
    discriminator: u8,
}

impl UnlockInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 1 }
    }
}

impl Default for UnlockInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `Unlock`.
///
/// ### Accounts:
///
///   0. `[signer]` depositor_owner_account
///   1. `[]` depositor_token_account
///   2. `[writable]` lockup_account
#[derive(Clone, Debug, Default)]
pub struct UnlockBuilder {
    depositor_owner_account: Option<solana_program::pubkey::Pubkey>,
    depositor_token_account: Option<solana_program::pubkey::Pubkey>,
    lockup_account: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl UnlockBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn depositor_owner_account(
        &mut self,
        depositor_owner_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.depositor_owner_account = Some(depositor_owner_account);
        self
    }
    #[inline(always)]
    pub fn depositor_token_account(
        &mut self,
        depositor_token_account: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.depositor_token_account = Some(depositor_token_account);
        self
    }
    #[inline(always)]
    pub fn lockup_account(&mut self, lockup_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lockup_account = Some(lockup_account);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Unlock {
            depositor_owner_account: self
                .depositor_owner_account
                .expect("depositor_owner_account is not set"),
            depositor_token_account: self
                .depositor_token_account
                .expect("depositor_token_account is not set"),
            lockup_account: self.lockup_account.expect("lockup_account is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `unlock` CPI accounts.
pub struct UnlockCpiAccounts<'a, 'b> {
    pub depositor_owner_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub depositor_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lockup_account: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `unlock` CPI instruction.
pub struct UnlockCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub depositor_owner_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub depositor_token_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub lockup_account: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> UnlockCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: UnlockCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            depositor_owner_account: accounts.depositor_owner_account,
            depositor_token_account: accounts.depositor_token_account,
            lockup_account: accounts.lockup_account,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(3 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.depositor_owner_account.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.depositor_token_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lockup_account.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = UnlockInstructionData::new().try_to_vec().unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::PALADIN_LOCKUP_PROGRAM_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(3 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.depositor_owner_account.clone());
        account_infos.push(self.depositor_token_account.clone());
        account_infos.push(self.lockup_account.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Unlock` via CPI.
///
/// ### Accounts:
///
///   0. `[signer]` depositor_owner_account
///   1. `[]` depositor_token_account
///   2. `[writable]` lockup_account
#[derive(Clone, Debug)]
pub struct UnlockCpiBuilder<'a, 'b> {
    instruction: Box<UnlockCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> UnlockCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(UnlockCpiBuilderInstruction {
            __program: program,
            depositor_owner_account: None,
            depositor_token_account: None,
            lockup_account: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn depositor_owner_account(
        &mut self,
        depositor_owner_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.depositor_owner_account = Some(depositor_owner_account);
        self
    }
    #[inline(always)]
    pub fn depositor_token_account(
        &mut self,
        depositor_token_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.depositor_token_account = Some(depositor_token_account);
        self
    }
    #[inline(always)]
    pub fn lockup_account(
        &mut self,
        lockup_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lockup_account = Some(lockup_account);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool`
    /// indicating whether the account is writable or not, and a `bool`
    /// indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = UnlockCpi {
            __program: self.instruction.__program,

            depositor_owner_account: self
                .instruction
                .depositor_owner_account
                .expect("depositor_owner_account is not set"),

            depositor_token_account: self
                .instruction
                .depositor_token_account
                .expect("depositor_token_account is not set"),

            lockup_account: self
                .instruction
                .lockup_account
                .expect("lockup_account is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct UnlockCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    depositor_owner_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    depositor_token_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lockup_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
