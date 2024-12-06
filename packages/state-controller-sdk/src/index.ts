import { StateController } from "@helium/modular-governance-idls/lib/types/state_controller";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { stateControllerResolvers } from "./resolvers";


export * from "./constants";
export * from "./pdas";
export * from "./resolvers";
export { SettingsBuilder, settings } from "./settingsBuilder";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<StateController>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
    // This is an Anchor 0.30+ IDL. Return the old IDLs
    // @ts-ignore
    if (!idl || idl?.address) {
      idl = IDL as any;
    }
  }

  const stateController = new Program<StateController>(
    idl as StateController,
    programId,
    provider,
    undefined,
    () => stateControllerResolvers
  ) as Program<StateController>;

  return stateController;
}

const IDL = {
  version: "0.1.2",
  name: "state_controller",
  instructions: [
    {
      name: "onVoteV0",
      accounts: [
        {
          name: "voter",
          isMut: false,
          isSigner: false,
        },
        {
          name: "voteController",
          isMut: false,
          isSigner: true,
        },
        {
          name: "stateController",
          isMut: true,
          isSigner: false,
        },
        {
          name: "proposal",
          isMut: false,
          isSigner: false,
          relations: ["proposal_config"],
        },
        {
          name: "proposalConfig",
          isMut: false,
          isSigner: false,
          relations: ["state_controller", "vote_controller"],
        },
      ],
      args: [
        {
          name: "args",
          type: {
            defined: "VoteArgsV0",
          },
        },
      ],
      returns: {
        option: {
          vec: "u16",
        },
      },
    },
    {
      name: "initializeResolutionSettingsV0",
      accounts: [
        {
          name: "payer",
          isMut: true,
          isSigner: true,
        },
        {
          name: "resolutionSettings",
          isMut: true,
          isSigner: false,
          pda: {
            seeds: [
              {
                kind: "const",
                type: "string",
                value: "resolution_settings",
              },
              {
                kind: "arg",
                type: {
                  defined: "InitializeResolutionSettingsArgsV0",
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
            defined: "InitializeResolutionSettingsArgsV0",
          },
        },
      ],
    },
    {
      name: "updateStateV0",
      accounts: [
        {
          name: "owner",
          isMut: false,
          isSigner: true,
        },
        {
          name: "proposal",
          isMut: true,
          isSigner: false,
          relations: ["owner", "proposal_config"],
        },
        {
          name: "proposalConfig",
          isMut: false,
          isSigner: false,
          relations: ["state_controller"],
        },
        {
          name: "stateController",
          isMut: false,
          isSigner: false,
        },
        {
          name: "proposalProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [
        {
          name: "args",
          type: {
            defined: "UpdateStateArgsV0",
          },
        },
      ],
    },
    {
      name: "resolveV0",
      accounts: [
        {
          name: "stateController",
          isMut: true,
          isSigner: false,
        },
        {
          name: "proposal",
          isMut: true,
          isSigner: false,
          relations: ["proposal_config"],
        },
        {
          name: "proposalConfig",
          isMut: false,
          isSigner: false,
          relations: ["state_controller"],
        },
        {
          name: "proposalProgram",
          isMut: false,
          isSigner: false,
        },
      ],
      args: [],
    },
  ],
  accounts: [
    {
      name: "resolutionSettingsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "settings",
            type: {
              defined: "ResolutionStrategy",
            },
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
      name: "InitializeResolutionSettingsArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "name",
            type: "string",
          },
          {
            name: "settings",
            type: {
              defined: "ResolutionStrategy",
            },
          },
        ],
      },
    },
    {
      name: "VoteArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "choice",
            type: "u16",
          },
          {
            name: "weight",
            type: "u128",
          },
          {
            name: "removeVote",
            docs: ["This is a remove operation"],
            type: "bool",
          },
        ],
      },
    },
    {
      name: "UpdateStateArgsV0",
      type: {
        kind: "struct",
        fields: [
          {
            name: "newState",
            type: {
              defined: "ProposalState",
            },
          },
        ],
      },
    },
    {
      name: "ResolutionStrategy",
      docs: [
        "Reverse polish notation calculator",
        "https://en.wikipedia.org/wiki/Reverse_Polish_notation",
        "Do this to have a flat structure since rust doesn't like unbounded nesting of types",
      ],
      type: {
        kind: "struct",
        fields: [
          {
            name: "nodes",
            type: {
              vec: {
                defined: "ResolutionNode",
              },
            },
          },
        ],
      },
    },
    {
      name: "ProposalState",
      type: {
        kind: "enum",
        variants: [
          {
            name: "Draft",
          },
          {
            name: "Cancelled",
          },
          {
            name: "Voting",
          },
          {
            name: "Custom",
            fields: [
              {
                name: "name",
                type: "string",
              },
              {
                name: "bin",
                type: "bytes",
              },
            ],
          },
        ],
      },
    },
    {
      name: "ResolutionNode",
      docs: ["Allow building complex operations to decide resolution."],
      type: {
        kind: "enum",
        variants: [
          {
            name: "Resolved",
            fields: [
              {
                name: "choices",
                type: {
                  vec: "u16",
                },
              },
            ],
          },
          {
            name: "EndTimestamp",
            fields: [
              {
                name: "end_ts",
                type: "i64",
              },
            ],
          },
          {
            name: "OffsetFromStartTs",
            fields: [
              {
                name: "offset",
                type: "i64",
              },
            ],
          },
          {
            name: "ChoiceVoteWeight",
            fields: [
              {
                name: "weight_threshold",
                type: "u128",
              },
            ],
          },
          {
            name: "ChoicePercentage",
            fields: [
              {
                name: "percentage",
                type: "i32",
              },
            ],
          },
          {
            name: "Top",
            fields: [
              {
                name: "n",
                type: "u16",
              },
            ],
          },
          {
            name: "NumResolved",
            fields: [
              {
                name: "n",
                type: "u16",
              },
            ],
          },
          {
            name: "And",
          },
          {
            name: "Or",
          },
          {
            name: "Not",
            fields: [
              {
                name: "choice_name",
                type: "string",
              },
            ],
          },
          {
            name: "TotalWeight",
            fields: [
              {
                name: "weight_threshold",
                type: "u128",
              },
            ],
          },
          {
            name: "ChoicePercentageOfCurrent",
            fields: [
              {
                name: "percentage",
                type: "i32",
              },
            ],
          },
        ],
      },
    },
  ],
  errors: [
    {
      code: 6000,
      name: "ProposalAlreadyResolved",
      msg: "Proposal was already resolved. Call resolve_v0",
    },
    {
      code: 6001,
      name: "ChoicesEmpty",
      msg: "Resolved choices must not be empty",
    },
    {
      code: 6002,
      name: "EndTimestampPassed",
      msg: "End timestamp has already passed",
    },
    {
      code: 6003,
      name: "InvalidOffset",
      msg: "Offset must be a positive integer",
    },
    {
      code: 6004,
      name: "InvalidPercentage",
      msg: "Percentage may not be less than 0 or greater than PERCENTAGE_DIVISOR",
    },
    {
      code: 6005,
      name: "InvalidTopN",
      msg: "Top n must be greater than 0",
    },
  ],
};