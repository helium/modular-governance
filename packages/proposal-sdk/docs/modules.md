[@helium/proposal-sdk](README.md) / Exports

# @helium/proposal-sdk

## Table of contents

### Variables

- [PROGRAM\_ID](undefined)

### Functions

- [init](undefined)
- [proposalKey](undefined)
- [proposalProgramResolver](undefined)
- [proposalResolvers](undefined)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: PublicKey

#### Defined in

[packages/proposal-sdk/src/constants.ts:3](https://github.com/helium/modular-governance/blob/098440d/packages/proposal-sdk/src/constants.ts#L3)

## Functions

### init

▸ **init**(`provider`, `programId?`, `idl?`): Promise<Program<Proposal\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | AnchorProvider | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |
| `idl?` | Idl | `undefined` |

#### Returns

Promise<Program<Proposal\>\>

#### Defined in

[packages/proposal-sdk/src/index.ts:13](https://github.com/helium/modular-governance/blob/098440d/packages/proposal-sdk/src/index.ts#L13)

___

### proposalKey

▸ **proposalKey**(`owner`, `seed`, `programId?`): [PublicKey, number]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `owner` | PublicKey | `undefined` |
| `seed` | Buffer | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |

#### Returns

[PublicKey, number]

#### Defined in

[packages/proposal-sdk/src/pdas.ts:4](https://github.com/helium/modular-governance/blob/098440d/packages/proposal-sdk/src/pdas.ts#L4)

___

### proposalProgramResolver

▸ **proposalProgramResolver**(`params`): Promise<Object\>

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

### proposalResolvers

▸ **proposalResolvers**(`params`): Promise<Object\>

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
