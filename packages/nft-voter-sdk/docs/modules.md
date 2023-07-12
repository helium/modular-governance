[@helium/nft-voter-sdk](README.md) / Exports

# @helium/nft-voter-sdk

## Table of contents

### Variables

- [PROGRAM\_ID](undefined)

### Functions

- [deposit](undefined)
- [init](undefined)
- [nftVoterProgramResolver](undefined)
- [nftVoterResolvers](undefined)
- [receiptKey](undefined)

## Variables

### PROGRAM\_ID

• `Const` **PROGRAM\_ID**: PublicKey

#### Defined in

[packages/nft-voter-sdk/src/constants.ts:3](https://github.com/helium/modular-governance/blob/098440d/packages/nft-voter-sdk/src/constants.ts#L3)

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

[packages/nft-voter-sdk/src/functions/deposit.ts:36](https://github.com/helium/modular-governance/blob/098440d/packages/nft-voter-sdk/src/functions/deposit.ts#L36)

___

### init

▸ **init**(`provider`, `programId?`, `idl?`): Promise<Program<NftVoter\>\>

Init the NFT Voter Sdk example

```ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PROGRAM_ID, init } from "@helium/nft-voter-sdk";

// run typedoc --help for a list of supported languages
// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env());
const provider = anchor.getProvider() as anchor.AnchorProvider;
const me = provider.wallet.publicKey;
let proposalProgram: Program<Proposal>;
let program: Program<NftVoter>;
program = await init(provider, PROGRAM_ID, anchor.workspace.NftVoter.idl);
```

#### Parameters

| Name | Type | Default value |
| :------ | :------ | :------ |
| `provider` | AnchorProvider | `undefined` |
| `programId` | PublicKey | `PROGRAM_ID` |
| `idl?` | Idl | `undefined` |

#### Returns

Promise<Program<NftVoter\>\>

#### Defined in

[packages/nft-voter-sdk/src/index.ts:30](https://github.com/helium/modular-governance/blob/098440d/packages/nft-voter-sdk/src/index.ts#L30)

___

### nftVoterProgramResolver

▸ **nftVoterProgramResolver**(`params`): Promise<Object\>

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

### nftVoterResolvers

▸ **nftVoterResolvers**(`params`): Promise<Object\>

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

[packages/nft-voter-sdk/src/pdas.ts:4](https://github.com/helium/modular-governance/blob/098440d/packages/nft-voter-sdk/src/pdas.ts#L4)
