[Documentation](../README.md) / [Modules](../modules.md) / organization\_wallet

# Module: organization\_wallet

## Table of contents

### Type Aliases

- [OrganizationWallet](organization_wallet.md#organizationwallet)

### Variables

- [IDL](organization_wallet.md#idl)

## Type Aliases

### OrganizationWallet

Ƭ **OrganizationWallet**: `Object`

#### Type declaration

| Name | Type |
| :------ | :------ |
| `accounts` | [{ `name`: ``"organizationWalletV0"`` ; `type`: { `fields`: [{ `name`: ``"index"`` ; `type`: ``"u16"``  }, { `name`: ``"organization"`` ; `type`: ``"publicKey"``  }, { `name`: ``"wallet"`` ; `type`: ``"publicKey"``  }, { `name`: ``"proposalConfigs"`` ; `type`: { `vec`: ``"publicKey"``  }  }, { `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }, { `name`: ``"walletBumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"walletProposalV0"`` ; `type`: { `fields`: [{ `name`: ``"proposal"`` ; `type`: ``"publicKey"``  }, { `name`: ``"organizationWallet"`` ; `type`: ``"publicKey"``  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }, { `name`: ``"choiceTransactions"`` ; `type`: { `vec`: { `vec`: { `defined`: ``"CompiledTransaction"``  }  }  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"organizationWalletPropoalV0"`` ; `type`: { `fields`: [{ `name`: ``"organizationWallet"`` ; `type`: ``"publicKey"``  }, { `name`: ``"proposal"`` ; `type`: ``"publicKey"``  }, { `name`: ``"accounts"`` ; `type`: { `vec`: ``"publicKey"``  }  }, { `name`: ``"transactionsByChoice"`` ; `type`: { `vec`: { `vec`: { `defined`: ``"CompiledTransaction"``  }  }  }  }, { `name`: ``"bumpSeed"`` ; `type`: ``"u8"``  }] ; `kind`: ``"struct"``  }  }] |
| `errors` | [{ `code`: ``6000`` ; `msg`: ``"The realloc increase was too large"`` ; `name`: ``"InvalidDataIncrease"``  }, { `code`: ``6001`` ; `name`: ``"InstructionSerializeFailed"``  }] |
| `instructions` | [{ `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"organizationWallet"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"organization_wallet"``  }, { `account`: ``"OrganizationV0"`` ; `kind`: ``"account"`` ; `path`: ``"organization"`` ; `type`: ``"publicKey"``  }, { `kind`: ``"arg"`` ; `path`: ``"args.index"`` ; `type`: { `defined`: ``"InitializeOrganizationWalletArgsV0"``  }  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"organization"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"InitializeOrganizationWalletArgsV0"``  }  }] ; `name`: ``"initializeOrganizationWalletV0"``  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"organizationWallet"``  }, { `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"authority"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"owner"`` ; `relations`: [``"authority"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"owner"``]  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"walletProposal"`` ; `pda`: { `seeds`: [{ `kind`: ``"const"`` ; `type`: ``"string"`` ; `value`: ``"wallet_proposal"``  }, { `account`: ``"OrganizationWalletV0"`` ; `kind`: ``"account"`` ; `path`: ``"organization_wallet"`` ; `type`: ``"publicKey"``  }, { `account`: ``"ProposalV0"`` ; `kind`: ``"account"`` ; `path`: ``"proposal"`` ; `type`: ``"publicKey"``  }]  }  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [] ; `name`: ``"initializeWalletProposalV0"``  }, { `accounts`: [{ `isMut`: ``true`` ; `isSigner`: ``true`` ; `name`: ``"payer"``  }, { `isMut`: ``false`` ; `isSigner`: ``true`` ; `name`: ``"authority"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"owner"`` ; `relations`: [``"authority"``]  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"proposal"`` ; `relations`: [``"owner"``]  }, { `isMut`: ``true`` ; `isSigner`: ``false`` ; `name`: ``"walletProposal"``  }, { `isMut`: ``false`` ; `isSigner`: ``false`` ; `name`: ``"systemProgram"``  }] ; `args`: [{ `name`: ``"args"`` ; `type`: { `defined`: ``"SetTransactionsArgsV0"``  }  }] ; `name`: ``"setTransactionsV0"``  }] |
| `name` | ``"organization_wallet"`` |
| `types` | [{ `name`: ``"ExecuteTransactionArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choice"`` ; `type`: ``"u16"``  }, { `name`: ``"transaction"`` ; `type`: ``"u16"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"InitializeOrganizationWalletArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"name"`` ; `type`: ``"string"``  }, { `name`: ``"authority"`` ; `type`: ``"publicKey"``  }, { `docs`: [``"List of valid proposal configs to execute on this wallet"``] ; `name`: ``"proposalConfigs"`` ; `type`: { `vec`: ``"publicKey"``  }  }, { `name`: ``"index"`` ; `type`: ``"u16"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"SetTransactionsArgsV0"`` ; `type`: { `fields`: [{ `name`: ``"choiceIndex"`` ; `type`: ``"u16"``  }, { `name`: ``"transactionIndex"`` ; `type`: ``"u16"``  }, { `docs`: [``"Accounts will come from remaining accounts, which allows for lookup tables"``, ``"and such to reduce size of txn call here"``] ; `name`: ``"instructions"`` ; `type`: { `vec`: { `defined`: ``"CompiledInstruction"``  }  }  }, { `name`: ``"signerSeeds"`` ; `type`: { `vec`: { `vec`: ``"bytes"``  }  }  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"CompiledInstruction"`` ; `type`: { `fields`: [{ `docs`: [``"Index into the transaction keys array indicating the program account that executes this instruction."``] ; `name`: ``"programIdIndex"`` ; `type`: ``"u8"``  }, { `docs`: [``"Ordered indices into the transaction keys array indicating which accounts to pass to the program."``] ; `name`: ``"accounts"`` ; `type`: ``"bytes"``  }, { `docs`: [``"The program input data."``] ; `name`: ``"data"`` ; `type`: ``"bytes"``  }] ; `kind`: ``"struct"``  }  }, { `name`: ``"CompiledTransaction"`` ; `type`: { `fields`: [{ `name`: ``"accounts"`` ; `type`: { `vec`: ``"publicKey"``  }  }, { `name`: ``"instructions"`` ; `type`: { `vec`: { `defined`: ``"CompiledInstruction"``  }  }  }, { `docs`: [``"Additional signer seeds. Should include bump. Useful for things like initializing a mint where"``, ``"you cannot pass a keypair."``, ``"Note that these seeds will be prefixed with \"custom\", org_wallet.index"``, ``"and the bump you pass and account should be consistent with this. But to save space"``, ``"in the instruction, they should be ommitted here. See tests for examples"``] ; `name`: ``"signerSeeds"`` ; `type`: { `vec`: { `vec`: ``"bytes"``  }  }  }] ; `kind`: ``"struct"``  }  }] |
| `version` | ``"0.1.0"`` |

#### Defined in

organization_wallet.ts:1

## Variables

### IDL

• `Const` **IDL**: [`OrganizationWallet`](organization_wallet.md#organizationwallet)

#### Defined in

organization_wallet.ts:446
