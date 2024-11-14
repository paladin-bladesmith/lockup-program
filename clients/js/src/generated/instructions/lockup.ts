/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  combineCodec,
  getAddressDecoder,
  getAddressEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
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
  type ReadonlyAccount,
  type ReadonlySignerAccount,
  type TransactionSigner,
  type WritableAccount,
} from '@solana/web3.js';
import { PALADIN_LOCKUP_PROGRAM_ADDRESS } from '../programs';
import { getAccountMetaFactory, type ResolvedAccount } from '../shared';

export type LockupInstruction<
  TProgram extends string = typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
  TAccountLockupAuthority extends string | IAccountMeta<string> = string,
  TAccountTokenOwner extends string | IAccountMeta<string> = string,
  TAccountDepositorTokenAccount extends string | IAccountMeta<string> = string,
  TAccountLockupPool extends string | IAccountMeta<string> = string,
  TAccountLockupAccount extends string | IAccountMeta<string> = string,
  TAccountEscrowAuthority extends string | IAccountMeta<string> = string,
  TAccountEscrowTokenAccount extends string | IAccountMeta<string> = string,
  TAccountTokenMint extends string | IAccountMeta<string> = string,
  TAccountTokenProgram extends
    | string
    | IAccountMeta<string> = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  TRemainingAccounts extends readonly IAccountMeta<string>[] = [],
> = IInstruction<TProgram> &
  IInstructionWithData<Uint8Array> &
  IInstructionWithAccounts<
    [
      TAccountLockupAuthority extends string
        ? ReadonlyAccount<TAccountLockupAuthority>
        : TAccountLockupAuthority,
      TAccountTokenOwner extends string
        ? ReadonlySignerAccount<TAccountTokenOwner> &
            IAccountSignerMeta<TAccountTokenOwner>
        : TAccountTokenOwner,
      TAccountDepositorTokenAccount extends string
        ? WritableAccount<TAccountDepositorTokenAccount>
        : TAccountDepositorTokenAccount,
      TAccountLockupPool extends string
        ? WritableAccount<TAccountLockupPool>
        : TAccountLockupPool,
      TAccountLockupAccount extends string
        ? WritableAccount<TAccountLockupAccount>
        : TAccountLockupAccount,
      TAccountEscrowAuthority extends string
        ? ReadonlyAccount<TAccountEscrowAuthority>
        : TAccountEscrowAuthority,
      TAccountEscrowTokenAccount extends string
        ? WritableAccount<TAccountEscrowTokenAccount>
        : TAccountEscrowTokenAccount,
      TAccountTokenMint extends string
        ? ReadonlyAccount<TAccountTokenMint>
        : TAccountTokenMint,
      TAccountTokenProgram extends string
        ? ReadonlyAccount<TAccountTokenProgram>
        : TAccountTokenProgram,
      ...TRemainingAccounts,
    ]
  >;

export type LockupInstructionData = {
  discriminator: number;
  metadata: Address;
  amount: bigint;
};

export type LockupInstructionDataArgs = {
  metadata: Address;
  amount: number | bigint;
};

export function getLockupInstructionDataEncoder(): Encoder<LockupInstructionDataArgs> {
  return transformEncoder(
    getStructEncoder([
      ['discriminator', getU8Encoder()],
      ['metadata', getAddressEncoder()],
      ['amount', getU64Encoder()],
    ]),
    (value) => ({ ...value, discriminator: 1 })
  );
}

export function getLockupInstructionDataDecoder(): Decoder<LockupInstructionData> {
  return getStructDecoder([
    ['discriminator', getU8Decoder()],
    ['metadata', getAddressDecoder()],
    ['amount', getU64Decoder()],
  ]);
}

export function getLockupInstructionDataCodec(): Codec<
  LockupInstructionDataArgs,
  LockupInstructionData
> {
  return combineCodec(
    getLockupInstructionDataEncoder(),
    getLockupInstructionDataDecoder()
  );
}

export type LockupInput<
  TAccountLockupAuthority extends string = string,
  TAccountTokenOwner extends string = string,
  TAccountDepositorTokenAccount extends string = string,
  TAccountLockupPool extends string = string,
  TAccountLockupAccount extends string = string,
  TAccountEscrowAuthority extends string = string,
  TAccountEscrowTokenAccount extends string = string,
  TAccountTokenMint extends string = string,
  TAccountTokenProgram extends string = string,
> = {
  /** Lockup authority */
  lockupAuthority: Address<TAccountLockupAuthority>;
  /** Token owner */
  tokenOwner: TransactionSigner<TAccountTokenOwner>;
  /** Depositor token account */
  depositorTokenAccount: Address<TAccountDepositorTokenAccount>;
  /** Lockup pool */
  lockupPool: Address<TAccountLockupPool>;
  /** Lockup account */
  lockupAccount: Address<TAccountLockupAccount>;
  /** Escrow authority */
  escrowAuthority: Address<TAccountEscrowAuthority>;
  /** Escrow token account */
  escrowTokenAccount: Address<TAccountEscrowTokenAccount>;
  /** Token mint */
  tokenMint: Address<TAccountTokenMint>;
  /** Token program */
  tokenProgram?: Address<TAccountTokenProgram>;
  metadata: LockupInstructionDataArgs['metadata'];
  amount: LockupInstructionDataArgs['amount'];
};

