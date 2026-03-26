# Nft Voter SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initialize_nft_voter_v0

#### Accounts

| Name           | Mutability | Signer | Docs |
| -------------- | ---------- | ------ | ---- |
| payer          | immut      | no     |      |
| proxy_config   | immut      | no     |      |
| nft_voter      | immut      | no     |      |
| collection     | immut      | no     |      |
| system_program | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### proxied_relinquish_vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs                                                   |
| ---------------- | ---------- | ------ | ------------------------------------------------------ |
| rent_refund      | immut      | no     | Account to receive sol rent_refund if marker is closed |
| marker           | immut      | no     |                                                        |
| nft_voter        | immut      | no     |                                                        |
| voter            | immut      | no     |                                                        |
| mint             | immut      | no     |                                                        |
| metadata         | immut      | no     |                                                        |
| proxy_assignment | immut      | no     |                                                        |
| proposal         | immut      | no     |                                                        |
| proposal_config  | immut      | no     |                                                        |
| state_controller | immut      | no     |                                                        |
| on_vote_hook     | immut      | no     |                                                        |
| proposal_program | immut      | no     |                                                        |
| system_program   | immut      | no     |                                                        |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### proxied_vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| payer            | immut      | no     |      |
| marker           | immut      | no     |      |
| proxy_assignment | immut      | no     |      |
| nft_voter        | immut      | no     |      |
| voter            | immut      | no     |      |
| mint             | immut      | no     |      |
| metadata         | immut      | no     |      |
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

### relinquish_vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs                                              |
| ---------------- | ---------- | ------ | ------------------------------------------------- |
| rent_refund      | immut      | no     | Account to receive sol refund if marker is closed |
| marker           | immut      | no     |                                                   |
| nft_voter        | immut      | no     |                                                   |
| voter            | immut      | no     |                                                   |
| mint             | immut      | no     |                                                   |
| metadata         | immut      | no     |                                                   |
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

### update_nft_voter_v0

#### Accounts

| Name          | Mutability | Signer | Docs |
| ------------- | ---------- | ------ | ---- |
| authority     | immut      | no     |      |
| proxy_config  | immut      | no     |      |
| new_authority | immut      | no     |      |
| nft_voter     | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### vote_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| payer            | immut      | no     |      |
| marker           | immut      | no     |      |
| nft_voter        | immut      | no     |      |
| voter            | immut      | no     |      |
| mint             | immut      | no     |      |
| metadata         | immut      | no     |      |
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

## Accounts

### NftVoterV0

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

### InitializeNftVoterArgsV0

| Field     | Type   |
| --------- | ------ |
| name      | string |
| authority | pubkey |

### NftVoterV0

| Field        | Type   |
| ------------ | ------ |
| authority    | pubkey |
| collection   | pubkey |
| name         | string |
| bump_seed    | u8     |
| proxy_config | pubkey |

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

### ProxyAssignmentV0

| Field           | Type   |
| --------------- | ------ |
| voter           | pubkey |
| proxy_config    | pubkey |
| asset           | pubkey |
| index           | u16    |
| next_voter      | pubkey |
| rent_refund     | pubkey |
| expiration_time | i64    |
| bump_seed       | u8     |

### ProxyConfigV0

| Field          | Type            |
| -------------- | --------------- |
| authority      | pubkey          |
| name           | string          |
| max_proxy_time | i64             |
| seasons        | [object Object] |

### RelinquishVoteArgsV0

| Field  | Type |
| ------ | ---- |
| choice | u16  |

### SeasonV0

| Field | Type |
| ----- | ---- |
| start | i64  |
| end   | i64  |

### VoteArgsV0

| Field  | Type |
| ------ | ---- |
| choice | u16  |

### VoteMarkerV0

| Field       | Type   |
| ----------- | ------ |
| voter       | pubkey |
| nft_voter   | pubkey |
| proposal    | pubkey |
| mint        | pubkey |
| choices     | u16    |
| bump_seed   | u8     |
| proxy_index | u16    |
| rent_refund | pubkey |
