# Organization SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initialize_organization_v0

#### Accounts

| Name           | Mutability | Signer | Docs |
| -------------- | ---------- | ------ | ---- |
| payer          | immut      | no     |      |
| organization   | immut      | no     |      |
| system_program | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### initialize_proposal_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| payer            | immut      | no     |      |
| authority        | immut      | no     |      |
| owner            | immut      | no     |      |
| proposal         | immut      | no     |      |
| proposal_config  | immut      | no     |      |
| organization     | immut      | no     |      |
| proposal_program | immut      | no     |      |
| system_program   | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### update_organization_v0

#### Accounts

| Name         | Mutability | Signer | Docs |
| ------------ | ---------- | ------ | ---- |
| organization | immut      | no     |      |
| authority    | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

### OrganizationV0

undefined

## Types

### ChoiceArg

| Field | Type   |
| ----- | ------ |
| name  | string |
| uri   | string |

### InitializeOrganizationArgsV0

| Field                   | Type   |
| ----------------------- | ------ |
| name                    | string |
| authority               | pubkey |
| default_proposal_config | pubkey |
| proposal_program        | pubkey |
| uri                     | string |

### InitializeProposalArgsV0

| Field                 | Type            |
| --------------------- | --------------- |
| name                  | string          |
| uri                   | string          |
| max_choices_per_voter | u16             |
| choices               | [object Object] |
| tags                  | string          |

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

### UpdateOrganizationArgsV0

| Field                   | Type   |
| ----------------------- | ------ |
| authority               | pubkey |
| default_proposal_config | pubkey |
| proposal_program        | pubkey |
| uri                     | string |