export function getLockupInstruction<
  TAccountLockupAuthority extends string,
  TAccountTokenOwner extends string,
  TAccountDepositorTokenAccount extends string,
  TAccountLockupPool extends string,
  TAccountLockupAccount extends string,
  TAccountEscrowAuthority extends string,
  TAccountEscrowTokenAccount extends string,
  TAccountTokenMint extends string,
  TAccountTokenProgram extends string,
>(
  input: LockupInput<
    TAccountLockupAuthority,
    TAccountTokenOwner,
    TAccountDepositorTokenAccount,
    TAccountLockupPool,
    TAccountLockupAccount,
    TAccountEscrowAuthority,
    TAccountEscrowTokenAccount,
    TAccountTokenMint,
    TAccountTokenProgram
  >
): LockupInstruction<
  typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
  TAccountLockupAuthority,
  TAccountTokenOwner,
  TAccountDepositorTokenAccount,
  TAccountLockupPool,
  TAccountLockupAccount,
  TAccountEscrowAuthority,
  TAccountEscrowTokenAccount,
  TAccountTokenMint,
  TAccountTokenProgram
> {
  // Program address.
  const programAddress = PALADIN_LOCKUP_PROGRAM_ADDRESS;

  // Original accounts.
  const originalAccounts = {
    lockupAuthority: {
      value: input.lockupAuthority ?? null,
      isWritable: false,
    },
    tokenOwner: { value: input.tokenOwner ?? null, isWritable: false },
    depositorTokenAccount: {
      value: input.depositorTokenAccount ?? null,
      isWritable: true,
    },
    lockupPool: { value: input.lockupPool ?? null, isWritable: true },
    lockupAccount: { value: input.lockupAccount ?? null, isWritable: true },
    escrowAuthority: {
      value: input.escrowAuthority ?? null,
      isWritable: false,
    },
    escrowTokenAccount: {
      value: input.escrowTokenAccount ?? null,
      isWritable: true,
    },
    tokenMint: { value: input.tokenMint ?? null, isWritable: false },
    tokenProgram: { value: input.tokenProgram ?? null, isWritable: false },
  };
  const accounts = originalAccounts as Record<
    keyof typeof originalAccounts,
    ResolvedAccount
  >;

  // Original args.
  const args = { ...input };

  // Resolve default values.
  if (!accounts.tokenProgram.value) {
    accounts.tokenProgram.value =
      'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA' as Address<'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA'>;
  }

  const getAccountMeta = getAccountMetaFactory(programAddress, 'programId');
  const instruction = {
    accounts: [
      getAccountMeta(accounts.lockupAuthority),
      getAccountMeta(accounts.tokenOwner),
      getAccountMeta(accounts.depositorTokenAccount),
      getAccountMeta(accounts.lockupPool),
      getAccountMeta(accounts.lockupAccount),
      getAccountMeta(accounts.escrowAuthority),
      getAccountMeta(accounts.escrowTokenAccount),
      getAccountMeta(accounts.tokenMint),
      getAccountMeta(accounts.tokenProgram),
    ],
    programAddress,
    data: getLockupInstructionDataEncoder().encode(
      args as LockupInstructionDataArgs
    ),
  } as LockupInstruction<
    typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
    TAccountLockupAuthority,
    TAccountTokenOwner,
    TAccountDepositorTokenAccount,
    TAccountLockupPool,
    TAccountLockupAccount,
    TAccountEscrowAuthority,
    TAccountEscrowTokenAccount,
    TAccountTokenMint,
    TAccountTokenProgram
  >;

  return instruction;
}

export type ParsedLockupInstruction<
  TProgram extends string = typeof PALADIN_LOCKUP_PROGRAM_ADDRESS,
  TAccountMetas extends readonly IAccountMeta[] = readonly IAccountMeta[],
> = {
  programAddress: Address<TProgram>;
  accounts: {
    /** Lockup authority */
    lockupAuthority: TAccountMetas[0];
    /** Token owner */
    tokenOwner: TAccountMetas[1];
    /** Depositor token account */
    depositorTokenAccount: TAccountMetas[2];
    /** Lockup pool */
    lockupPool: TAccountMetas[3];
    /** Lockup account */
    lockupAccount: TAccountMetas[4];
    /** Escrow authority */
    escrowAuthority: TAccountMetas[5];
    /** Escrow token account */
    escrowTokenAccount: TAccountMetas[6];
    /** Token mint */
    tokenMint: TAccountMetas[7];
    /** Token program */
    tokenProgram: TAccountMetas[8];
  };
  data: LockupInstructionData;
};

export function parseLockupInstruction<
  TProgram extends string,
  TAccountMetas extends readonly IAccountMeta[],
>(
  instruction: IInstruction<TProgram> &
    IInstructionWithAccounts<TAccountMetas> &
    IInstructionWithData<Uint8Array>
): ParsedLockupInstruction<TProgram, TAccountMetas> {
  if (instruction.accounts.length < 9) {
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
      tokenOwner: getNextAccount(),
      depositorTokenAccount: getNextAccount(),
      lockupPool: getNextAccount(),
      lockupAccount: getNextAccount(),
      escrowAuthority: getNextAccount(),
      escrowTokenAccount: getNextAccount(),
      tokenMint: getNextAccount(),
      tokenProgram: getNextAccount(),
    },
    data: getLockupInstructionDataDecoder().decode(instruction.data),
  };
}
