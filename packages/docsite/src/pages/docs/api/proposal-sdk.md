# Proposal SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initializeProposalV0

#### Accounts

| Name           | Mutability | Signer | Docs                                                           |
| -------------- | ---------- | ------ | -------------------------------------------------------------- |
| payer          | mut        | yes    |                                                                |
| namespace      | immut      | yes    | Every proposal must have a namespace to prevent seed collision |
| proposal       | mut        | no     |                                                                |
| owner          | immut      | no     |                                                                |
| proposalConfig | immut      | no     |                                                                |
| systemProgram  | immut      | no     |                                                                |

#### Args

| Name | Type                     | Docs |
| ---- | ------------------------ | ---- |
| args | InitializeProposalArgsV0 |      |

### initializeProposalConfigV0

#### Accounts

| Name           | Mutability | Signer | Docs                                                               |
| -------------- | ---------- | ------ | ------------------------------------------------------------------ |
| payer          | mut        | yes    |                                                                    |
| owner          | immut      | yes    | Every proposal config must have an owner to prevent seed collision |
| proposalConfig | mut        | no     |                                                                    |
| systemProgram  | immut      | no     |                                                                    |

#### Args

| Name | Type                           | Docs |
| ---- | ------------------------------ | ---- |
| args | InitializeProposalConfigArgsV0 |      |

### voteV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| voteController  | immut      | yes    |      |
| stateController | mut        | no     |      |
| proposalConfig  | immut      | no     |      |
| proposal        | mut        | no     |      |
| onVoteHook      | immut      | no     |      |

#### Args

| Name | Type       | Docs |
| ---- | ---------- | ---- |
| args | VoteArgsV0 |      |

### updateStateV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| stateController | immut      | yes    |      |
| proposal        | mut        | no     |      |
| proposalConfig  | immut      | no     |      |

#### Args

| Name | Type              | Docs |
| ---- | ----------------- | ---- |
| args | UpdateStateArgsV0 |      |

## Accounts

| Name               | Type            | Docs  |
| ------------------ | --------------- | ----- | ---- |
| ProposalConfigV0   |                 | Field | Type |
| -----              | ----            |
| voteController     | publicKey       |
| stateController    | publicKey       |
| onVoteHook         | publicKey       |
| name               | string          |
| bumpSeed           | u8              |
|                    |
| ProposalV0         |                 | Field | Type |
| -----              | ----            |
| namespace          | publicKey       |
| owner              | publicKey       |
| state              | [object Object] |
| createdAt          | i64             |
| proposalConfig     | publicKey       |
| maxChoicesPerVoter | u16             |
| seed               | bytes           |
| name               | string          |
| uri                | string          |
| tags               | [object Object] |
| choices            | [object Object] |
| bumpSeed           | u8              |
|                    |

## Types

### InitializeProposalConfigArgsV0

| Field           | Type      |
| --------------- | --------- |
| name            | string    |
| voteController  | publicKey |
| stateController | publicKey |
| onVoteHook      | publicKey |

### ChoiceArg

| Field | Type            |
| ----- | --------------- |
| name  | string          |
| uri   | [object Object] |

### InitializeProposalArgsV0

| Field              | Type            |
| ------------------ | --------------- |
| seed               | bytes           |
| name               | string          |
| uri                | string          |
| maxChoicesPerVoter | u16             |
| choices            | [object Object] |
| tags               | [object Object] |

### UpdateStateArgsV0

| Field    | Type            |
| -------- | --------------- |
| newState | [object Object] |

### VoteArgsV0

| Field      | Type |
| ---------- | ---- |
| choice     | u16  |
| weight     | u128 |
| removeVote | bool |

### Choice

| Field  | Type            |
| ------ | --------------- |
| weight | u128            |
| name   | string          |
| uri    | [object Object] |

### ProposalState

| Variant   | Fields                   |
| --------- | ------------------------ |
| Draft     |                          |
| Cancelled |                          |
| Voting    | start_ts: i64            |
| Resolved  | choices: [object Object] |
| Custom    | state: string            |
