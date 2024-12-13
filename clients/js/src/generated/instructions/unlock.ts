/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getStructDecoder,
  getStructEncoder,
  getU8Decoder,
  getU8Encoder,
  transformEncoder,
  type Address,
  type Codec,
  type Decoder,
  type Encoder,
  type IAccountMeta,
  type IAccountSignerMeta,
  type IInstruction,
  type IInstructionWithAccounts,
  type IInstructionWithData,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { PALADIN_LOCKUP_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export type UnlockInstruction<
  TProgram extends string = typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
  TAccountLockupAuthority extends string | IAccountMeta<string> = string,
  TAccountLockupPool extends string | IAccountMeta<string> = string,
  TAccountLockupAccount extends string | IAccountMeta<string> = string,
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountLockupAuthority extends string
        ? ReadonlySignerAccount<TAccountLockupAuthority> &
            IAccountSignerMeta<TAccountLockupAuthority>
        : TAccountLockupAuthority,
      TAccountLockupPool extends string
        ? WritableAccount<TAccountLockupPool>
        : TAccountLockupPool,
      TAccountLockupAccount extends string
        ? WritableAccount<TAccountLockupAccount>
        : TAccountLockupAccount,
      ...TRemainingAccounts,
    ]
  >;

export type UnlockInstructionData = { discriminator: number };

export type UnlockInstructionDataArgs = {};

export function getUnlockInstructionDataEncoder(): Encoder<UnlockInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([['discriminator', getU8Encoder()]]),
    (value) => ({ ...value, discriminator: 2 })
  );
}

export function getUnlockInstructionDataDecoder(): Decoder<UnlockInstructionData> {
  return getStructDecoder([['discriminator', getU8Decoder()]]);
}

export function getUnlockInstructionDataCodec(): Codec<
  UnlockInstructionDataArgs,
  UnlockInstructionData
> {
  return combineCodec(
    getUnlockInstructionDataEncoder(),
    getUnlockInstructionDataDecoder()
  );
}

export type UnlockInput<
  TAccountLockupAuthority extends string = string,
  TAccountLockupPool extends string = string,
  TAccountLockupAccount extends string = string,
> = {
  /** Lockup authority */
  lockupAuthority: TransactionSigner<TAccountLockupAuthority>;
  /** Lockup pool */
  lockupPool: Address<TAccountLockupPool>;
  /** Lockup account */
  lockupAccount: Address<TAccountLockupAccount>;
};

export function getUnlockInstruction<
  TAccountLockupAuthority extends string,
  TAccountLockupPool extends string,
  TAccountLockupAccount extends string,
>(
  input: UnlockInput<
    TAccountLockupAuthority,
    TAccountLockupPool,
    TAccountLockupAccount
  >
): UnlockInstruction<
  typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
  TAccountLockupAuthority,
  TAccountLockupPool,
  TAccountLockupAccount
> {
  // Program address.
  const programAddress = PALADIN_LOCKUP_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    lockupAuthority: {
      value: input.lockupAuthority ?? null,
      isWritable: false,
    },
    lockupPool: { value: input.lockupPool ?? null, isWritable: true },
    lockupAccount: { value: input.lockupAccount ?? null, isWritable: true },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.lockupAuthority),
      getAccountMeta(accounts.lockupPool),
      getAccountMeta(accounts.lockupAccount),
    ],
    programAddress,
    data: getUnlockInstructionDataEncoder().encode({}),
  } as UnlockInstruction<
    typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
    TAccountLockupAuthority,
    TAccountLockupPool,
    TAccountLockupAccount
  >;

  return instruction;
}

export type ParsedUnlockInstruction<
  TProgram extends string = typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Lockup authority */
    lockupAuthority: TAccountMetas[0];
    /** Lockup pool */
    lockupPool: TAccountMetas[1];
    /** Lockup account */
    lockupAccount: TAccountMetas[2];
  };
  data: UnlockInstructionData;
};

export function parseUnlockInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedUnlockInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 3) {
    // TODO: Coded error.
    throw new Error('Not enough accounts');
  }
  let accountIndex = 0;
  const getNextAccount = () => {
    const accountMeta = instruction.accounts![accountIndex]!;
    accountIndex += 1;
    return accountMeta;
  };
  return {
    programAddress: instruction.programAddress,
    accounts: {
      lockupAuthority: getNextAccount(),
      lockupPool: getNextAccount(),
      lockupAccount: getNextAccount(),
    },
    data: getUnlockInstructionDataDecoder().decode(instruction.data),
  };
}
