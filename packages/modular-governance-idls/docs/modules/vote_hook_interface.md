[Documentation](../README.md) / [Modules](../modules.md) / vote\_hook\_interface

# Module: vote\_hook\_interface

## Table of contents

### Type Aliases

- [VoteHookInterface](vote_hook_interface.md#votehookinterface)

### Variables

- [IDL](vote_hook_interface.md#idl)

## Type Aliases

### VoteHookInterface

Ƭ **VoteHookInterface**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `instructions` | [{ `accounts`: [{ `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"voteController"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"proposal"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"VoteArgsV0"``  }  }] ; `docs`: [``"If this hook is being run on a state controller, we can optionally resolve the vote"``] ; `name`: ``"onVoteV0"`` ; `returns`: { `option`: { `vec`: ``"u16"``  }  }  }] |
| `name` | ``"vote_hook_interface"`` |
| `types` | [{ `name`: ``"VoteArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choice"`` ; `type`: ``"u16"``  }, { `name`: ``"weight"`` ; `type`: ``"u128"``  }, { `docs`: [``"This is a remove operation"``] ; `name`: ``"removeVote"`` ; `type`: ``"bool"``  }] ; `kind`: ``"struct"``  }  }] |
| `version` | ``"0.1.0"`` |

#### Defined in

vote_hook_interface.ts:1

## Variables

### IDL

• `Const` **IDL**: [`VoteHookInterface`](vote_hook_interface.md#votehookinterface)

#### Defined in

vote_hook_interface.ts:74
