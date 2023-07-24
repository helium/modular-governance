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

Note that you only need to install the sdks of the contracts you wish to use.

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
const proposalProgram = await initProposal(provider, PROPOSAL_PID)
```

## Creating and Managing Proposals

An Organization is not required for a proposal. This allows maximum flexibility, allowing builders to create their own hierarchical structures outside of the organization smart contract. Let's create a simple Yes/No proposal.

First, create a proposal config. This tells a proposal how votes will be tallied and how state progression occurs. By setting ourself as the vote and state controller, we control both. Using our wallet, we can add/remove votes and move the proposal from Draft to Voting to Resolved.

```js async name=proposal-config
const {
  pubkeys: { proposalConfig },
} = await proposalProgram.methods
  .initializeProposalConfigV0({
    name,
    voteController: provider.wallet.publicKey,
    stateController: provider.wallet.publicKey,
    // Set to default pubkey to disable vote hooks
    onVoteHook: PublicKey.default,
  })
  .rpcAndKeys();
```

Now lets create the proposal:

```js async name=create-proposal deps=proposal-config
const {
        pubkeys: { proposal },
      } = await proposalProgram.methods
        .initializeProposalV0({
          seed: Buffer.from(name, "utf-8"),
          // Allows for voters to vote for more than one choice
          maxChoicesPerVoter: 1,
          name,
          // Optionally put additional JSON information about the proposal here.
          uri: "https://example.com",
          choices: [
            {
              name: "Yes",
              // Optionally put additional JSON information about the choice here. This is useful for long form markdown choices
              uri: null,
            },
            {
              name: "No",
              uri: null,
            },
          ],
          tags: ["test", "tags"],
        })
        .accounts({ proposalConfig })
        .rpcAndKeys();
```

Notice that every Anchor call only requires a minimum subset of accounts. Modular-governance's smart resolvers remove the pain of account munging! Just specify the accounts that you think are necessary, and it is most likely sufficient.

### Proposal Lifecycle

Every proposal has a `state` field that starts as a `Draft`. In order to vote, a proposal needs to be in state `Voting`. When a vote is finished, it will be `Resolved`.

The main way to update the `state` of a proposal is via the state controller. Let's set this proposal to voting:

```js async name=state deps=create-proposal
await proposalProgram.methods
  .updateStateV0({
    newState: {
      voting: {
        // Started voting now
        startTs: new anchor.BN((new Date().valueOf()) / 1000),
      },
    },
  })
  .accounts({ proposal })
  .rpc();
```

#### Custom States

You can implement custom states like signoff via the custom state

```js
await proposalProgram.methods
  .updateStateV0({
    newState: {
      custom: {
        name: "Sign Off",
        bin: Buffer.from([]) // You can include a custom struct here if necessary
      },
    },
  })
  .accounts({ proposal })
  .rpc();
```

### Voting

Now that the proposal is in the `Voting` state, you can vote on a proposal by calling `voteV0`

```js async name=vote deps=state
await proposalProgram.methods
  .voteV0({
    // This is voting for choice "No"
    choice: 1,
    weight: new anchor.BN(2),
    // Vote controllers are able to switch this flag to add and remove votes
    removeVote: false,
  })
  // The voter is used for indexing, associating votes with wallets. This is more strictly enforced by vote controllers
  .accounts({ proposal, voter: provider.wallet.publicKey })
  .rpc();
