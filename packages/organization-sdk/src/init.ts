import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { Organization } from "@helium/modular-governance-idls/lib/types/organization";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { organizationsResolvers } from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<Organization>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
    // This is an Anchor 0.30+ IDL. Return the old IDLs
    // @ts-ignore
    if (!idl || idl?.address) {
      idl = IDL as any;
    }
  }

  const organizations = new Program<Organization>(
    idl as Organization,
    programId,
    provider,
    undefined,
    () => organizationsResolvers
  ) as Program<Organization>;

  return organizations;
}

const IDL = {
  version: "0.1.1",
  name: "organization",
  instructions: [
    {
      name: "initializeOrganizationV0",
      accounts: [
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "organization",
          isMut: true,
          isSigner: false,
          pda: {
            seeds: [
              {
                kind: "const",
                type: "string",
                value: "organization",
              },
              {
                kind: "arg",
                type: {
                  defined: "InitializeOrganizationArgsV0",
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
            defined: "InitializeOrganizationArgsV0",
          },
        },
      ],
    },
    {
      name: "initializeProposalV0",
      accounts: [
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
        {
          name: "owner",
          isMut: false,
          isSigner: false,
        },
        {
          name: "proposal",
          isMut: true,
          isSigner: false,
        },
        {
          name: "proposalConfig",
          isMut: false,
          isSigner: false,
        },
        {
          name: "organization",
          isMut: true,
          isSigner: false,
          relations: ["proposal_program", "authority"],
        },
        {
          name: "proposalProgram",
          isMut: false,
          isSigner: false,
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
            defined: "InitializeProposalArgsV0",
          },
        },
      ],
    },
    {
      name: "updateOrganizationV0",
      accounts: [
        {
          name: "organization",
          isMut: true,
          isSigner: false,
          relations: ["authority"],
        },
        {
          name: "authority",
          isMut: false,
          isSigner: true,
        },
      ],
      args: [
        {
          name: "args",
          type: {
            defined: "UpdateOrganizationArgsV0",
          },
        },
      ],
    },
  ],
  accounts: [
    {
      name: "organizationV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "numProposals",
            type: "u32",
          },
          {
            name: "authority",
            docs: ["Authority to create proposals under this organization"],
            type: "publicKey",
          },
          {
            name: "defaultProposalConfig",
            type: "publicKey",
          },
          {
            name: "proposalProgram",
            type: "publicKey",
          },
          {
            name: "name",
            type: "string",
          },
          {
            name: "uri",
            type: "string",
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
      name: "InitializeOrganizationArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "authority",
            type: "publicKey",
          },
          {
            name: "defaultProposalConfig",
            type: "publicKey",
          },
          {
            name: "proposalProgram",
            type: "publicKey",
          },
          {
            name: "uri",
            type: "string",
          },
        ],
      },
    },
    {
      name: "ChoiceArg",
      type: {
        kind: "struct",
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "uri",
            docs: ["Any other data that you may want to put in here"],
            type: {
              option: "string",
            },
          },
        ],
      },
    },
    {
      name: "InitializeProposalArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "uri",
            type: "string",
          },
          {
            name: "maxChoicesPerVoter",
            type: "u16",
          },
          {
            name: "choices",
            type: {
              vec: {
                defined: "ChoiceArg",
              },
            },
          },
          {
            name: "tags",
            type: {
              vec: "string",
            },
          },
        ],
      },
    },
    {
      name: "UpdateOrganizationArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "authority",
            type: {
              option: "publicKey",
            },
          },
          {
            name: "defaultProposalConfig",
            type: {
              option: "publicKey",
            },
          },
          {
            name: "proposalProgram",
            type: {
              option: "publicKey",
            },
          },
          {
            name: "uri",
            type: {
              option: "string",
            },
          },
        ],
      },
    },
  ],
};