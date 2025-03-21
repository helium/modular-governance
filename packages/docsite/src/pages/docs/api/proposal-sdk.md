# Proposal SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initialize_proposal_config_v0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| payer           | immut      | no     |      |
| owner           | immut      | no     |      |
| proposal_config | immut      | no     |      |
| system_program  | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### initialize_proposal_v0

#### Accounts

| Name            | Mutability | Signer | Docs                                                           |
| --------------- | ---------- | ------ | -------------------------------------------------------------- |
| payer           | immut      | no     |                                                                |
| namespace       | immut      | no     | Every proposal must have a namespace to prevent seed collision |
| proposal        | immut      | no     |                                                                |
| owner           | immut      | no     |                                                                |
| proposal_config | immut      | no     |                                                                |
| system_program  | immut      | no     |                                                                |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### update_proposal_config_v0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| proposal_config | immut      | no     |      |
| authority       | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### update_state_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| state_controller | immut      | no     |      |
| proposal         | immut      | no     |      |
| proposal_config  | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| vote_controller  | immut      | no     |      |
| voter            | immut      | no     |      |
| state_controller | immut      | no     |      |
| proposal_config  | immut      | no     |      |
| proposal         | immut      | no     |      |
| on_vote_hook     | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

### ProposalConfigV0

undefined

### ProposalV0

undefined

## Types

### Choice

| Field  | Type   |
| ------ | ------ |
| weight | u128   |
| name   | string |
| uri    | string |

### ChoiceArg

| Field | Type   |
| ----- | ------ |
| name  | string |
| uri   | string |

### InitializeProposalArgsV0

| Field                 | Type            |
| --------------------- | --------------- |
| seed                  | bytes           |
| name                  | string          |
| uri                   | string          |
| max_choices_per_voter | u16             |
| choices               | [object Object] |
| tags                  | string          |

### InitializeProposalConfigArgsV0

| Field            | Type   |
| ---------------- | ------ |
| name             | string |
| vote_controller  | pubkey |
| state_controller | pubkey |
| on_vote_hook     | pubkey |
| authority        | pubkey |

### ProposalConfigV0

| Field            | Type   |
| ---------------- | ------ |
| vote_controller  | pubkey |
| state_controller | pubkey |
| on_vote_hook     | pubkey |
| name             | string |
| bump_seed        | u8     |
| authority        | pubkey |

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

### UpdateProposalConfigArgsV0

| Field            | Type   |
| ---------------- | ------ |
| vote_controller  | pubkey |
| state_controller | pubkey |
| on_vote_hook     | pubkey |
| authority        | pubkey |

### UpdateStateArgsV0

| Field     | Type            |
| --------- | --------------- |
| new_state | [object Object] |

### VoteArgsV0

| Field       | Type |
| ----------- | ---- |
| choice      | u16  |
| weight      | u128 |
| remove_vote | bool |
