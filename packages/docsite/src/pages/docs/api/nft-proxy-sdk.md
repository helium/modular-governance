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

| Name          | Mutability | Signer | Docs                                                                   |
| ------------- | ---------- | ------ | ---------------------------------------------------------------------- |
| payer         | mut        | yes    |                                                                        |
| asset         | immut      | no     |                                                                        |
| approver      | immut      | yes    |                                                                        |
| owner         | immut      | no     | or in the case of a primary proxy (first in the line), Pubkey::default |
| tokenAccount  | immut      | no     |                                                                        |
| proxyConfig   | immut      | no     |                                                                        |
| currentProxy  | mut        | no     |                                                                        |
| recipient     | immut      | no     |                                                                        |
| nextProxy     | mut        | no     |                                                                        |
| systemProgram | immut      | no     |                                                                        |

#### Args

| Name | Type              | Docs |
| ---- | ----------------- | ---- |
| args | AssignProxyArgsV0 |      |

### unassignProxyV0

#### Accounts

| Name          | Mutability | Signer | Docs                                                                   |
| ------------- | ---------- | ------ | ---------------------------------------------------------------------- |
| rentRefund    | mut        | no     |                                                                        |
| asset         | immut      | no     |                                                                        |
| approver      | immut      | yes    |                                                                        |
| owner         | immut      | no     | or in the case of a primary proxy (first in the line), Pubkey::default |
| tokenAccount  | immut      | no     |                                                                        |
| currentProxy  | immut      | no     |                                                                        |
| prevProxy     | mut        | no     |                                                                        |
| proxy         | mut        | no     |                                                                        |
| systemProgram | immut      | no     |                                                                        |

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
| seasons      | i64       |

### ProxyV0

| Field          | Type      |
| -------------- | --------- |
| owner          | publicKey |
| proxyConfig    | publicKey |
| asset          | publicKey |
| index          | u16       |
| nextOwner      | publicKey |
| rentRefund     | publicKey |
| expirationTime | i64       |
| bumpSeed       | u8        |

## Types

### AssignProxyArgsV0

| Field          | Type |
| -------------- | ---- |
| expirationTime | i64  |

### InitializeProxyConfigArgsV0

| Field        | Type   |
| ------------ | ------ |
| name         | string |
| maxProxyTime | i64    |
| seasons      | i64    |
