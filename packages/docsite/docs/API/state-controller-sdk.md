[@helium/state-controller-sdk](README.md) / Exports

# @helium/state-controller-sdk

## Table of contents

### Classes

- [SettingsBuilder](undefined)

### Variables

- [PROGRAM\_ID](undefined)

### Functions

- [init](undefined)
- [resolutionSettingsKey](undefined)
- [settings](undefined)
- [stateControllerProgramResolver](undefined)
- [stateControllerResolvers](undefined)

## Classes

### SettingsBuilder

• **SettingsBuilder**: Class SettingsBuilder

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:9](https://github.com/helium/modular-governance/blob/098440d/packages/state-controller-sdk/src/settingsBuilder.ts#L9)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: PublicKey

#### Defined in

[packages/state-controller-sdk/src/constants.ts:3](https://github.com/helium/modular-governance/blob/098440d/packages/state-controller-sdk/src/constants.ts#L3)

## Functions

### init

▸ **init**(`provider`, `programId?`, `idl?`): Promise<Program<StateController\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | AnchorProvider | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |
| `idl?` | Idl | `undefined` |

#### Returns

Promise<Program<StateController\>\>

#### Defined in

[packages/state-controller-sdk/src/index.ts:13](https://github.com/helium/modular-governance/blob/098440d/packages/state-controller-sdk/src/index.ts#L13)

___

### resolutionSettingsKey

▸ **resolutionSettingsKey**(`name`, `programId?`): [PublicKey, number]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `name` | String | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |

#### Returns

[PublicKey, number]

#### Defined in

[packages/state-controller-sdk/src/pdas.ts:4](https://github.com/helium/modular-governance/blob/098440d/packages/state-controller-sdk/src/pdas.ts#L4)

___

### settings

▸ **settings**(): SettingsBuilder

#### Returns

SettingsBuilder

#### Defined in

[packages/state-controller-sdk/src/settingsBuilder.ts:5](https://github.com/helium/modular-governance/blob/098440d/packages/state-controller-sdk/src/settingsBuilder.ts#L5)

___

### stateControllerProgramResolver

▸ **stateControllerProgramResolver**(`params`): Promise<Object\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `params` | Object |
| `params.accounts` | AccountsGeneric |
| `params.args` | any[] |
| `params.idlIx` | any |
| `params.programId` | PublicKey |
| `params.provider` | default |

#### Returns

Promise<Object\>

#### Defined in

node_modules/@coral-xyz/anchor/dist/cjs/program/accounts-resolver.d.ts:11

___

### stateControllerResolvers

▸ **stateControllerResolvers**(`params`): Promise<Object\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `params` | Object |
| `params.accounts` | AccountsGeneric |
| `params.args` | any[] |
| `params.idlIx` | any |
| `params.programId` | PublicKey |
| `params.provider` | default |

#### Returns

Promise<Object\>

#### Defined in

node_modules/@coral-xyz/anchor/dist/cjs/program/accounts-resolver.d.ts:11
