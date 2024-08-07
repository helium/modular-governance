# Nft Proxy SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### initializeProxyConfigV0

#### Accounts

| Name          | Mutability | Signer | Docs |
| ------------- | ---------- | ------ | ---- |
| payer         | mut        | yes    |      |
| authority     | immut      | no     |      |
| proxyConfig   | mut        | no     |      |
| systemProgram | immut      | no     |      |

#### Args

| Name | Type                        | Docs |
| ---- | --------------------------- | ---- |
| args | InitializeProxyConfigArgsV0 |      |

### assignProxyV0

#### Accounts

| Name                   | Mutability | Signer | Docs                                                                   |
| ---------------------- | ---------- | ------ | ---------------------------------------------------------------------- |
| payer                  | mut        | yes    |                                                                        |
| asset                  | immut      | no     |                                                                        |
| approver               | immut      | yes    |                                                                        |
| voter                  | immut      | no     | or in the case of a primary proxy (first in the line), Pubkey::default |
| tokenAccount           | immut      | no     |                                                                        |
| proxyConfig            | immut      | no     |                                                                        |
| currentProxyAssignment | mut        | no     |                                                                        |
| recipient              | immut      | no     |                                                                        |
| nextProxyAssignment    | mut        | no     |                                                                        |
| systemProgram          | immut      | no     |                                                                        |

#### Args

| Name | Type              | Docs |
| ---- | ----------------- | ---- |
| args | AssignProxyArgsV0 |      |

### unassignProxyV0

#### Accounts

| Name                   | Mutability | Signer | Docs                                                                   |
| ---------------------- | ---------- | ------ | ---------------------------------------------------------------------- |
| rentRefund             | mut        | no     |                                                                        |
| asset                  | immut      | no     |                                                                        |
| approver               | immut      | yes    |                                                                        |
| voter                  | immut      | no     | or in the case of a primary proxy (first in the line), Pubkey::default |
| tokenAccount           | immut      | no     |                                                                        |
| currentProxyAssignment | immut      | no     |                                                                        |
| prevProxyAssignment    | mut        | no     |                                                                        |
| proxyAssignment        | mut        | no     |                                                                        |
| proxyConfig            | immut      | no     |                                                                        |
| systemProgram          | immut      | no     |                                                                        |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### updateProxyConfigV0

#### Accounts

| Name          | Mutability | Signer | Docs |
| ------------- | ---------- | ------ | ---- |
| payer         | mut        | yes    |      |
| authority     | immut      | no     |      |
| proxyConfig   | mut        | no     |      |
| systemProgram | immut      | no     |      |

#### Args

| Name | Type                    | Docs |
| ---- | ----------------------- | ---- |
| args | UpdateProxyConfigArgsV0 |      |

### unassignExpiredProxyV0

#### Accounts

| Name                | Mutability | Signer | Docs |
| ------------------- | ---------- | ------ | ---- |
| rentRefund          | mut        | no     |      |
| prevProxyAssignment | mut        | no     |      |
| proxyAssignment     | mut        | no     |      |
| systemProgram       | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### closeExpiredProxyV0

#### Accounts

| Name            | Mutability | Signer | Docs |
| --------------- | ---------- | ------ | ---- |
| rentRefund      | mut        | no     |      |
| proxyAssignment | mut        | no     |      |
| systemProgram   | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

## Accounts

### ProxyConfigV0

| Field        | Type      |
| ------------ | --------- |
| authority    | publicKey |
| name         | string    |
| maxProxyTime | i64       |
| seasons      | SeasonV0  |

### ProxyAssignmentV0

| Field          | Type      |
| -------------- | --------- |
| voter          | publicKey |
| proxyConfig    | publicKey |
| asset          | publicKey |
| index          | u16       |
| nextVoter      | publicKey |
| rentRefund     | publicKey |
| expirationTime | i64       |
| bumpSeed       | u8        |

## Types

### AssignProxyArgsV0

| Field          | Type |
| -------------- | ---- |
| expirationTime | i64  |

### InitializeProxyConfigArgsV0

| Field        | Type     |
| ------------ | -------- |
| name         | string   |
| maxProxyTime | i64      |
| seasons      | SeasonV0 |

### UpdateProxyConfigArgsV0

| Field        | Type            |
| ------------ | --------------- |
| maxProxyTime | i64             |
| seasons      | [object Object] |

### SeasonV0

| Field | Type |
| ----- | ---- |
| start | i64  |
| end   | i64  |
