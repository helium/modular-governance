---
sidebar_position: 1
slug: /
---

# Getting Started

Helium Modular Governance lets you effortlessly DAOs to manage your communities on Solana!

Looking to learn more about launching a DAO, creating proposals, and unleashing your community? Read on.

## Initializing the SDK

Every smart contract on Helium Modular Governance comes with an SDK. The six main smart contracts are `nft-voter-sdk`, `organization`, `organization_wallet`, `proposal`, `state_controller`, and `token_voter`.

Let's get started by installing the sdks

```shell
yarn add @helium/nft-voter-sdk @helium/organization-sdk @helium/organization-wallet-sdk @helium/proposal-sdk @helium/state-controller-sdk @helium/token-voter-sdk
```

Now, we can initialize the sdks:

```jsx
import * as anchor from '@coral-xyz/anchor'
import { Program } from '@coral-xyz/anchor'
import { PublicKey } from '@solana/web3.js'
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from '@helium/proposal-sdk'
import { PROGRAM_ID, init, proposalKey } from '@helium/organization-sdk'

const provider = anchor.getProvider()

// Create unique name
const name = `DAO Test ${Math.floor(Math.random() * 1000)}`
const organizationSdk = await init(provider, PROGRAM_ID)
const proposalSdk = await initProposal(provider, PROPOSAL_PID)
```

## Creating an Organization

:::info Live Code
You can run and edit all of the code blocks in this tutorial against Solana devnet! The above block contains all of the needed imports.

Use `var` instead of `let` or `const` so that these blocks can be re-run
:::

Let's create an organization named TEST

### Creating an organization with a default proposal config

```jsx async name=create_organization
var {
  pubkeys: { organization },
} = await organizationSdk.methods.initializeOrganizationV0({
  name,
  authority: provider.wallet.publicKey,
  defaultProposalConfig: PublicKey.default,
  proposalProgram: proposalSdk.programId,
  uri: 'https://example.com',
})
```

### Creating an organization with a proposal config

When creating an organization you also can specify a default proposal config. This config will be used for all proposals created by the organization.

```jsx async name=create_organization
var {
  pubkeys: { proposalConfig },
} = await proposalSdk.methods.initializeProposalConfigV0({
  name,
  voteController: provider.wallet.publicKey,
  stateController: provider.wallet.publicKey,
  onVoteHook: PublicKey.default,
})

var {
    pubkeys: { organization },
} = await organizationSdk.methods.initializeOrganizationV0({
    name,
    authority: provider.wallet.publicKey,
    defaultProposalConfig: proposalConfig!,
    proposalProgram: proposalSdk.programId,
    uri: "https://example.com",
})
```

We can fetch the accounts we just created

```jsx async name=fetch deps=create_organization
var acct = await program.account.organizationV0.fetch(organization!);
```

## Creating a proposal

Now, we can create a proposal with the default config:

```jsx async name=create_proposal deps=fetch
var {
  pubkeys: { proposal },
} = await program.methods
  .organizationSdk({
    maxChoicesPerVoter: 1,
    name: 'Proposal Test',
    uri: 'https://example.com',
    choices: [
      {
        name: 'Yes',
        uri: null,
      },
      {
        name: 'No',
        uri: null,
      },
    ],
    tags: ['test', 'tags'],
  })
  .accounts({ organization, proposalConfig })
```

Now we can fetch the proposal we just created:

```jsx async name=fetch_proposal deps=fetch
var acct = await proposalProgram.account.proposalV0.fetch(proposal!);
```

## Voting on a proposal

Now, we can vote on the proposal:

```jsx async name=vote deps=fetch_proposal
await proposalSdk.methods
  .voteV0({
    choice: 1,
    weight: new anchor.BN(2),
    removeVote: false,
  })
  .accounts({ proposal, voter: provider.wallet.publicKey })
```

## Updating the state on a proposal

Now, we can progress the state on the proposal using the state controller:

```jsx async name=progress deps=vote
await proposalSdk.methods
  .updateStateV0({
    newState: { custom: { state: 'hello' } },
  })
  .accounts({ proposal })
```

## Update the voting state on a proposal

In order to allow voting on a proposal, we need to update the voting state on the proposal:

```jsx async name=update_voting_state deps=progress
await proposalSdk.methods
  .updateStateV0({
    newState: {
      voting: {
        startTs: new anchor.BN(0),
      },
    },
  })
  .accounts({ proposal })
```

## Next Steps

Interested in using Modular Governance with React? Checkout our [React Examples](./react).

To gain a deeper understanding of the API, check out the API pages on

- [NFT Voter](../api/nft-voter-sdk)
- [Organization](../api/organization-sdk)
- [Proposal](../api/proposal-sdk)
- [State Controller](../api/state-controller-sdk)
- [Token Voter](../api/token-voter-sdk)
