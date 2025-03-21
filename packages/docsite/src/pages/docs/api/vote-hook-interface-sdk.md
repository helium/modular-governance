# Vote Hook Interface SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

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

| Name   | Type            | Docs |
| ------ | --------------- | ---- |
| \_args | [object Object] |      |

## Accounts

## Types

### VoteArgsV0

| Field       | Type |
| ----------- | ---- |
| choice      | u16  |
| weight      | u128 |
| remove_vote | bool |
