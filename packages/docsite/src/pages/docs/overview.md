# Overview

In this guide we will learn how Helium Modular Governanance contract works, how to set it up, and how to use it to create proposals, vote for them, and execute them.

{% callout title="Quick tip" %}
If you are looking for a quick start guide, check out the [Getting Started](/docs/learn/getting_started) guide.
{% /callout %}

## Introduction

Decentralized protocols are in constant evolution from the moment they are publicly released. Often, the initial team retains control of this evolution in the first stages, but eventually delegates it to a community of stakeholders. The process by which the Helium community makes decisions is called on-chain governance, and it has become a central component of decentralized protocols, fueling varied decisions such as parameter tweaking, smart contract upgrades, integrations with other protocols, treasury management, grants, etc.

In the process of building Helium governance, we found that there is no one-size fits all governance model. Instead, a governance standard needs to be sufficiently modular to allow experimentation with governance models.

This governance protocol is generally implemented by the “Organization“, “Proposal“, “State Controller“ contracts. For Helium Modular Governance Contracts, we set out to build a system of governance contracts so that different requirements can be accommodated by using the general purpose contracts and supplementing with custom contracts where needed. You will find the most common requirements out of the box in Helium Modular Governance Contracts, but writing additional ones is simple, and we will be adding new features as requested by the community in future releases.

## Compatibility

Helium Modular Governance was designed with a concern for compatibility with existing voting systems that were based on Solana Token Program(Tokens) and Metaplex Token Metadata Program(NFTs). Because of this, you will find two modules, `nft-voter` and `token-voter` for accommodating these simple voting systems. More complex voting systems can be created by following the design patterns in these contracts, and calling the `vote_v0` endpoint.

## Setup

### Token

The voting power of each account in our governance setup will be determined by an either a token voter, nft voter, or defined by a custom vote controller. When creating an organization the organization owner can specify a default proposal config. This config will be used for all proposals created by the organization. It is important to decide which type of voter you want to use before creating an organization. If you want to use a token voter you need to create a token first. If you want to use a nft voter you need to create an nft collection first.

If your project already has a live token then you should use that token. If you don't have a token yet you can create one using the [Token Program](https://spl.solana.com/token)

### Organization

While organizations are not necessary to create proposals, they act as storage and suggested configuration for all proposals relating to an entity. An organization is initialized with a name, an authority, a default proposal config, a proposal program and a uri. The uri is a link to a json file that contains information about the organization. Thie json file can be used to store information about the organization such as a logo, a description, a website, etc.
Creating an organization ensures that all proposals are neatly contained under that organization. This is a useful pattern for building user interfaces with our sdk, as you can easily collect a list of proposals.

### Proposals

Proposals are created by the organization or by any account holding a governance token. Proposals can be created with a proposal config or without a proposal config. If a proposal config is provided it will override the default proposal config of the organization. If no proposal config is provided the default proposal config of the organization will be used.

### Vote Controller

The vote controller is built to handle all types of voting such as NFT, token, helium veHNT tokens, and more.

### State Controller

The state controller contract is built to handle all types of proposal states such as Draft, Voting, Executing, and more. The default state controller can handle custom boolean logic determining how a vote is resolved to a set of choices. This state controller can be overridden with a custom state controller smart contract of your choice to further customize behavior.

## Proposal Lifecycle

Proposal lifecycles can be customized by replacing the state controller and on vote hook contracts in the proposal config. This allows for the implementation of more complex states like Vetoed, drafts, signing off, etc.

Its important to note that proposals can be created without an organization. In this case, the proposal will not be associated with an organization and will not be able to use the organization's default proposal config. This was done in order to offer max amount of flexibility to the user. Additionally, proposals can be created with an organization and in this case only the organization owner can create proposals. If you would like to allow any voter to create a proposal this can be done by forking the state-controller contract.

### Cast a Vote

Once a proposal is active, voters can cast their vote. In the case of the governance token being a mint such as a Helium Token then the token-voter contract will be used. If the governance token is a NFT then the nft-voter contract will be used. The token-voter and nft-voter contracts are initialized with a name and authority.

### Execute the Proposal

Once the voting period is over, if quorum was reached (enough voting power participated) and the majority voted in favor (depending on the resolution settings of the state controller), the proposal is considered successful.

### Attaching Transactions to Proposals

In some cases, a passing proposal should result in instructions executed on chain. This work is still in progress. Check back later for updates!
