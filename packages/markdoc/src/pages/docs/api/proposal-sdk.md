[@helium/proposal-sdk - v0.0.1](README) / Exports

# @helium/proposal-sdk - v0.0.1

## Table of contents

### Variables

- [PROGRAM\_ID](proposal-sdk#program\_id)

### Functions

- [init](proposal-sdk#init)
- [proposalKey](proposal-sdk#proposal-key)
- [proposalProgramResolver](proposal-sdk#proposal-program-resolver)
- [proposalResolvers](proposal-sdk#proposal-resolvers)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: `PublicKey`

#### Defined in

[packages/proposal-sdk/src/constants.ts:3](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/proposal-sdk/src/constants.ts#L3)

## Functions

### init

▸ **init**(`provider`, `programId?`, `idl?`): `Promise`<`Program`<`Proposal`\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | `AnchorProvider` | `undefined` |
| `programId` | `PublicKey` | `PROGRAM_ID` |
| `idl?` | `Idl` | `undefined` |

#### Returns

`Promise`<`Program`<`Proposal`\>\>

#### Defined in

[packages/proposal-sdk/src/index.ts:13](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/proposal-sdk/src/index.ts#L13)

___

### proposalKey

▸ **proposalKey**(`owner`, `seed`, `programId?`): [`PublicKey`, `number`]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `owner` | `PublicKey` | `undefined` |
| `seed` | `Buffer` | `undefined` |
| `programId` | `PublicKey` | `PROGRAM_ID` |

#### Returns

[`PublicKey`, `number`]

#### Defined in

[packages/proposal-sdk/src/pdas.ts:4](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/proposal-sdk/src/pdas.ts#L4)

___

### proposalProgramResolver

▸ **proposalProgramResolver**(`params`): `Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

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

### proposalResolvers

▸ **proposalResolvers**(`params`): `Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

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
