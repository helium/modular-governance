# Organization Wallet SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### execute_transaction_v0

#### Accounts

| Name                | Mutability | Signer | Docs |
| ------------------- | ---------- | ------ | ---- |
| organization_wallet | immut      | no     |      |
| proposal            | immut      | no     |      |
| choice_transaction  | immut      | no     |      |
| wallet              | immut      | no     |      |
| refund              | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### initialize_organization_wallet_v0

#### Accounts

| Name                | Mutability | Signer | Docs |
| ------------------- | ---------- | ------ | ---- |
| payer               | immut      | no     |      |
| organization_wallet | immut      | no     |      |
| organization        | immut      | no     |      |
| authority           | immut      | no     |      |
| system_program      | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### set_transactions_v0

#### Accounts

| Name                | Mutability | Signer | Docs |
| ------------------- | ---------- | ------ | ---- |
| payer               | immut      | no     |      |
| owner               | immut      | no     |      |
| organization_wallet | immut      | no     |      |
| proposal            | immut      | no     |      |
| wallet_proposal     | immut      | no     |      |
| choice_transaction  | immut      | no     |      |
| system_program      | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### update_organization_wallet_v0

#### Accounts

| Name                | Mutability | Signer | Docs |
| ------------------- | ---------- | ------ | ---- |
| payer               | immut      | no     |      |
| organization_wallet | immut      | no     |      |
| organization        | immut      | no     |      |
| authority           | immut      | no     |      |
| system_program      | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

### ChoiceTransactionV0

undefined

### OrganizationWalletV0

undefined

### WalletProposalV0

undefined

## Types

### Choice

| Field  | Type   |
| ------ | ------ |
| weight | u128   |
| name   | string |
| uri    | string |

### ChoiceTransactionV0

| Field                    | Type            |
| ------------------------ | --------------- |
| wallet_proposal          | pubkey          |
| proposal                 | pubkey          |
| organization_wallet      | pubkey          |
| choice_index             | u16             |
| allow_execution_offset   | u32             |
| disable_execution_offset | u32             |
| bump_seed                | u8              |
| transaction              | [object Object] |

### CompiledInstructionV0

| Field            | Type  |
| ---------------- | ----- |
| program_id_index | u8    |
| accounts         | bytes |
| data             | bytes |

### CompiledTransactionArgV0

| Field          | Type            |
| -------------- | --------------- |
| num_rw_signers | u8              |
| num_ro_signers | u8              |
| num_rw         | u8              |
| instructions   | [object Object] |
| signer_seeds   | bytes           |

### CompiledTransactionV0

| Field          | Type            |
| -------------- | --------------- |
| num_rw_signers | u8              |
| num_ro_signers | u8              |
| num_rw         | u8              |
| accounts       | pubkey          |
| instructions   | [object Object] |
| signer_seeds   | bytes           |

### InitializeOrganizationWalletArgsV0

| Field            | Type   |
| ---------------- | ------ |
| name             | string |
| proposal_configs | pubkey |
| index            | u16    |

### OrganizationV0

| Field                   | Type   |
| ----------------------- | ------ |
| num_proposals           | u32    |
| authority               | pubkey |
| default_proposal_config | pubkey |
| proposal_program        | pubkey |
| name                    | string |
| uri                     | string |
| bump_seed               | u8     |

### OrganizationWalletV0

| Field            | Type   |
| ---------------- | ------ |
| index            | u16    |
| organization     | pubkey |
| wallet           | pubkey |
| proposal_configs | pubkey |
| name             | string |
| bump_seed        | u8     |
| wallet_bump_seed | u8     |

### ProposalState

| Variant   | Fields                                |
| --------- | ------------------------------------- |
| Draft     |                                       |
| Cancelled |                                       |
| Voting    | start_ts: i64                         |
| Resolved  | choices: [object Object], end_ts: i64 |
| Custom    | name: string, bin: bytes              |

### ProposalV0

| Field                 | Type            |
| --------------------- | --------------- |
| namespace             | pubkey          |
| owner                 | pubkey          |
| state                 | [object Object] |
| created_at            | i64             |
| proposal_config       | pubkey          |
| max_choices_per_voter | u16             |
| seed                  | bytes           |
| name                  | string          |
| uri                   | string          |
| tags                  | string          |
| choices               | [object Object] |
| bump_seed             | u8              |

### SetTransactionsArgsV0

| Field                    | Type            |
| ------------------------ | --------------- |
| choice_index             | u16             |
| transaction_index        | u16             |
| allow_execution_offset   | u32             |
| disable_execution_offset | u32             |
| transaction              | [object Object] |

### UpdateOrganizationWalletArgsV0

| Field            | Type            |
| ---------------- | --------------- |
| name             | string          |
| proposal_configs | [object Object] |

### WalletProposalV0

| Field                      | Type   |
| -------------------------- | ------ |
| proposal                   | pubkey |
| organization_wallet        | pubkey |
| num_transactions_by_choice | u16    |
