/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import {
  assertAccountExists,
  assertAccountsExist,
  combineCodec,
  decodeAccount,
  fetchEncodedAccount,
  fetchEncodedAccounts,
  getArrayDecoder,
  getArrayEncoder,
  getStructDecoder,
  getStructEncoder,
  getU64Decoder,
  getU64Encoder,
  getU8Decoder,
  getU8Encoder,
  type Account,
  type Address,
  type Codec,
  type Decoder,
  type EncodedAccount,
  type Encoder,
  type FetchAccountConfig,
  type FetchAccountsConfig,
  type MaybeAccount,
  type MaybeEncodedAccount,
} from '@solana/web3.js';
import {
  getLockupPoolEntryDecoder,
  getLockupPoolEntryEncoder,
  type LockupPoolEntry,
  type LockupPoolEntryArgs,
} from '../types';

export type LockupPool = {
  discriminator: Array<number>;
  entries: Array<LockupPoolEntry>;
  entriesLen: bigint;
};

export type LockupPoolArgs = {
  discriminator: Array<number>;
  entries: Array<LockupPoolEntryArgs>;
  entriesLen: number | bigint;
};

export function getLockupPoolEncoder(): Encoder<LockupPoolArgs> {
  return getStructEncoder([
    ['discriminator', getArrayEncoder(getU8Encoder(), { size: 8 })],
    ['entries', getArrayEncoder(getLockupPoolEntryEncoder(), { size: 1024 })],
    ['entriesLen', getU64Encoder()],
  ]);
}

export function getLockupPoolDecoder(): Decoder<LockupPool> {
  return getStructDecoder([
    ['discriminator', getArrayDecoder(getU8Decoder(), { size: 8 })],
    ['entries', getArrayDecoder(getLockupPoolEntryDecoder(), { size: 1024 })],
    ['entriesLen', getU64Decoder()],
  ]);
}

export function getLockupPoolCodec(): Codec<LockupPoolArgs, LockupPool> {
  return combineCodec(getLockupPoolEncoder(), getLockupPoolDecoder());
}

export function decodeLockupPool<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<LockupPool, TAddress>;
export function decodeLockupPool<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<LockupPool, TAddress>;
export function decodeLockupPool<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<LockupPool, TAddress> | MaybeAccount<LockupPool, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getLockupPoolDecoder()
  );
}

export async function fetchLockupPool<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<LockupPool, TAddress>> {
  const maybeAccount = await fetchMaybeLockupPool(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeLockupPool<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<LockupPool, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeLockupPool(maybeAccount);
}

export async function fetchAllLockupPool(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<LockupPool>[]> {
  const maybeAccounts = await fetchAllMaybeLockupPool(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeLockupPool(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<LockupPool>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeLockupPool(maybeAccount));
}

export function getLockupPoolSize(): number {
  return 40976;
}
