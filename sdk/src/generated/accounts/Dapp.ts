/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
import * as web3 from '@solana/web3.js'
import * as beetSolana from '@metaplex-foundation/beet-solana'

/**
 * Arguments used to create {@link Dapp}
 * @category Accounts
 * @category generated
 */
export type DappArgs = {
  id: number
  groupRoot: beet.bignum
  bump: number
  createKey: web3.PublicKey
  creator: web3.PublicKey
  group: web3.PublicKey
}

export const dappDiscriminator = [46, 9, 159, 144, 21, 128, 236, 52]
/**
 * Holds the data for the {@link Dapp} Account and provides de/serialization
 * functionality for that data
 *
 * @category Accounts
 * @category generated
 */
export class Dapp implements DappArgs {
  private constructor(
    readonly id: number,
    readonly groupRoot: beet.bignum,
    readonly bump: number,
    readonly createKey: web3.PublicKey,
    readonly creator: web3.PublicKey,
    readonly group: web3.PublicKey
  ) {}

  /**
   * Creates a {@link Dapp} instance from the provided args.
   */
  static fromArgs(args: DappArgs) {
    return new Dapp(
      args.id,
      args.groupRoot,
      args.bump,
      args.createKey,
      args.creator,
      args.group
    )
  }

  /**
   * Deserializes the {@link Dapp} from the data of the provided {@link web3.AccountInfo}.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static fromAccountInfo(
    accountInfo: web3.AccountInfo<Buffer>,
    offset = 0
  ): [Dapp, number] {
    return Dapp.deserialize(accountInfo.data, offset)
  }

  /**
   * Retrieves the account info from the provided address and deserializes
   * the {@link Dapp} from its data.
   *
   * @throws Error if no account info is found at the address or if deserialization fails
   */
  static async fromAccountAddress(
    connection: web3.Connection,
    address: web3.PublicKey,
    commitmentOrConfig?: web3.Commitment | web3.GetAccountInfoConfig
  ): Promise<Dapp> {
    const accountInfo = await connection.getAccountInfo(
      address,
      commitmentOrConfig
    )
    if (accountInfo == null) {
      throw new Error(`Unable to find Dapp account at ${address}`)
    }
    return Dapp.fromAccountInfo(accountInfo, 0)[0]
  }

  /**
   * Provides a {@link web3.Connection.getProgramAccounts} config builder,
   * to fetch accounts matching filters that can be specified via that builder.
   *
   * @param programId - the program that owns the accounts we are filtering
   */
  static gpaBuilder(
    programId: web3.PublicKey = new web3.PublicKey(
      'CGTjkfCkFqEPhp28aBK6afd2SaqeVTju1pdYZzdrX3dn'
    )
  ) {
    return beetSolana.GpaBuilder.fromStruct(programId, dappBeet)
  }

  /**
   * Deserializes the {@link Dapp} from the provided data Buffer.
   * @returns a tuple of the account data and the offset up to which the buffer was read to obtain it.
   */
  static deserialize(buf: Buffer, offset = 0): [Dapp, number] {
    return dappBeet.deserialize(buf, offset)
  }

  /**
   * Serializes the {@link Dapp} into a Buffer.
   * @returns a tuple of the created Buffer and the offset up to which the buffer was written to store it.
   */
  serialize(): [Buffer, number] {
    return dappBeet.serialize({
      accountDiscriminator: dappDiscriminator,
      ...this,
    })
  }

  /**
   * Returns the byteSize of a {@link Buffer} holding the serialized data of
   * {@link Dapp}
   */
  static get byteSize() {
    return dappBeet.byteSize
  }

  /**
   * Fetches the minimum balance needed to exempt an account holding
   * {@link Dapp} data from rent
   *
   * @param connection used to retrieve the rent exemption information
   */
  static async getMinimumBalanceForRentExemption(
    connection: web3.Connection,
    commitment?: web3.Commitment
  ): Promise<number> {
    return connection.getMinimumBalanceForRentExemption(
      Dapp.byteSize,
      commitment
    )
  }

  /**
   * Determines if the provided {@link Buffer} has the correct byte size to
   * hold {@link Dapp} data.
   */
  static hasCorrectByteSize(buf: Buffer, offset = 0) {
    return buf.byteLength - offset === Dapp.byteSize
  }

  /**
   * Returns a readable version of {@link Dapp} properties
   * and can be used to convert to JSON and/or logging
   */
  pretty() {
    return {
      id: this.id,
      groupRoot: (() => {
        const x = <{ toNumber: () => number }>this.groupRoot
        if (typeof x.toNumber === 'function') {
          try {
            return x.toNumber()
          } catch (_) {
            return x
          }
        }
        return x
      })(),
      bump: this.bump,
      createKey: this.createKey.toBase58(),
      creator: this.creator.toBase58(),
      group: this.group.toBase58(),
    }
  }
}

/**
 * @category Accounts
 * @category generated
 */
export const dappBeet = new beet.BeetStruct<
  Dapp,
  DappArgs & {
    accountDiscriminator: number[] /* size: 8 */
  }
>(
  [
    ['accountDiscriminator', beet.uniformFixedSizeArray(beet.u8, 8)],
    ['id', beet.u32],
    ['groupRoot', beet.u64],
    ['bump', beet.u8],
    ['createKey', beetSolana.publicKey],
    ['creator', beetSolana.publicKey],
    ['group', beetSolana.publicKey],
  ],
  Dapp.fromArgs,
  'Dapp'
)
