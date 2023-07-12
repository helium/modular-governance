[@helium/token-voter-sdk](README.md) / Exports

# @helium/token-voter-sdk

## Table of contents

### Variables

- [PROGRAM\_ID](undefined)

### Functions

- [deposit](undefined)
- [init](undefined)
- [receiptKey](undefined)
- [tokenVoterProgramResolver](undefined)
- [tokenVoterResolvers](undefined)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: PublicKey

#### Defined in

[packages/token-voter-sdk/src/constants.ts:3](https://github.com/helium/modular-governance/blob/098440d/packages/token-voter-sdk/src/constants.ts#L3)

## Functions

### deposit

▸ **deposit**(`«destructured»`): Promise<any\>

#### Parameters

| Name | Type |
| :------ | :------ |
| `«destructured»` | any |

#### Returns

Promise<any\>

#### Defined in

[packages/token-voter-sdk/src/functions/deposit.ts:36](https://github.com/helium/modular-governance/blob/098440d/packages/token-voter-sdk/src/functions/deposit.ts#L36)

___

### init

▸ **init**(`provider`, `programId?`, `idl?`): Promise<Program<TokenVoter\>\>

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | AnchorProvider | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |
| `idl?` | Idl | `undefined` |

#### Returns

Promise<Program<TokenVoter\>\>

#### Defined in

[packages/token-voter-sdk/src/index.ts:13](https://github.com/helium/modular-governance/blob/098440d/packages/token-voter-sdk/src/index.ts#L13)

___

### receiptKey

▸ **receiptKey**(`mint`, `programId?`): [PublicKey, number]

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `mint` | PublicKey | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |

#### Returns

[PublicKey, number]

#### Defined in

[packages/token-voter-sdk/src/pdas.ts:4](https://github.com/helium/modular-governance/blob/098440d/packages/token-voter-sdk/src/pdas.ts#L4)

___

### tokenVoterProgramResolver

▸ **tokenVoterProgramResolver**(`params`): Promise<Object\>

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

### tokenVoterResolvers

▸ **tokenVoterResolvers**(`params`): Promise<Object\>

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
