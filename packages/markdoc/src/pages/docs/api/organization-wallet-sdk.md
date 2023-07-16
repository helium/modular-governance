# Organization Wallet SDK

## Instructions

### initializeOrganizationWalletV0

#### Accounts

| Name               | Mutability | Signer | Docs |
| ------------------ | ---------- | ------ | ---- |
| payer              | mut        | yes    |      |
| organizationWallet | mut        | no     |      |
| organization       | immut      | no     |      |
| systemProgram      | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### initializeWalletProposalV0

#### Accounts

| Name               | Mutability | Signer | Docs |
| ------------------ | ---------- | ------ | ---- |
| payer              | mut        | yes    |      |
| organizationWallet | immut      | no     |      |
| authority          | immut      | yes    |      |
| owner              | immut      | no     |      |
| proposal           | immut      | no     |      |
| walletProposal     | mut        | no     |      |
| systemProgram      | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### setTransactionsV0

#### Accounts

| Name           | Mutability | Signer | Docs |
| -------------- | ---------- | ------ | ---- |
| payer          | mut        | yes    |      |
| authority      | immut      | yes    |      |
| owner          | immut      | no     |      |
| proposal       | immut      | no     |      |
| walletProposal | mut        | no     |      |
| systemProgram  | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

| Name                        | Type            | Docs  |
| --------------------------- | --------------- | ----- | ---- |
| OrganizationWalletV0        |                 | Field | Type |
| -----                       | ----            |
| index                       | u16             |
| organization                | publicKey       |
| wallet                      | publicKey       |
| proposalConfigs             | [object Object] |
| name                        | string          |
| bumpSeed                    | u8              |
| walletBumpSeed              | u8              |
|                             |
| WalletProposalV0            |                 | Field | Type |
| -----                       | ----            |
| proposal                    | publicKey       |
| organizationWallet          | publicKey       |
| bumpSeed                    | u8              |
| choiceTransactions          | [object Object] |
|                             |
| OrganizationWalletPropoalV0 |                 | Field | Type |
| -----                       | ----            |
| organizationWallet          | publicKey       |
| proposal                    | publicKey       |
| accounts                    | [object Object] |
| transactionsByChoice        | [object Object] |
| bumpSeed                    | u8              |
|                             |

## Types

### ExecuteTransactionArgsV0

| Field       | Type |
| ----------- | ---- |
| choice      | u16  |
| transaction | u16  |

### InitializeOrganizationWalletArgsV0

| Field           | Type            |
| --------------- | --------------- |
| name            | string          |
| authority       | publicKey       |
| proposalConfigs | [object Object] |
| index           | u16             |

### SetTransactionsArgsV0

| Field            | Type            |
| ---------------- | --------------- |
| choiceIndex      | u16             |
| transactionIndex | u16             |
| instructions     | [object Object] |
| signerSeeds      | [object Object] |

### CompiledInstruction

| Field          | Type  |
| -------------- | ----- |
| programIdIndex | u8    |
| accounts       | bytes |
| data           | bytes |

### CompiledTransaction

| Field        | Type            |
| ------------ | --------------- |
| accounts     | [object Object] |
| instructions | [object Object] |
| signerSeeds  | [object Object] |
