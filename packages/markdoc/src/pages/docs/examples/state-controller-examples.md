## Dependencies

```typescript
import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Proposal } from "../target/types/proposal"
import { StateController } from "../target/types/state_controller"
import { PublicKey } from "@solana/web3.js"
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk"
import {
  PROGRAM_ID,
  SettingsBuilder,
  init,
  settings,
} from "@helium/state-controller-sdk"
import { expect } from "chai"
import { ensureIdls, makeid } from "./utils"
```

### state-controller

```typescript
// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env())
const provider = anchor.getProvider() as anchor.AnchorProvider
const me = provider.wallet.publicKey
let proposalProgram: Program<Proposal>
let program: Program<StateController>

let name: string
beforeEach(async () => {
  name = makeid(10)
  program = await init(
    provider,
    PROGRAM_ID,
    anchor.workspace.StateController.idl
  )

  proposalProgram = await initProposal(
    provider,
    PROPOSAL_PID,
    anchor.workspace.Proposal.idl
  )
})
```

### with proposal

```typescript
let nodes = settings().resolved([1]).build()
let proposalConfig: PublicKey | undefined
let proposal: PublicKey | undefined
let resolutionSettings: PublicKey | undefined
beforeEach(async () => {
  await ensureIdls()

  ;({
    pubkeys: { resolutionSettings },
  } = await program.methods
    .initializeResolutionSettingsV0({
      name,
      settings: {
        nodes,
      },
    })
    .rpcAndKeys({ skipPreflight: true }))
  ;({
    pubkeys: { proposalConfig },
  } = await proposalProgram.methods
    .initializeProposalConfigV0({
      name,
      voteController: me,
      stateController: resolutionSettings!,
      onVoteHook: PROGRAM_ID,
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

  await program.methods
    .updateStateV0({
      newState: { voting: {} },
    })
    .accounts({ proposal })
    .rpc()
})
```

### with resolved

```typescript
before(async () => {
  nodes = settings().resolved([1]).build()
})
```

#### resolves to the choice selected

```typescript
        await program.methods.resolveV0().accounts({ proposal }).rpc();

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([1]);
    })
  })


```

### with end passed

```typescript
before(async () => {
  nodes = settings().endTimestamp(new anchor.BN(1)).build()
})
```

#### resolves to all choices

```typescript
        await program.methods.resolveV0().accounts({ proposal }).rpc();

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([0, 1]);
    });
  });


```

### with end not passed

```typescript
before(async () => {
  nodes = settings().endTimestamp(new anchor.BN(2688657226)).build()
})
```

#### not resolve

```typescript
        await program.methods.resolveV0().accounts({ proposal }).rpc();

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(Boolean(acct.state.voting)).to.be.true;
    });
  });


```

### with offset

```typescript
before(async () => {
  nodes = settings().offsetFromStartTs(new anchor.BN(1)).build()
})
```

#### resolves to all choices

```typescript
        await sleep(3000)
      await program.methods.resolveV0().accounts({ proposal }).rpc();

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([0, 1]);
    });
  });


```

### with choice vote weight

```typescript
before(async () => {
  nodes = settings().choiceVoteWeight(new anchor.BN(1)).build()
})
```

#### resolves to the choices with that weight

```typescript
        await proposalProgram.methods
        .voteV0({
          choice: 0,
          weight: new anchor.BN(1),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc( { skipPreflight: true});

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([0]);
    });
  });


```

### with choice percentage

```typescript
before(async () => {
  nodes = settings().choicePercentage(50).build()
})
```

#### resolves to the choices with that percentage

```typescript
        await proposalProgram.methods
        .voteV0({
          choice: 1,
          weight: new anchor.BN(1),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([1]);
    });
  });


```

### with top

```typescript
before(async () => {
  nodes = settings().top().build()
})
```

#### resolves to the max choice

```typescript
        await proposalProgram.methods
        .voteV0({
          choice: 1,
          weight: new anchor.BN(1),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([1]);
    });
  });


```

### top choice with minimum vote weight of 5 at the end time

```typescript
before(async () => {
  nodes = settings()
    .and(
      settings().offsetFromStartTs(new anchor.BN(5)),
      settings().and(
        settings().choiceVoteWeight(new anchor.BN(5)),
        settings().top()
      )
    )
    .build()
})
```

#### resolves to the max choice when there is enough weight

```typescript
        await proposalProgram.methods
        .voteV0({
          choice: 1,
          weight: new anchor.BN(5),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(Boolean(acct.state.voting)).to.be.true;

      await sleep(10000)

      await proposalProgram.methods
        .voteV0({
          choice: 0,
          weight: new anchor.BN(3),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([1]);
    });


```

#### resolves to no values when not enough weight

```typescript
        await proposalProgram.methods
        .voteV0({
          choice: 1,
          weight: new anchor.BN(2),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(Boolean(acct.state.voting)).to.be.true;

      await sleep(10000);

      await proposalProgram.methods
        .voteV0({
          choice: 0,
          weight: new anchor.BN(3),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([]);
    });
  });


```

### helium flavor governance

```typescript
before(async () => {
  nodes = settings()
    .and(
      settings().offsetFromStartTs(new anchor.BN(5)),
      settings().and(
        settings().and(
          settings().choiceVoteWeight(new anchor.BN(5)),
          settings().choicePercentage(66.6)
        ),
        settings().top()
      )
    )
    .build()
})
```

#### resolves to the max choice when there is enough weight

```typescript
        await proposalProgram.methods
        .voteV0({
          choice: 1,
          weight: new anchor.BN(5),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(Boolean(acct.state.voting)).to.be.true;

      await sleep(10000);

      await proposalProgram.methods
        .voteV0({
          choice: 0,
          weight: new anchor.BN(1),
          removeVote: false,
        })
        .accounts({ proposal, voter: me })
        .rpc({ skipPreflight: true });

      acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.state.resolved?.choices).to.deep.eq([1]);
    });
  });
})
});

function sleep(ms: number) {
return new Promise((resolve) => setTimeout(resolve, ms))
}


```