```

## Organizations

Proposals on their own are powerful, but we need a way to organize them. Organizations are a holding pattern for proposals. They allow you to group and list proposals, as well as keeping a default configuration for proposals.

### Creating an Organization

```jsx async name=create_organization
var {
  pubkeys: { organization },
} = await organizationSdk.methods.initializeOrganizationV0({
  name,
  authority: provider.wallet.publicKey,
  // Reuse the proposal config from above
  defaultProposalConfig: proposalConfig,
  // Modular governance does not require you use Helium's deployed programs. You can deploy your own.
  proposalProgram: proposalProgram.programId,
  // Extra json information about the organization
  uri: 'https://example.com',
})
```
We can fetch the accounts we just created

```jsx async name=fetch deps=create_organization
var acct = await proposalProgram.account.organizationV0.fetch(organization!);
```

## Creating a proposal

Now, we can create a proposal with the default config:

```jsx async name=create_proposal deps=fetch
var {
  pubkeys: { proposal },
} = await proposalProgram.methods
  .initializeProposal({
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

###  Voting on a proposal

Now, we can vote on the proposal:

```jsx async name=vote deps=fetch_proposal
await proposalProgram.methods
  .updateStateV0({
    newState: {
      voting: {
        // Started voting now
        startTs: new anchor.BN((new Date().valueOf()) / 1000),
      },
    },
  })
  .accounts({ proposal })
  .rpc();
await proposalProgram.methods
  .voteV0({
    choice: 1,
    weight: new anchor.BN(2),
    removeVote: false,
  })
  .accounts({ proposal, voter: provider.wallet.publicKey })
```

## State Controllers

State controllers are programs that impose rules and structure to proposal `state` transition. The default state controller allows boolean logic for determining when a proposal is resolved.

State controllers work in two ways

  * The state controller has the authority to update the state of the proposal. This typically happens via calling the `resolveV0` endpoint, which is permissionless. Clockwork can call this endpoint after a vote finishes, for example
  * Via the on vote hook. If specified in the proposal config, `onVoteV0` is called on the state controller, and the controller can optionally return an early resolution.

Let's take a look at the default state controller.

We can form very complex resolution settings. Let's take a hypothetical governance with early tipping where we want a proposal to resolve to the top choice when either

   * 1 week has passed AND the top choice has > 50% of the vote OR
   * The top choice has at least 75% of the votes AND at least 100 vote weight.

First, we need to get our resolution settings `nodes`. `nodes` represent boolean logic to accomplish the above.

```js
import {settings, init, PROGRAM_ID as STATE_CONTROLLER_PROGRAM_ID} from "@helium/state-controller-sdk"
import BN from "bn.js"

const stateControllerProgram = await init(provider);

const nodes = settings().or(
  settings().and(
    // One week from the start time
    settings().offsetFromStartTs(new BN(60 * 60 * 24 * 7)),
    settings().and(
      settings().choicePercentage(50),
      settings().top(1)
    )
  ),
  settings().and(
    settings().and(
      settings().choicePercentage(75),
      settings().choiceVoteWeight(new BN(100))
    ),
    // Choice percentage and vote weight resolve eagerly. Without a time bound, they will resolve to `[]` (no choices won)
    // immediately. Specifying an `AND` operation with `numResolved` will prevent the vote from resolving this branch
    // until the left side of the branch has at least `1` resolved option
    settings().numResolved(1)
  )
).build()
```

Next, create the resolution controller

```js
const {
  pubkeys: { resolutionSettings },
} = await stateControllerProgram.methods
  .initializeResolutionSettingsV0({
    name: "Early Tipping 75%, 1 week 50%",
    settings: {
      nodes,
    },
  })
  .rpcAndKeys()
```


Next, create a proposal config:


```js
const {
  pubkeys: { proposalConfig },
} = await proposalProgram.methods
  .initializeProposalConfigV0({
    name,
    voteController: provider.wallet.publicKey,
    stateController: resolutionSettings,
    onVoteHook: STATE_CONTROLLER_PROGRAM_ID,
  })
  .rpcAndKeys();
```

Next, we can initiate voting via the state controller

```js
await proposalProgram.methods
  .updateStateV0({
    newState: { voting: {} },
  })
  .accounts({ proposal })
  .rpc();
```

Notice that the `voting` state here does not take `startTs`. This is by design -- the state controller enforces setting a proper start ts. The state controller also enforces that only the `owner` of a proposal can move it from the
`Draft` state to the `Voting` state.

Now, let's eagerly resolve the vote

```js
await proposalProgram.methods
  .voteV0({
    // This is voting for choice "No"
    choice: 1,
    weight: new anchor.BN(100),
    removeVote: false,
  })
  .accounts({ proposal, voter: provider.wallet.publicKey })
  .rpc();
```

If we fetch the proposal, we will see it has resolved:

```js
const proposalAcct = await proposalProgram.account.proposalV0.fetch(proposal)
expect(proposalAcct.state.resolved.choices).to.deep.eq([1])
```

### Creating your own State Controller

For more complex workflows and resolution strategies, you may wish to create your own state controller. The only requirement for a state controller is that it uses its authority to update the state when necessary. This could be a wallet owned by a web2 service. Or a new program. 

## Vote Controllers

Vote controllers translate voting mechanism to vote weights on a proposal. There are many vote mechanisms. 1 NFT = 1 vote (`nft-voter`). 1 Token = 1 vote (`token-voter`). VeTokens that give more vote weight to longer lockups (`helium-program-library/voter-stake-registry`). The opportunities are endless, and so modular governance explicitly doesn't take any opinions. As far as a proposal is concerned, there is an address that signs to set votes for voters.

Let's use an Token voter. The `token-voter` contract works by allowing voters to deposit their voting tokens,
and in exchange receive an NFT `receipt`. This allows the program to track votes while knowing individual
tokens cannot move to another account while they are being used in a vote, stopping double votes.

First, configure proposal config and token voter:

```js
import { init } from "@helium/token-voter-sdk";
import { createMint } from "@helium/spl-utils";
const tokenVoterProgram = await init(provider);
```

```js
const me = provider.wallet.publicKey;
// If you don't already have a mint address
const mint = await createMint(provider, 8, me, me);

const {
  pubkeys: { tokenVoter },
} = await program.methods
  .initializeTokenVoterV0({
    name,
    authority: me,
    // JSON metadata for the collection of the receipt NFTs
    collectionUri: "https://example.com",
  })
  .preInstructions([
    ComputeBudgetProgram.setComputeUnitLimit({ units: 500000 }),
  ])
  .accounts({
    mint,
  })
  .rpcAndKeys()
const {
  pubkeys: { proposalConfig },
} = await proposalProgram.methods
  .initializeProposalConfigV0({
    name,
    voteController: tokenVoter,
    stateController: provider.wallet.publicKey,
    // Set to default pubkey to disable vote hooks
    onVoteHook: PublicKey.default,
  })
  .rpcAndKeys();
```

Now, create a proposal and set it to voting:

```js
const {
  pubkeys: { proposal },
} = await proposalProgram.methods
  .initializeProposalV0({
    seed: Buffer.from(name, "utf-8"),
    maxChoicesPerVoter: 1,
    name,
    uri: "https://example.com",
    choices: [
      {
        name: "Yes",
        uri: null,
      },
      {
        name: "No",
        uri: null,
      },
    ],
    tags: ["test", "tags"],
  })
  .accounts({ proposalConfig })
  .rpcAndKeys({ skipPreflight: true }));

await proposalProgram.methods
  .updateStateV0({
    newState: {
      voting: {
        startTs: new BN(0),
      },
    },
  })
  .accounts({ proposal })
  .rpc();
```

Next, the user deposits their tokens:

```js
const {
  pubkeys: { receipt },
} = await (
  await deposit({
    program,
    amount: toBN(10, 0),
    // The JSON metadata for the receipt NFT.
    metadataUri: "https://example.com",
    tokenVoter,
  })
).rpcAndKeys();
```

Now vote:

```js
await program.methods
  .voteV0({
    choice: 0,
  })
  .accounts({ receipt, proposal })
  .rpcAndKeys({ skipPreflight: true })
```

To withdraw our tokens, we need to relinquish all active votes:

```js
await program.methods
  .relinquishVoteV0({
    choice: 0,
  })
  // Refund specifies a rent refund destination
  .accounts({ receipt, proposal, refund: me })
  .rpc();
```

Now we can withdraw:
```js
await program.methods
  .withdrawV0()
  .accounts({ receipt, refund: me })
  .rpc({ skipPreflight: true })
```


The `nft-voter` contract works similarly to the `token-voter`, but does not require deposits and withdrawals. Simply
initialize `nft-voter` with your NFT collection, and issue vote/relinquish vote with any NFT in that collection.

## Next Steps

Interested in using Modular Governance with React? Checkout our [React Examples](./react).

To gain a deeper understanding of the API, check out the API pages on

- [NFT Voter](../api/nft-voter-sdk)
- [Organization](../api/organization-sdk)
- [Proposal](../api/proposal-sdk)
- [State Controller](../api/state-controller-sdk)
- [Token Voter](../api/token-voter-sdk)
