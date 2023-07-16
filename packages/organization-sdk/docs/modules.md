[@helium/organization-sdk - v0.0.1](README) / Exports

# @helium/organization-sdk - v0.0.1

## Table of contents

### Variables

- [PROGRAM\_ID](organization-sdk#program\_id)

### Functions

- [init](organization-sdk#init)
- [organizationKey](organization-sdk#organization-key)
- [organizationsProgramResolver](organization-sdk#organizations-program-resolver)
- [organizationsResolvers](organization-sdk#organizations-resolvers)
- [proposalKey](organization-sdk#proposal-key)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: `PublicKey`

#### Defined in

[packages/organization-sdk/src/constants.ts:3](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/organization-sdk/src/constants.ts#L3)

## Functions

### init

▸ **init**(`provider`, `programId?`, `idl?`): `Promise`<`Program`<`Organization`\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | `AnchorProvider` | `undefined` |
| `programId` | `PublicKey` | `PROGRAM_ID` |
| `idl?` | `Idl` | `undefined` |

#### Returns

`Promise`<`Program`<`Organization`\>\>

#### Defined in

[packages/organization-sdk/src/init.ts:7](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/organization-sdk/src/init.ts#L7)

___

### organizationKey

▸ **organizationKey**(`name`, `programId?`): [`PublicKey`, `number`]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `name` | `String` | `undefined` |
| `programId` | `PublicKey` | `PROGRAM_ID` |

#### Returns

[`PublicKey`, `number`]

#### Defined in

[packages/organization-sdk/src/pdas.ts:18](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/organization-sdk/src/pdas.ts#L18)

___

### organizationsProgramResolver

▸ **organizationsProgramResolver**(`params`): `Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `params` | `Object` |
| `params.accounts` | `AccountsGeneric` |
| `params.args` | `any`[] |
| `params.idlIx` | `any` |
| `params.programId` | `PublicKey` |
| `params.provider` | `default` |

#### Returns

`Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

#### Defined in

node_modules/@coral-xyz/anchor/dist/cjs/program/accounts-resolver.d.ts:11

___

### organizationsResolvers

▸ **organizationsResolvers**(`params`): `Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `params` | `Object` |
| `params.accounts` | `AccountsGeneric` |
| `params.args` | `any`[] |
| `params.idlIx` | `any` |
| `params.programId` | `PublicKey` |
| `params.provider` | `default` |

#### Returns

`Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

#### Defined in

node_modules/@coral-xyz/anchor/dist/cjs/program/accounts-resolver.d.ts:11

___

### proposalKey

▸ **proposalKey**(`organization`, `index`, `programId?`): [`PublicKey`, `number`]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `organization` | `PublicKey` | `undefined` |
| `index` | `number` | `undefined` |
| `programId` | `PublicKey` | `PROPOSAL_PROGRAM_ID` |

#### Returns

[`PublicKey`, `number`]

#### Defined in

[packages/organization-sdk/src/pdas.ts:5](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/organization-sdk/src/pdas.ts#L5)
