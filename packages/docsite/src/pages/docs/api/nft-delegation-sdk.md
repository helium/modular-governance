# Nft Delegation SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### delegateV0

#### Accounts

| Name              | Mutability | Signer | Docs |
| ----------------- | ---------- | ------ | ---- |
| payer             | mut        | yes    |      |
| mint              | immut      | no     |      |
| owner             | immut      | yes    |      |
| tokenAccount      | immut      | no     |      |
| currentDelegation | mut        | no     |      |
| recipient         | immut      | no     |      |
| nextDelegation    | mut        | no     |      |
| systemProgram     | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### undelegateV0

#### Accounts

| Name              | Mutability | Signer | Docs |
| ----------------- | ---------- | ------ | ---- |
| rentRefund        | mut        | no     |      |
| owner             | immut      | yes    |      |
| currentDelegation | immut      | no     |      |
| prevDelegation    | mut        | no     |      |
| delegation        | mut        | no     |      |
| systemProgram     | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

## Accounts

### DelegationV0

| Field      | Type      |
| ---------- | --------- |
| owner      | publicKey |
| asset      | publicKey |
| index      | u16       |
| nextOwner  | publicKey |
| rentRefund | publicKey |
| bumpSeed   | u8        |

## Types
