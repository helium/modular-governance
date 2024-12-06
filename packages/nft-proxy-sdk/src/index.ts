import { NftProxy } from "@helium/modular-governance-idls/lib/types/nft_proxy";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { nftProxyResolvers } from "./resolvers";

export * from "./constants";
export * from "./pdas";
export * from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<NftProxy>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
    // This is an Anchor 0.30+ IDL. Return the old IDLs
    // @ts-ignore
    if (!idl || idl?.address) {
      idl = IDL as any;
    }
  }

  const tokenVoter = new Program<NftProxy>(
    idl as NftProxy,
    programId,
    provider,
    undefined,
    () => nftProxyResolvers
  ) as Program<NftProxy>;

  return tokenVoter;
}

const IDL = {
  version: "0.0.1",
  name: "nft_proxy",
  instructions: [
    {
      name: "initializeProxyConfigV0",
      accounts: [
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: false,
        },
        {
          name: "proxyConfig",
          isMut: true,
          isSigner: false,
          pda: {
            seeds: [
              {
                kind: "const",
                type: "string",
                value: "proxy_config",
              },
              {
                kind: "arg",
                type: {
                  defined: "InitializeProxyConfigArgsV0",
                },
                path: "args.name",
              },
            ],
          },
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "args",
          type: {
            defined: "InitializeProxyConfigArgsV0",
          },
        },
      ],
    },
    {
      name: "assignProxyV0",
      accounts: [
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "asset",
          isMut: false,
          isSigner: false,
        },
        {
          name: "approver",
          isMut: false,
          isSigner: true,
        },
        {
          name: "voter",
          isMut: false,
          isSigner: false,
          docs: [
            "or in the case of a primary proxy (first in the line), Pubkey::default",
          ],
        },
        {
          name: "tokenAccount",
          isMut: false,
          isSigner: false,
          isOptional: true,
        },
        {
          name: "proxyConfig",
          isMut: false,
          isSigner: false,
        },
        {
          name: "currentProxyAssignment",
          isMut: true,
          isSigner: false,
          pda: {
            seeds: [
              {
                kind: "const",
                type: "string",
                value: "proxy_assignment",
              },
              {
                kind: "account",
                type: "publicKey",
                account: "ProxyConfigV0",
                path: "proxy_config",
              },
              {
                kind: "account",
                type: "publicKey",
                account: "Mint",
                path: "asset",
              },
              {
                kind: "account",
                type: "publicKey",
                path: "voter",
              },
            ],
          },
        },
        {
          name: "recipient",
          isMut: false,
          isSigner: false,
        },
        {
          name: "nextProxyAssignment",
          isMut: true,
          isSigner: false,
          pda: {
            seeds: [
              {
                kind: "const",
                type: "string",
                value: "proxy_assignment",
              },
              {
                kind: "account",
                type: "publicKey",
                account: "ProxyConfigV0",
                path: "proxy_config",
              },
              {
                kind: "account",
                type: "publicKey",
                account: "Mint",
                path: "asset",
              },
              {
                kind: "account",
                type: "publicKey",
                path: "recipient",
              },
            ],
          },
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "args",
          type: {
            defined: "AssignProxyArgsV0",
          },
        },
      ],
    },
    {
      name: "unassignProxyV0",
      accounts: [
        {
          name: "rentRefund",
          isMut: true,
          isSigner: false,
        },
        {
          name: "asset",
          isMut: false,
          isSigner: false,
        },
        {
          name: "approver",
          isMut: false,
          isSigner: true,
        },
        {
          name: "voter",
          isMut: false,
          isSigner: false,
          docs: [
            "or in the case of a primary proxy (first in the line), Pubkey::default",
          ],
        },
        {
          name: "tokenAccount",
          isMut: false,
          isSigner: false,
          isOptional: true,
        },
        {
          name: "currentProxyAssignment",
          isMut: false,
          isSigner: false,
          relations: ["proxy_config", "voter", "asset"],
        },
        {
          name: "prevProxyAssignment",
          isMut: true,
          isSigner: false,
          relations: ["proxy_config"],
        },
        {
          name: "proxyAssignment",
          isMut: true,
          isSigner: false,
          relations: ["proxy_config", "rent_refund"],
        },
        {
          name: "proxyConfig",
          isMut: false,
          isSigner: false,
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: "updateProxyConfigV0",
      accounts: [
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: false,
        },
        {
          name: "proxyConfig",
          isMut: true,
          isSigner: false,
          relations: ["authority"],
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "args",
          type: {
            defined: "UpdateProxyConfigArgsV0",
          },
        },
      ],
    },
    {
      name: "unassignExpiredProxyV0",
      accounts: [
        {
          name: "rentRefund",
          isMut: true,
          isSigner: false,
        },
        {
          name: "prevProxyAssignment",
          isMut: true,
          isSigner: false,
        },
        {
          name: "proxyAssignment",
          isMut: true,
          isSigner: false,
          relations: ["rent_refund"],
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
    {
      name: "closeExpiredProxyV0",
      accounts: [
        {
          name: "rentRefund",
          isMut: true,
          isSigner: false,
        },
        {
          name: "proxyAssignment",
          isMut: true,
          isSigner: false,
          relations: ["rent_refund"],
        },
        {
          name: "systemProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: "proxyConfigV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "name",
            type: "string",
          },
          {
            name: "maxProxyTime",
            type: "i64",
          },
          {
            name: "seasons",
            type: {
              vec: {
                defined: "SeasonV0",
              },
            },
          },
        ],
      },
    },
    {
      name: "proxyAssignmentV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "voter",
            type: "publicKey",
          },
          {
            name: "proxyConfig",
            type: "publicKey",
          },
          {
            name: "asset",
            type: "publicKey",
          },
          {
            name: "index",
            type: "u16",
          },
          {
            name: "nextVoter",
            type: "publicKey",
          },
          {
            name: "rentRefund",
            type: "publicKey",
          },
          {
            name: "expirationTime",
            type: "i64",
          },
          {
            name: "bumpSeed",
            type: "u8",
          },
        ],
      },
    },
  ],
  types: [
    {
      name: "AssignProxyArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "expirationTime",
            type: "i64",
          },
        ],
      },
    },
    {
      name: "InitializeProxyConfigArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "maxProxyTime",
            type: "i64",
          },
          {
            name: "seasons",
            type: {
              vec: {
                defined: "SeasonV0",
              },
            },
          },
        ],
      },
    },
    {
      name: "UpdateProxyConfigArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "maxProxyTime",
            type: {
              option: "i64",
            },
          },
          {
            name: "seasons",
            type: {
              option: {
                vec: {
                  defined: "SeasonV0",
                },
              },
            },
          },
        ],
      },
    },
    {
      name: "SeasonV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "start",
            type: "i64",
          },
          {
            name: "end",
            type: "i64",
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: "ExpirationExceedsMax",
      msg: "The specified expiration time exceeds the maximum allowed for this proxy configuration",
    },
    {
      code: 6001,
      name: "ExpirationExceedsSeasonMax",
      msg: "The specified expiration time exceeds the maximum allowed for this season",
    },
    {
      code: 6002,
      name: "ExpirationPast",
      msg: "The specified expiration time has already passed",
    },
    {
      code: 6003,
      name: "ExpirationExceedsPreceedingProxy",
      msg: "The specified expiration time exceeds the expiration of the existing delegatio",
    },
    {
      code: 6004,
      name: "SeasonsNotSorted",
      msg: "The seasons are not sorted",
    },
    {
      code: 6005,
      name: "InvalidDataIncrease",
      msg: "The data size increase is not valid",
    },
    {
      code: 6006,
      name: "ExpirationTimeInvalid",
      msg: "The expiration time is invalid",
    },
    {
      code: 6007,
      name: "ExpirationNotPast",
      msg: "The specified expiration time has not passed",
    },
  ],
};