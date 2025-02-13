# Token Voter SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### deposit_v0

#### Accounts

| Name                      | Mutability | Signer | Docs |
| ------------------------- | ---------- | ------ | ---- |
| token_voter               | immut      | no     |      |
| collection                | immut      | no     |      |
| collection_metadata       | immut      | no     |      |
| collection_master_edition | immut      | no     |      |
| receipt                   | immut      | no     |      |
| mint                      | immut      | no     |      |
| metadata                  | immut      | no     |      |
| master_edition            | immut      | no     |      |
| receipt_token_account     | immut      | no     |      |
| recipient                 | immut      | no     |      |
| vault                     | immut      | no     |      |
| token_account             | immut      | no     |      |
| payer                     | immut      | no     |      |
| deposit_mint              | immut      | no     |      |
| system_program            | immut      | no     |      |
| token_program             | immut      | no     |      |
| associated_token_program  | immut      | no     |      |
| token_metadata_program    | immut      | no     |      |
| rent                      | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### initialize_token_voter_v0

#### Accounts

| Name                     | Mutability | Signer | Docs |
| ------------------------ | ---------- | ------ | ---- |
| payer                    | immut      | no     |      |
| token_voter              | immut      | no     |      |
| collection               | immut      | no     |      |
| metadata                 | immut      | no     |      |
| master_edition           | immut      | no     |      |
| token_account            | immut      | no     |      |
| mint                     | immut      | no     |      |
| rent                     | immut      | no     |      |
| token_program            | immut      | no     |      |
| associated_token_program | immut      | no     |      |
| token_metadata_program   | immut      | no     |      |
| system_program           | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### relinquish_vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs                                              |
| ---------------- | ---------- | ------ | ------------------------------------------------- |
| refund           | immut      | no     | Account to receive sol refund if marker is closed |
| marker           | immut      | no     |                                                   |
| token_voter      | immut      | no     |                                                   |
| voter            | immut      | no     |                                                   |
| receipt          | immut      | no     |                                                   |
| mint             | immut      | no     |                                                   |
| token_account    | immut      | no     |                                                   |
| proposal         | immut      | no     |                                                   |
| proposal_config  | immut      | no     |                                                   |
| state_controller | immut      | no     |                                                   |
| on_vote_hook     | immut      | no     |                                                   |
| proposal_program | immut      | no     |                                                   |
| system_program   | immut      | no     |                                                   |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| payer            | immut      | no     |      |
| marker           | immut      | no     |      |
| token_voter      | immut      | no     |      |
| voter            | immut      | no     |      |
| receipt          | immut      | no     |      |
| mint             | immut      | no     |      |
| token_account    | immut      | no     |      |
| proposal         | immut      | no     |      |
| proposal_config  | immut      | no     |      |
| state_controller | immut      | no     |      |
| on_vote_hook     | immut      | no     |      |
| proposal_program | immut      | no     |      |
| system_program   | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### withdraw_v0

#### Accounts

| Name                     | Mutability | Signer | Docs |
| ------------------------ | ---------- | ------ | ---- |
| token_voter              | immut      | no     |      |
| collection               | immut      | no     |      |
| collection_metadata      | immut      | no     |      |
| receipt                  | immut      | no     |      |
| mint                     | immut      | no     |      |
| metadata                 | immut      | no     |      |
| master_edition           | immut      | no     |      |
| receipt_token_account    | immut      | no     |      |
| vault                    | immut      | no     |      |
| token_account            | immut      | no     |      |
| payer                    | immut      | no     |      |
| refund                   | immut      | no     |      |
| owner                    | immut      | no     |      |
| deposit_mint             | immut      | no     |      |
| system_program           | immut      | no     |      |
| token_program            | immut      | no     |      |
| associated_token_program | immut      | no     |      |
| token_metadata_program   | immut      | no     |      |
| rent                     | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

## Accounts

### ProposalConfigV0

undefined

### ProposalV0

undefined

### ReceiptV0

undefined

### TokenVoterV0

undefined

### VoteMarkerV0

undefined

## Types

### Choice

| Field  | Type   |
| ------ | ------ |
| weight | u128   |
| name   | string |
| uri    | string |

### DepositArgsV0

| Field        | Type   |
| ------------ | ------ |
| amount       | u64    |
| metadata_uri | string |

### InitializeTokenVoterArgsV0

| Field          | Type   |
| -------------- | ------ |
| name           | string |
| authority      | pubkey |
| collection_uri | string |

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

### ReceiptV0

| Field            | Type   |
| ---------------- | ------ |
| token_voter      | pubkey |
| mint             | pubkey |
| amount           | u64    |
| num_active_votes | u64    |
| bump_seed        | u8     |

### RelinquishVoteArgsV0

| Field  | Type |
| ------ | ---- |
| choice | u16  |

### TokenVoterV0

| Field        | Type   |
| ------------ | ------ |
| authority    | pubkey |
| deposit_mint | pubkey |
| collection   | pubkey |
| name         | string |
| bump_seed    | u8     |

### VoteArgsV0

| Field  | Type |
| ------ | ---- |
| choice | u16  |

### VoteMarkerV0

| Field       | Type   |
| ----------- | ------ |
| voter       | pubkey |
| token_voter | pubkey |
| proposal    | pubkey |
| mint        | pubkey |
| choices     | u16    |
| bump_seed   | u8     |
