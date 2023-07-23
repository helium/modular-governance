# State Controller SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### onVoteV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| voteController  | immut      | yes    |      |
| stateController | mut        | no     |      |
| proposal        | immut      | no     |      |
| proposalConfig  | immut      | no     |      |

#### Args

| Name | Type       | Docs |
| ---- | ---------- | ---- |
| args | VoteArgsV0 |      |

### initializeResolutionSettingsV0

#### Accounts

| Name               | Mutability | Signer | Docs |
| ------------------ | ---------- | ------ | ---- |
| payer              | mut        | yes    |      |
| resolutionSettings | mut        | no     |      |
| systemProgram      | immut      | no     |      |

#### Args

| Name | Type                               | Docs |
| ---- | ---------------------------------- | ---- |
| args | InitializeResolutionSettingsArgsV0 |      |

### updateStateV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| owner           | immut      | yes    |      |
| proposal        | mut        | no     |      |
| proposalConfig  | immut      | no     |      |
| stateController | immut      | no     |      |
| proposalProgram | immut      | no     |      |

#### Args

| Name | Type              | Docs |
| ---- | ----------------- | ---- |
| args | UpdateStateArgsV0 |      |

### resolveV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| stateController | mut        | no     |      |
| proposal        | mut        | no     |      |
| proposalConfig  | immut      | no     |      |
| proposalProgram | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

## Accounts

| Name                 | Type            | Docs  |
| -------------------- | --------------- | ----- | ---- |
| ResolutionSettingsV0 |                 | Field | Type |
| -----                | ----            |
| name                 | string          |
| settings             | [object Object] |
| bumpSeed             | u8              |
|                      |

## Types

### InitializeResolutionSettingsArgsV0

| Field    | Type            |
| -------- | --------------- |
| name     | string          |
| settings | [object Object] |

### VoteArgsV0

| Field      | Type |
| ---------- | ---- |
| choice     | u16  |
| weight     | u128 |
| removeVote | bool |

### UpdateStateArgsV0

| Field    | Type            |
| -------- | --------------- |
| newState | [object Object] |

### ResolutionStrategy

| Field | Type            |
| ----- | --------------- |
| nodes | [object Object] |

### ProposalState

| Variant   | Fields        |
| --------- | ------------- |
| Draft     |               |
| Cancelled |               |
| Voting    |               |
| Custom    | state: string |

### ResolutionNode

| Variant           | Fields                   |
| ----------------- | ------------------------ |
| Resolved          | choices: [object Object] |
| EndTimestamp      | end_ts: i64              |
| OffsetFromStartTs | offset: i64              |
| ChoiceVoteWeight  | weight_threshold: u128   |
| ChoicePercentage  | percentage: i32          |
| Top               | n: u16                   |
| And               |                          |
| Or                |                          |