---
title: Installing Helium Modular Governance
description: Learn how to install Helium Modular Governance
---

Modular Governance is a set of smart contracts that can be used to create a governance system for your project. It is designed to be modular and flexible so that it can be used for a wide variety of use cases.

---

{% callout type="warning" title="DISCLAIMER" %}
It's essential to be aware that Helium Modular Governance is currently in its Beta phase and may undergo modifications. Except for the NFT voter program, all contracts have been audited. Our team is dedicated to ensuring the security of the contracts and the stability of the API. Should you have any inquiries or concerns, please do not hesitate to contact us on [Discord](https://discord.gg/helium).
{% /callout %}

## Install the SDKs

Before proceeding make sure your computer has [Node.js](https://nodejs.org/en/) installed.

```shell
yarn add @helium/nft-voter-sdk @helium/organization-sdk @helium/organization-wallet-sdk @helium/proposal-sdk @helium/state-controller-sdk @helium/token-voter-sdk
```
