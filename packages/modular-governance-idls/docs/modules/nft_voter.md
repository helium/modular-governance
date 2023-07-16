[Documentation](../README.md) / [Modules](../modules.md) / nft\_voter

# Module: nft\_voter

## Table of contents

### Type Aliases

- [NftVoter](nft_voter.md#nftvoter)

### Variables

- [IDL](nft_voter.md#idl)

## Type Aliases

### NftVoter

Ƭ **NftVoter**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `accounts` | [{ `name`: ``"nftVoterV0"`` ; `type`: { `fields`: [{ `name`: ``"authority"`` ; `type`: ``"publicKey"``  }, { `name`: ``"collection"`` ; `type`: ``"publicKey"``  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"voteMarkerV0"`` ; `type`: { `fields`: [{ `name`: ``"voter"`` ; `type`: ``"publicKey"``  }, { `name`: ``"nftVoter"`` ; `type`: ``"publicKey"``  }, { `name`: ``"proposal"`` ; `type`: ``"publicKey"``  }, { `name`: ``"mint"`` ; `type`: ``"publicKey"``  }, { `name`: ``"choices"`` ; `type`: { `vec`: ``"u16"``  }  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }] |
| `errors` | [{ `code`: ``6000`` ; `msg`: ``"Already voted for this choice"`` ; `name`: ``"AlreadyVoted"``  }, { `code`: ``6001`` ; `msg`: ``"Exceeded max choices"`` ; `name`: ``"MaxChoicesExceeded"``  }, { `code`: ``6002`` ; `msg`: ``"No vote to relinquish for this choice"`` ; `name`: ``"NoVoteForThisChoice"``  }] |
| `instructions` | [{ `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"nftVoter"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"nft_voter"``  }, { `kind`: ``"arg"`` ; `path`: ``"args.name"`` ; `type`: { `defined`: ``"InitializeNftVoterArgsV0"``  }  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"collection"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeNftVoterArgsV0"``  }  }] ; `name`: ``"initializeNftVoterV0"``  }, { `accounts`: [{ `docs`: [``"Account to receive sol refund if marker is closed"``] ; `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"refund"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"marker"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"marker"``  }, { `account`: ``"NftVoterV0"`` ; `kind`: ``"account"`` ; `path`: ``"nft_voter"`` ; `type`: ``"publicKey"``  }, { `account`: ``"Mint"`` ; `kind`: ``"account"`` ; `path`: ``"mint"`` ; `type`: ``"publicKey"``  }, { `account`: ``"ProposalV0"`` ; `kind`: ``"account"`` ; `path`: ``"proposal"`` ; `type`: ``"publicKey"``  }]  } ; `relations`: [``"nft_voter"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"nftVoter"``  }, { `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"voter"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"mint"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"metadata"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"tokenAccount"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"on_vote_hook"``, ``"state_controller"``]  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"onVoteHook"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalProgram"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"RelinquishVoteArgsV0"``  }  }] ; `name`: ``"relinquishVoteV0"``  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"marker"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"marker"``  }, { `account`: ``"NftVoterV0"`` ; `kind`: ``"account"`` ; `path`: ``"nft_voter"`` ; `type`: ``"publicKey"``  }, { `account`: ``"Mint"`` ; `kind`: ``"account"`` ; `path`: ``"mint"`` ; `type`: ``"publicKey"``  }, { `account`: ``"ProposalV0"`` ; `kind`: ``"account"`` ; `path`: ``"proposal"`` ; `type`: ``"publicKey"``  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"nftVoter"``  }, { `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"voter"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"mint"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"metadata"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"tokenAccount"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"proposal_config"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalConfig"`` ; `relations`: [``"on_vote_hook"``, ``"state_controller"``]  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"stateController"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"onVoteHook"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposalProgram"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"VoteArgsV0"``  }  }] ; `name`: ``"voteV0"``  }] |
| `name` | ``"nft_voter"`` |
| `types` | [{ `name`: ``"InitializeNftVoterArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"authority"`` ; `type`: ``"publicKey"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"RelinquishVoteArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choice"`` ; `type`: ``"u16"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"VoteArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choice"`` ; `type`: ``"u16"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"Key"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `name`: ``"Uninitialized"``  }, { `name`: ``"EditionV1"``  }, { `name`: ``"MasterEditionV1"``  }, { `name`: ``"ReservationListV1"``  }, { `name`: ``"MetadataV1"``  }, { `name`: ``"ReservationListV2"``  }, { `name`: ``"MasterEditionV2"``  }, { `name`: ``"EditionMarker"``  }, { `name`: ``"UseAuthorityRecord"``  }, { `name`: ``"CollectionAuthorityRecord"``  }, { `name`: ``"TokenOwnedEscrow"``  }, { `name`: ``"TokenRecord"``  }, { `name`: ``"MetadataDelegate"``  }]  }  }, { `name`: ``"CollectionDetails"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `fields`: [{ `name`: ``"size"`` ; `type`: ``"u64"``  }] ; `name`: ``"V1"``  }]  }  }, { `docs`: [``"Configuration for programmable assets."``] ; `name`: ``"ProgrammableConfig"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `fields`: [{ `docs`: [``"Programmable authorization rules."``] ; `name`: ``"rule_set"`` ; `type`: { `option`: ``"publicKey"``  }  }] ; `name`: ``"V1"``  }]  }  }, { `name`: ``"UseMethod"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `name`: ``"Burn"``  }, { `name`: ``"Multiple"``  }, { `name`: ``"Single"``  }]  }  }, { `name`: ``"TokenStandard"`` ; `type`: { `kind`: ``"enum"`` ; `variants`: [{ `name`: ``"NonFungible"``  }, { `name`: ``"FungibleAsset"``  }, { `name`: ``"Fungible"``  }, { `name`: ``"NonFungibleEdition"``  }, { `name`: ``"ProgrammableNonFungible"``  }]  }  }] |
| `version` | ``"0.1.0"`` |

#### Defined in

nft_voter.ts:1

## Variables

### IDL

• `Const` **IDL**: [`NftVoter`](nft_voter.md#nftvoter)

#### Defined in

nft_voter.ts:535
