# Organization SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initializeOrganizationV0

#### Accounts

| Name          | Mutability | Signer | Docs |
| ------------- | ---------- | ------ | ---- |
| payer         | mut        | yes    |      |
| organization  | mut        | no     |      |
| systemProgram | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### initializeProposalV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| payer           | mut        | yes    |      |
| authority       | immut      | yes    |      |
| owner           | immut      | no     |      |
| proposal        | mut        | no     |      |
| proposalConfig  | immut      | no     |      |
| organization    | mut        | no     |      |
| proposalProgram | immut      | no     |      |
| systemProgram   | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

| Name                  | Type      | Docs  |
| --------------------- | --------- | ----- | ---- |
| OrganizationV0        |           | Field | Type |
| -----                 | ----      |
| numProposals          | u32       |
| authority             | publicKey |
| defaultProposalConfig | publicKey |
| proposalProgram       | publicKey |
| name                  | string    |
| uri                   | string    |
| bumpSeed              | u8        |
|                       |

## Types

### InitializeOrganizationArgsV0

| Field                 | Type      |
| --------------------- | --------- |
| name                  | string    |
| authority             | publicKey |
| defaultProposalConfig | publicKey |
| proposalProgram       | publicKey |
| uri                   | string    |

### ChoiceArg

| Field | Type            |
| ----- | --------------- |
| name  | string          |
| uri   | [object Object] |

### InitializeProposalArgsV0

| Field              | Type            |
| ------------------ | --------------- |
| name               | string          |
| uri                | string          |
| maxChoicesPerVoter | u16             |
| choices            | [object Object] |
| tags               | [object Object] |
