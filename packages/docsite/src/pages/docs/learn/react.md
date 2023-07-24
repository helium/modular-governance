---
sidebar_position: 1
---

# React

The Helium Modular Governance SDK comes with a suite of useful ReactJS hooks.

## Setup

You'll want to set up [solana-wallet-adapter](https://github.com/solana-labs/wallet-adapter), the Helium SDK Provider, and the Helium AccountProvider (an intelligent caching layer on Solana's rpc).

First, setup Solana wallet adapters:

```jsx
import { WalletAdapterNetwork } from '@solana/wallet-adapter-base'
import {
  ConnectionProvider,
  WalletProvider,
} from '@solana/wallet-adapter-react'
import { clusterApiUrl } from '@solana/web3.js'
import React, { FC, useMemo } from 'react'

// Default styles that can be overridden by your app
require('@solana/wallet-adapter-react-ui/styles.css')

export const Wallet: FC = ({ children }) => {
  // Can be set to 'devnet', 'testnet', or 'mainnet-beta'
  const network = WalletAdapterNetwork.Devnet

  // You can also provide a custom RPC endpoint
  const endpoint = useMemo(() => clusterApiUrl(network), [network])

  return (
    <ConnectionProvider endpoint={endpoint}>
      <WalletProvider autoConnect>{children}</WalletProvider>
    </ConnectionProvider>
  )
}
```

Then, setup the Helium SDK Provider:

```jsx
import { AccountProvider } from '@helium/helium-react-hooks'
import { Wallet } from './Wallet'

export const App: FC = ({ children }) => (
  <Wallet>
    <AccountProvider>{children}</AccountProvider>
  </Wallet>
)
```

## Displaying an organization

Let's create a simple organization for testing, then display it:

```jsx async name=create_organization
var {
  pubkeys: { organization },
} = await organizationSdk.methods.initializeOrganizationV0({
  name: 'Test',
  authority: provider.wallet.publicKey,
  defaultProposalConfig: PublicKey.default,
  proposalProgram: proposalProgram.programId,
  uri: 'https://example.com',
})
```

Now display it in React! We can use an advanced, pre-canned trade form:

```js
import { CSSReset } from '@chakra-ui/react'
import { useOrganization } from '@helium/modular-governance-hooks'
```

```jsx live
function OrgDisplay() {
  const org = useOrganization(organization) // Getting organization from above code;

  if (org) {
    // Shadow div and css reset are not required, but will make sure our styles do not conflict with yours
    return (
      <ReactShadow.div>
        <AccountProvider resetCSS onError={(e) => console.error(e)}>
          <div>{org}</div>
        </AccountProvider>
      </ReactShadow.div>
    )
  }

  return <div>Please run the code block above</div>
}
```

## Displaying an organization's proposals

Using the same organization from above, we can display its proposals:

```jsx live
function OrgProposals() {
  const proposals = useOrganizationProposals(organization) // Getting organization from above code;

  if (proposals) {
    // Shadow div and css reset are not required, but will make sure our styles do not conflict with yours
    return (
      <ReactShadow.div>
        <AccountProvider resetCSS onError={(e) => console.error(e)}>
          <div>{proposals}</div>
        </AccountProvider>
      </ReactShadow.div>
    )
  }

  return <div>Please run the code block above</div>
}
```

## Displaying a proposal

Using the same organization from above, we can display a proposal:

```jsx live
function ProposalDisplay() {
  const proposals = useOrganizationProposals(organization) // Getting organization from above code;
  const proposal = useProposal(proposals[0]) // Getting proposal from above code;

  if (proposal) {
    // Shadow div and css reset are not required, but will make sure our styles do not conflict with yours
    return (
      <ReactShadow.div>
        <AccountProvider resetCSS onError={(e) => console.error(e)}>
          <div>{proposal}</div>
        </AccountProvider>
      </ReactShadow.div>
    )
  }

  return <div>Please run the code block above</div>
}
```
