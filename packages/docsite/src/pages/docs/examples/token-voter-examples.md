## Dependencies

```typescript
import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Proposal } from "../target/types/proposal"
import { TokenVoter } from "../target/types/token_voter"
import { ComputeBudgetProgram, Keypair, PublicKey } from "@solana/web3.js"
import {
  createMint,
  createAtaAndMint,
  toBN,
  createMintInstructions,
} from "@helium/spl-utils"
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk"
import { PROGRAM_ID, deposit, init, receiptKey } from "@helium/token-voter-sdk"
import { expect } from "chai"
import { ensureIdls, makeid } from "./utils"
import { BN } from "bn.js"
```

### token-voter

```typescript
// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env())
const provider = anchor.getProvider() as anchor.AnchorProvider
const me = provider.wallet.publicKey
let proposalProgram: Program<Proposal>
let program: Program<TokenVoter>

let name: string
beforeEach(async () => {
  name = makeid(10)
  program = await init(provider, PROGRAM_ID, anchor.workspace.TokenVoter.idl)

  proposalProgram = await initProposal(
    provider,
    PROPOSAL_PID,
    anchor.workspace.Proposal.idl
  )
})
```

### with proposal

```typescript
let proposalConfig: PublicKey | undefined
let proposal: PublicKey | undefined
let tokenVoter: PublicKey | undefined
let mint: PublicKey
beforeEach(async () => {
  await ensureIdls()
  mint = await createMint(provider, 0, me, me)
  await createAtaAndMint(provider, mint, toBN(10, 0))
  ;({
    pubkeys: { tokenVoter },
  } = await program.methods
    .initializeTokenVoterV0({
      name,
      authority: me,
      collectionUri: "https://example.com",
    })
    .preInstructions([
      ComputeBudgetProgram.setComputeUnitLimit({ units: 500000 }),
    ])
    .accounts({
      mint,
    })
    .rpcAndKeys({ skipPreflight: true }))
  ;({
    pubkeys: { proposalConfig },
  } = await proposalProgram.methods
    .initializeProposalConfigV0({
      name,
      voteController: tokenVoter!,
      stateController: me,
      onVoteHook: PublicKey.default,
    })
    .rpcAndKeys({ skipPreflight: true }))
  ;({
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
    .rpcAndKeys({ skipPreflight: true }))

  await proposalProgram.methods
    .updateStateV0({
      newState: {
        voting: {
          startTs: new BN(0),
        },
      },
    })
    .accounts({ proposal })
    .rpc()
})
```

#### allows depositing tokens

```typescript
      const {
      pubkeys: { receipt },
    } = await (
      await deposit({
        program,
        amount: toBN(10, 0),
        metadataUri: "https://example.com",
        tokenVoter,
      })
    ).rpcAndKeys({ skipPreflight: true });

    const acct = await program.account.receiptV0.fetch(receipt!);
    expect(acct.amount.toString()).to.eq("10");
  });


```

### with deposited tokens

```typescript
let receipt: PublicKey | undefined
beforeEach(async () => {
  ;({
    pubkeys: { receipt },
  } = await (
    await deposit({
      program,
      amount: toBN(10, 0),
      metadataUri: "https://example.com",
      tokenVoter,
    })
  ).rpcAndKeys({ skipPreflight: true }))
})
```

#### allows withdrawing tokens

```typescript
        await program.methods
        .withdrawV0()
        .accounts({ receipt, refund: me })
        .rpc({ skipPreflight: true });

      expect(await program.account.receiptV0.fetchNullable(receipt!)).to.be.null
    });


```

#### allows voting on and relinquishing votes on the proposal

```typescript
        const {
        pubkeys: { marker },
      } = await program.methods
        .voteV0({
          choice: 0,
        })
        .accounts({ receipt, proposal })
        .rpcAndKeys({ skipPreflight: true });

      let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.choices[0].weight.toNumber()).to.eq(10);
      let markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
      expect(markerA?.choices).to.deep.eq([0]);

      await program.methods
        .relinquishVoteV0({
          choice: 0,
        })
        .accounts({ receipt, proposal, refund: me })
        .rpc({ skipPreflight: true });

      acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.choices[0].weight.toNumber()).to.eq(0);
      markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
      expect(markerA).to.be.null;
    });
  });
});
});

```
