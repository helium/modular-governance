[Documentation](../README.md) / [Modules](../modules.md) / proposal

# Module: proposal

## Table of contents

### Type Aliases

- [Proposal](proposal.md#proposal)

### Variables

- [IDL](proposal.md#idl)

## Type Aliases

### Proposal

Ƭ **Proposal**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `accounts` | [{ `name`: ``"proposalConfigV0"`` ; `type`: { `fields`: [{ `docs`: [``"Signer that controls voting and vote weights"``] ; `name`: ``"voteController"`` ; `type`: ``"publicKey"``  }, { `docs`: [``"Signer that controls the transitions of `ProposalState`"``, ``"You can either use the default `state-controller` smart contract"``, ``"Or you can implement a program that calls the `resolve_v0` method."``, ``"The vote can only be resolved when this `resolution_settings` PDA signs `resolve_v0`. This allows"``, ``"you to trigger resolution on either (a) a vote, (b) a timestamp, or (c) some custom trigger with clockwork"``] ; `name`: ``"stateController"`` ; `type`: ``"publicKey"``  }, { `docs`: [``"Optional program that will be called with `on_vote_v0` after every vote. This allows you to resolve"``, ``"the vote eagerly. For most use cases, this should just be the owner of the state controller."``, ``"WARNING: This program has full authority to set the outcome of votes, make sure you trust it"``] ; `name`: ``"onVoteHook"`` ; `type`: ``"publicKey"``  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"proposalV0"`` ; `type`: { `fields`: [{ `name`: ``"namespace"`` ; `type`: ``"publicKey"``  }, { `name`: ``"owner"`` ; `type`: ``"publicKey"``  }, { `name`: ``"state"`` ; `type`: { `defined`: ``"ProposalState"``  }  }, { `name`: ``"createdAt"`` ; `type`: ``"i64"``  }, { `name`: ``"proposalConfig"`` ; `type`: ``"publicKey"``  }, { `docs`: [``"Allows for multiple selection votes"``] ; `name`: ``"maxChoicesPerVoter"`` ; `type`: ``"u16"``  }, { `name`: ``"seed"`` ; `type`: ``"bytes"``  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `docs`: [``"URI to json containing name, description, etc"``] ; `name`: ``"uri"`` ; `type`: ``"string"``  }, { `name`: ``"tags"`` ; `type`: { `vec`: ``"string"``  }  }, { `name`: ``"choices"`` ; `type`: { `vec`: { `defined`: ``"Choice"``  }  }  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }] |
| `errors` | [{ `code`: ``6000`` ; `msg`: ``"Error in arithmetic"`` ; `name`: ``"ArithmeticError"``  }] |
| `instructions` | [{ `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `docs`: [``"Every proposal must have a namespace to prevent seed collision"``] ; `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"namespace"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"owner"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeProposalArgsV0"``  }  }] ; `name`: ``"initializeProposalV0"``  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `docs`: [``"Every proposal config must have an owner to prevent seed collision"``] ; `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"owner"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"proposal_config"``  }, { `kind`: ``"arg"`` ; `path`: ``"args.name"`` ; `type`: { `defined`: ``"InitializeProposalConfigArgsV0"``  }  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeProposalConfigArgsV0"``  }  }] ; `name`: ``"initializeProposalConfigV0"``  }, { `accounts`: [{ `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"voteController"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"on_vote_hook"``, ``"state_controller"``, ``"vote_controller"``]  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"onVoteHook"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"VoteArgsV0"``  }  }] ; `name`: ``"voteV0"``  }, { `accounts`: [{ `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"stateController"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"state_controller"``]  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"UpdateStateArgsV0"``  }  }] ; `name`: ``"updateStateV0"``  }] |
| `name` | ``"proposal"`` |
| `types` | [{ `name`: ``"InitializeProposalConfigArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `docs`: [``"Signer that controls voting and vote weights"``] ; `name`: ``"voteController"`` ; `type`: ``"publicKey"``  }, { `docs`: [``"Signer that controls the transitions of `ProposalState`"``, ``"You can either use the default `state-controller` smart contract"``, ``"Or you can implement a program that calls the `resolve_v0` method."``, ``"The vote can only be resolved when this `resolution_settings` PDA signs `resolve_v0`. This allows"``, ``"you to trigger resolution on either (a) a vote, (b) a timestamp, or (c) some custom trigger with clockwork"``] ; `name`: ``"stateController"`` ; `type`: ``"publicKey"``  }, { `docs`: [``"Optional program that will be called with `on_vote_v0` after every vote"``, ``"Defaults to the owner of `resolution_settings`, which allows it to reactively call resolve_v0"``] ; `name`: ``"onVoteHook"`` ; `type`: ``"publicKey"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"ChoiceArg"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `docs`: [``"Any other data that you may want to put in here"``] ; `name`: ``"uri"`` ; `type`: { `option`: ``"string"``  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"InitializeProposalArgsV0"`` ; `type`: { `fields`: [{ `docs`: [``"Allow a custom seed for indexing"``] ; `name`: ``"seed"`` ; `type`: ``"bytes"``  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"uri"`` ; `type`: ``"string"``  }, { `docs`: [``"Allows for multiple selection votes"``] ; `name`: ``"maxChoicesPerVoter"`` ; `type`: ``"u16"``  }, { `name`: ``"choices"`` ; `type`: { `vec`: { `defined`: ``"ChoiceArg"``  }  }  }, { `name`: ``"tags"`` ; `type`: { `vec`: ``"string"``  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"UpdateStateArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"newState"`` ; `type`: { `defined`: ``"ProposalState"``  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"VoteArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choice"`` ; `type`: ``"u16"``  }, { `name`: ``"weight"`` ; `type`: ``"u128"``  }, { `docs`: [``"This is a remove operation"``] ; `name`: ``"removeVote"`` ; `type`: ``"bool"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"Choice"`` ; `type`: { `fields`: [{ `docs`: [``"Total vote weight behind this choice. u128 to support u64 tokens multiplied by a large multiplier (as in helium)"``] ; `name`: ``"weight"`` ; `type`: ``"u128"``  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `docs`: [``"Any other data that you may want to put in here"``] ; `name`: ``"uri"`` ; `type`: { `option`: ``"string"``  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"ProposalState"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `name`: ``"Draft"``  }, { `name`: ``"Cancelled"``  }, { `fields`: [{ `name`: ``"start_ts"`` ; `type`: ``"i64"``  }] ; `name`: ``"Voting"``  }, { `fields`: [{ `name`: ``"choices"`` ; `type`: { `vec`: ``"u16"``  }  }] ; `name`: ``"Resolved"``  }, { `fields`: [{ `name`: ``"state"`` ; `type`: ``"string"``  }] ; `name`: ``"Custom"``  }]  }  }] |
| `version` | ``"0.1.0"`` |

#### Defined in

proposal.ts:1

## Variables

### IDL

• `Const` **IDL**: [`Proposal`](proposal.md#proposal)

#### Defined in

proposal.ts:521
