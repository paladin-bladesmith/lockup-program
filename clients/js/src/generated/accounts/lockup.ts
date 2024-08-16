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
  getAddressDecoder,
  getAddressEncoder,
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
  getNullableU64Decoder,
  getNullableU64Encoder,
  type NullableU64,
  type NullableU64Args,
} from '../../hooked';

export type Lockup = {
  discriminator: Array<number>;
  amount: bigint;
  authority: Address;
  lockupStartTimestamp: bigint;
  lockupEndTimestamp: NullableU64;
  mint: Address;
  metadata: Address;
};

export type LockupArgs = {
  discriminator: Array<number>;
  amount: number | bigint;
  authority: Address;
  lockupStartTimestamp: number | bigint;
  lockupEndTimestamp: NullableU64Args;
  mint: Address;
  metadata: Address;
};

export function getLockupEncoder(): Encoder<LockupArgs> {
  return getStructEncoder([
    ['discriminator', getArrayEncoder(getU8Encoder(), { size: 8 })],
    ['amount', getU64Encoder()],
    ['authority', getAddressEncoder()],
    ['lockupStartTimestamp', getU64Encoder()],
    ['lockupEndTimestamp', getNullableU64Encoder()],
    ['mint', getAddressEncoder()],
    ['metadata', getAddressEncoder()],
  ]);
}

export function getLockupDecoder(): Decoder<Lockup> {
  return getStructDecoder([
    ['discriminator', getArrayDecoder(getU8Decoder(), { size: 8 })],
    ['amount', getU64Decoder()],
    ['authority', getAddressDecoder()],
    ['lockupStartTimestamp', getU64Decoder()],
    ['lockupEndTimestamp', getNullableU64Decoder()],
    ['mint', getAddressDecoder()],
    ['metadata', getAddressDecoder()],
  ]);
}

export function getLockupCodec(): Codec<LockupArgs, Lockup> {
  return combineCodec(getLockupEncoder(), getLockupDecoder());
}

export function decodeLockup<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress>
): Account<Lockup, TAddress>;
export function decodeLockup<TAddress extends string = string>(
  encodedAccount: MaybeEncodedAccount<TAddress>
): MaybeAccount<Lockup, TAddress>;
export function decodeLockup<TAddress extends string = string>(
  encodedAccount: EncodedAccount<TAddress> | MaybeEncodedAccount<TAddress>
): Account<Lockup, TAddress> | MaybeAccount<Lockup, TAddress> {
  return decodeAccount(
    encodedAccount as MaybeEncodedAccount<TAddress>,
    getLockupDecoder()
  );
}

export async function fetchLockup<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<Account<Lockup, TAddress>> {
  const maybeAccount = await fetchMaybeLockup(rpc, address, config);
  assertAccountExists(maybeAccount);
  return maybeAccount;
}

export async function fetchMaybeLockup<TAddress extends string = string>(
  rpc: Parameters<typeof fetchEncodedAccount>[0],
  address: Address<TAddress>,
  config?: FetchAccountConfig
): Promise<MaybeAccount<Lockup, TAddress>> {
  const maybeAccount = await fetchEncodedAccount(rpc, address, config);
  return decodeLockup(maybeAccount);
}

export async function fetchAllLockup(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<Account<Lockup>[]> {
  const maybeAccounts = await fetchAllMaybeLockup(rpc, addresses, config);
  assertAccountsExist(maybeAccounts);
  return maybeAccounts;
}

export async function fetchAllMaybeLockup(
  rpc: Parameters<typeof fetchEncodedAccounts>[0],
  addresses: Array<Address>,
  config?: FetchAccountsConfig
): Promise<MaybeAccount<Lockup>[]> {
  const maybeAccounts = await fetchEncodedAccounts(rpc, addresses, config);
  return maybeAccounts.map((maybeAccount) => decodeLockup(maybeAccount));
}
