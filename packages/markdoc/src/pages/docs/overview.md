# Overview

In this guide we will learn how Helium Modular Governanance contract works, how to set it up, and how to use it to create proposals, vote for them, and execute them.

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide.
{% /callout %}

## Introduction

Decentralized protocols are in constant evolution from the moment they are publicly released. Often, the initial team retains control of this evolution in the first stages, but eventually delegates it to a community of stakeholders. The process by which this community makes decisions is called on-chain governance, and it has become a central component of decentralized protocols, fueling varied decisions such as parameter tweaking, smart contract upgrades, integrations with other protocols, treasury management, grants, etc.

This governance protocol is generally implemented by the “Organization“, “Proposal“, “Token Voter“ & NFT Voter“ contracts. For Helium Modular Governance Contracts, we set out to build a modular system of Governor contracts so that forking is not needed, and different requirements can be accommodated by using our abstract and general purpose contracts. You will find the most common requirements out of the box in Helium Modular Governance Contracts, but writing additional ones is simple, and we will be adding new features as requested by the community in future releases. Additionally, the design of Helium Modular Governance requires minimal use of storage and results in more gas efficient operation.

## Compatibility

Heliu Modular Governance was designed with a concern for compatibility with existing systems that were based on Solana Token Program(Tokens) and Metaplex Bubblegum Program(NFTs). Because of this, you will find that many modules are presented in two variants, one of which is built for compatibility with those systems.

## Setup

### Token

The voting power of each account in our governance setup will be determined by an either a token voter or a nft voter. When creating an organization the organization owner can specify a default proposal config. This config will be used for all proposals created by the organization. It is important to decide which type of voter you want to use before creating an organization. If you want to use a token voter you need to create a token first. If you want to use a nft voter you need to create an nft collection first.

If your project already has a live token then you should use that token. If you don't have a token yet you can create one using the [Token Program](https://spl.solana.com/token)

### Organization

The first step is to create an organization. The organization is the main contract that will have authority over the other contracts. The organization contract is initialized with a name, an authority, a default proposal config, a proposal program and a uri. The uri is a link to a json file that contains information about the organization. Thie json file can be used to store information about the organization such as a logo, a description, a website, etc.

### Proposals

Proposals are created by the organization or by any account holding a governance token. Proposals can be created with a proposal config or without a proposal config. If a proposal config is provided it will override the default proposal config of the organization. If no proposal config is provided the default proposal config of the organization will be used.

### Vote Controller

The vote controller is built to handle all types of voting such as NFT, token, helium veHNT tokens, and more.

### State Controller

The state controller contract is built to handle all types of proposal states such as Draft, Voting, Executing, and more.

## Proposal Lifecycle

Proposal lifecycles can be customized by the organization owner. This allows for the implementation of more complex states like Vetoed, drafts, signing off, etc.

### Create a Proposal

Let’s say we want to create a proposal to give a team a grant, in the form of ERC20 tokens from the governance treasury. This proposal will consist of a single action where the target is the ERC20 token, calldata is the encoded function call `transfer(<team wallet>, <grant amount>)`, and with 0 ETH attached.

Generally a proposal will be created with the help of an interface such as Tally or Defender. Here we will show how to create the proposal using Ethers.js.

### Cast a Vote

Once a proposal is active, voters can cast their vote. In the case of the governance token being a mint such as a Helium Token then the token-voter contract will be used. If the governance token is a NFT then the nft-voter contract will be used. The token-voter and nft-voter contracts are initialized with a name and authority.

### Execute the Proposal

Once the voting period is over, if quorum was reached (enough voting power participated) and the majority voted in favor, the proposal is considered successful and can proceed to be executed. Once a proposal passes, it can be queued and executed from the same place you voted.

If a timelock was set up, then...
