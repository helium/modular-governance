[@helium/organization-sdk](README.md) / Exports

# @helium/organization-sdk

## Table of contents

### Variables

- [PROGRAM\_ID](undefined)

### Functions

- [init](undefined)
- [organizationsProgramResolver](undefined)
- [organizationsResolvers](undefined)
- [proposalKey](undefined)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: PublicKey

#### Defined in

[packages/organization-sdk/src/constants.ts:3](https://github.com/helium/modular-governance/blob/098440d/packages/organization-sdk/src/constants.ts#L3)

## Functions

### init

▸ **init**(`provider`, `programId?`, `idl?`): Promise<Program<Organization\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | AnchorProvider | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |
| `idl?` | Idl | `undefined` |

#### Returns

Promise<Program<Organization\>\>

#### Defined in

[packages/organization-sdk/src/init.ts:7](https://github.com/helium/modular-governance/blob/098440d/packages/organization-sdk/src/init.ts#L7)

___

### organizationsProgramResolver

▸ **organizationsProgramResolver**(`params`): Promise<Object\>

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

### organizationsResolvers

▸ **organizationsResolvers**(`params`): Promise<Object\>

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

### proposalKey

▸ **proposalKey**(`organization`, `index`, `programId?`): [PublicKey, number]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `organization` | PublicKey | `undefined` |
| `index` | number | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |

#### Returns

[PublicKey, number]

#### Defined in

[packages/organization-sdk/src/pdas.ts:4](https://github.com/helium/modular-governance/blob/098440d/packages/organization-sdk/src/pdas.ts#L4)
