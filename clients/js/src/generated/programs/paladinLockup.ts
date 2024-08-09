/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  containsBytes,
  getU8Encoder,
  type Address,
  type ReadonlyUint8Array,
} from '@solana/web3.js';
import {
  type ParsedLockupInstruction,
  type ParsedUnlockInstruction,
  type ParsedWithdrawInstruction,
} from '../instructions';

export const PALADIN_LOCKUP_PROGRAM_ADDRESS =
  'Dbf7u6x15DhjMrBMunY3XoRWdByrCCt2dbyoPrCXN6SQ' as Address<'Dbf7u6x15DhjMrBMunY3XoRWdByrCCt2dbyoPrCXN6SQ'>;

export enum PaladinLockupAccount {
  Lockup,
}

export enum PaladinLockupInstruction {
  Lockup,
  Unlock,
  Withdraw,
}

export function identifyPaladinLockupInstruction(
  instruction: { data: ReadonlyUint8Array } | ReadonlyUint8Array
): PaladinLockupInstruction {
  const data = 'data' in instruction ? instruction.data : instruction;
  if (containsBytes(data, getU8Encoder().encode(0), 0)) {
    return PaladinLockupInstruction.Lockup;
  }
  if (containsBytes(data, getU8Encoder().encode(1), 0)) {
    return PaladinLockupInstruction.Unlock;
  }
  if (containsBytes(data, getU8Encoder().encode(2), 0)) {
    return PaladinLockupInstruction.Withdraw;
  }
  throw new Error(
    'The provided instruction could not be identified as a paladinLockup instruction.'
  );
}

export type ParsedPaladinLockupInstruction<
  TProgram extends string = 'Dbf7u6x15DhjMrBMunY3XoRWdByrCCt2dbyoPrCXN6SQ',
> =
  | ({
      instructionType: PaladinLockupInstruction.Lockup;
    } & ParsedLockupInstruction<TProgram>)
  | ({
      instructionType: PaladinLockupInstruction.Unlock;
    } & ParsedUnlockInstruction<TProgram>)
  | ({
      instructionType: PaladinLockupInstruction.Withdraw;
    } & ParsedWithdrawInstruction<TProgram>);