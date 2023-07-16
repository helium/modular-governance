[@helium/state-controller-sdk - v0.0.1](README) / Exports

# @helium/state-controller-sdk - v0.0.1

## Table of contents

### Classes

- [SettingsBuilder](classes/SettingsBuilder)

### Variables

- [PROGRAM\_ID](state-controller-sdk#program\_id)

### Functions

- [init](state-controller-sdk#init)
- [resolutionSettingsKey](state-controller-sdk#resolution-settings-key)
- [settings](state-controller-sdk#settings)
- [stateControllerProgramResolver](state-controller-sdk#state-controller-program-resolver)
- [stateControllerResolvers](state-controller-sdk#state-controller-resolvers)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: `PublicKey`

#### Defined in

[packages/state-controller-sdk/src/constants.ts:3](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/constants.ts#L3)

## Functions

### init

▸ **init**(`provider`, `programId?`, `idl?`): `Promise`<`Program`<`StateController`\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | `AnchorProvider` | `undefined` |
| `programId` | `PublicKey` | `PROGRAM_ID` |
| `idl?` | `Idl` | `undefined` |

#### Returns

`Promise`<`Program`<`StateController`\>\>

#### Defined in

[packages/state-controller-sdk/src/index.ts:13](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/index.ts#L13)

___

### resolutionSettingsKey

▸ **resolutionSettingsKey**(`name`, `programId?`): [`PublicKey`, `number`]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `name` | `String` | `undefined` |
| `programId` | `PublicKey` | `PROGRAM_ID` |

#### Returns

[`PublicKey`, `number`]

#### Defined in

[packages/state-controller-sdk/src/pdas.ts:4](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/pdas.ts#L4)

___

### settings

▸ **settings**(): [`SettingsBuilder`](classes/SettingsBuilder)

#### Returns

[`SettingsBuilder`](classes/SettingsBuilder)

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:5](https://github.com/DeWiCats/modular-governance/blob/9f88f14/packages/state-controller-sdk/src/settingsBuilder.ts#L5)

___

### stateControllerProgramResolver

▸ **stateControllerProgramResolver**(`params`): `Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

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

### stateControllerResolvers

▸ **stateControllerResolvers**(`params`): `Promise`<{ `accounts`: `AccountsGeneric` ; `resolved`: `number`  }\>

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
