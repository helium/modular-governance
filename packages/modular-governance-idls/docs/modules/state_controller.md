[Documentation](../README.md) / [Modules](../modules.md) / state\_controller

# Module: state\_controller

## Table of contents

### Type Aliases

- [StateController](state_controller.md#statecontroller)

### Variables

- [IDL](state_controller.md#idl)

## Type Aliases

### StateController

Ƭ **StateController**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `accounts` | [{ `name`: ``"resolutionSettingsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"settings"`` ; `type`: { `defined`: ``"ResolutionStrategy"``  }  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }] |
| `errors` | [{ `code`: ``6000`` ; `msg`: ``"Error in arithmetic"`` ; `name`: ``"ArithmeticError"``  }] |
| `instructions` | [{ `accounts`: [{ `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"voteController"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"state_controller"``, ``"vote_controller"``]  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"VoteArgsV0"``  }  }] ; `name`: ``"onVoteV0"`` ; `returns`: { `option`: { `vec`: ``"u16"``  }  }  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"resolutionSettings"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"resolution_settings"``  }, { `kind`: ``"arg"`` ; `path`: ``"args.name"`` ; `type`: { `defined`: ``"InitializeResolutionSettingsArgsV0"``  }  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeResolutionSettingsArgsV0"``  }  }] ; `name`: ``"initializeResolutionSettingsV0"``  }, { `accounts`: [{ `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"owner"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"owner"``, ``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"state_controller"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"UpdateStateArgsV0"``  }  }] ; `name`: ``"updateStateV0"``  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"state_controller"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalProgram"``  }] ; `args`: [] ; `name`: ``"resolveV0"``  }] |
| `name` | ``"state_controller"`` |
| `types` | [{ `name`: ``"InitializeResolutionSettingsArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"settings"`` ; `type`: { `defined`: ``"ResolutionStrategy"``  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"VoteArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choice"`` ; `type`: ``"u16"``  }, { `name`: ``"weight"`` ; `type`: ``"u128"``  }, { `docs`: [``"This is a remove operation"``] ; `name`: ``"removeVote"`` ; `type`: ``"bool"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"UpdateStateArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"newState"`` ; `type`: { `defined`: ``"ProposalState"``  }  }] ; `kind`: ``"struct"``  }  }, { `docs`: [``"Reverse polish notation calculator"``, ``"https://en.wikipedia.org/wiki/Reverse_Polish_notation"``, ``"Do this to have a flat structure since rust doesn't like unbounded nesting of types"``] ; `name`: ``"ResolutionStrategy"`` ; `type`: { `fields`: [{ `name`: ``"nodes"`` ; `type`: { `vec`: { `defined`: ``"ResolutionNode"``  }  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"ProposalState"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `name`: ``"Draft"``  }, { `name`: ``"Cancelled"``  }, { `name`: ``"Voting"``  }, { `fields`: [{ `name`: ``"state"`` ; `type`: ``"string"``  }] ; `name`: ``"Custom"``  }]  }  }, { `docs`: [``"Allow building complex operations to decide resolution."``] ; `name`: ``"ResolutionNode"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `fields`: [{ `name`: ``"choices"`` ; `type`: { `vec`: ``"u16"``  }  }] ; `name`: ``"Resolved"``  }, { `fields`: [{ `name`: ``"end_ts"`` ; `type`: ``"i64"``  }] ; `name`: ``"EndTimestamp"``  }, { `fields`: [{ `name`: ``"offset"`` ; `type`: ``"i64"``  }] ; `name`: ``"OffsetFromStartTs"``  }, { `fields`: [{ `name`: ``"weight_threshold"`` ; `type`: ``"u128"``  }] ; `name`: ``"ChoiceVoteWeight"``  }, { `fields`: [{ `name`: ``"percentage"`` ; `type`: ``"i32"``  }] ; `name`: ``"ChoicePercentage"``  }, { `fields`: [{ `name`: ``"n"`` ; `type`: ``"u16"``  }] ; `name`: ``"Top"``  }, { `name`: ``"And"``  }, { `name`: ``"Or"``  }]  }  }] |
| `version` | ``"0.1.0"`` |

#### Defined in

state_controller.ts:1

## Variables

### IDL

• `Const` **IDL**: [`StateController`](state_controller.md#statecontroller)

#### Defined in

state_controller.ts:382
