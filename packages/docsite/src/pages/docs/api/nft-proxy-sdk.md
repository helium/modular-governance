# Nft Proxy SDK

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide. We also have some react examples [here](/docs/learn/react).
{% /callout %}

## Instructions

### assign_proxy_v0

#### Accounts

| Name                     | Mutability | Signer | Docs                                                                   |
| ------------------------ | ---------- | ------ | ---------------------------------------------------------------------- |
| payer                    | immut      | no     |                                                                        |
| asset                    | immut      | no     |                                                                        |
| approver                 | immut      | no     |                                                                        |
| voter                    | immut      | no     | or in the case of a primary proxy (first in the line), Pubkey::default |
| token_account            | immut      | no     |                                                                        |
| proxy_config             | immut      | no     |                                                                        |
| current_proxy_assignment | immut      | no     |                                                                        |
| recipient                | immut      | no     |                                                                        |
| next_proxy_assignment    | immut      | no     |                                                                        |
| system_program           | immut      | no     |                                                                        |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### close_expired_proxy_v0

#### Accounts

| Name             | Mutability | Signer | Docs |
| ---------------- | ---------- | ------ | ---- |
| rent_refund      | immut      | no     |      |
| proxy_assignment | immut      | no     |      |
| system_program   | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### initialize_proxy_config_v0

#### Accounts

| Name           | Mutability | Signer | Docs |
| -------------- | ---------- | ------ | ---- |
| payer          | immut      | no     |      |
| authority      | immut      | no     |      |
| proxy_config   | immut      | no     |      |
| system_program | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

### unassign_expired_proxy_v0

#### Accounts

| Name                  | Mutability | Signer | Docs |
| --------------------- | ---------- | ------ | ---- |
| rent_refund           | immut      | no     |      |
| prev_proxy_assignment | immut      | no     |      |
| proxy_assignment      | immut      | no     |      |
| system_program        | immut      | no     |      |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### unassign_proxy_v0

#### Accounts

| Name                     | Mutability | Signer | Docs                                                                   |
| ------------------------ | ---------- | ------ | ---------------------------------------------------------------------- |
| rent_refund              | immut      | no     |                                                                        |
| asset                    | immut      | no     |                                                                        |
| approver                 | immut      | no     |                                                                        |
| voter                    | immut      | no     | or in the case of a primary proxy (first in the line), Pubkey::default |
| token_account            | immut      | no     |                                                                        |
| current_proxy_assignment | immut      | no     |                                                                        |
| prev_proxy_assignment    | immut      | no     |                                                                        |
| proxy_assignment         | immut      | no     |                                                                        |
| proxy_config             | immut      | no     |                                                                        |
| system_program           | immut      | no     |                                                                        |

#### Args

| Name | Type | Docs |
| ---- | ---- | ---- |

### update_proxy_config_v0

#### Accounts

| Name           | Mutability | Signer | Docs |
| -------------- | ---------- | ------ | ---- |
| payer          | immut      | no     |      |
| authority      | immut      | no     |      |
| proxy_config   | immut      | no     |      |
| system_program | immut      | no     |      |

#### Args

| Name | Type            | Docs |
| ---- | --------------- | ---- |
| args | [object Object] |      |

## Accounts

### ProxyAssignmentV0

undefined

### ProxyConfigV0

undefined

## Types

### AssignProxyArgsV0

| Field           | Type |
| --------------- | ---- |
| expiration_time | i64  |

### InitializeProxyConfigArgsV0

| Field          | Type            |
| -------------- | --------------- |
| name           | string          |
| max_proxy_time | i64             |
| seasons        | [object Object] |

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

### SeasonV0

| Field | Type |
| ----- | ---- |
| start | i64  |
| end   | i64  |

### UpdateProxyConfigArgsV0

| Field          | Type            |
| -------------- | --------------- |
| max_proxy_time | i64             |
| seasons        | [object Object] |
