# State Controller SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initialize_resolution_settings_v0

#### Accounts

| Name                | Mutability | Signer | Docs |
| ------------------- | ---------- | ------ | ---- |
| payer               | immut      | no     |      |
| resolution_settings | immut      | no     |      |
| system_program      | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### on_vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| voter            | immut      | no     |      |
| vote_controller  | immut      | no     |      |
| state_controller | immut      | no     |      |
| proposal         | immut      | no     |      |
| proposal_config  | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### resolve_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| state_controller | immut      | no     |      |
| proposal         | immut      | no     |      |
| proposal_config  | immut      | no     |      |
| proposal_program | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### update_state_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| owner            | immut      | no     |      |
| proposal         | immut      | no     |      |
| proposal_config  | immut      | no     |      |
| state_controller | immut      | no     |      |
| proposal_program | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

### ProposalConfigV0

undefined

### ProposalV0

undefined

### ResolutionSettingsV0

undefined

## Types

### Choice

| Field  | Type   |
| ------ | ------ |
| weight | u128   |
| name   | string |
| uri    | string |

### InitializeResolutionSettingsArgsV0

| Field    | Type            |
| -------- | --------------- |
| name     | string          |
| settings | [object Object] |

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

### ResolutionNode

| Variant                   | Fields                   |
| ------------------------- | ------------------------ |
| Resolved                  | choices: [object Object] |
| EndTimestamp              | end_ts: i64              |
| OffsetFromStartTs         | offset: i64              |
| ChoiceVoteWeight          | weight_threshold: u128   |
| ChoicePercentage          | percentage: i32          |
| Top                       | n: u16                   |
| NumResolved               | n: u16                   |
| And                       |                          |
| Or                        |                          |
| Not                       | choice_name: string      |
| TotalWeight               | weight_threshold: u128   |
| ChoicePercentageOfCurrent | percentage: i32          |

### ResolutionSettingsV0

| Field     | Type            |
| --------- | --------------- |
| name      | string          |
| settings  | [object Object] |
| bump_seed | u8              |

### ResolutionStrategy

| Field | Type            |
| ----- | --------------- |
| nodes | [object Object] |

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
