## Dependencies

```typescript
import * as anchor from "@coral-xyz/anchor"
import { Program } from "@coral-xyz/anchor"
import { Proposal } from "../target/types/proposal"
import { Organization } from "../target/types/organization"
import { PublicKey } from "@solana/web3.js"
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk"
import { PROGRAM_ID, init, proposalKey } from "@helium/organization-sdk"
import { expect } from "chai"
import { ensureIdls, makeid } from "./utils"
```

### organization

```typescript
// Configure the client to use the local cluster.
anchor.setProvider(anchor.AnchorProvider.env())
const provider = anchor.getProvider() as anchor.AnchorProvider
const me = provider.wallet.publicKey
let proposalProgram: Program<Proposal>
let program: Program<Organization>

let name: string
beforeEach(async () => {
  await ensureIdls()

  name = makeid(10)
  program = await init(provider, PROGRAM_ID, anchor.workspace.Organization.idl)

  proposalProgram = await initProposal(
    provider,
    PROPOSAL_PID,
    anchor.workspace.Proposal.idl
  )
})
```

#### initializes an organizatiopn by name

```typescript
    const {
    pubkeys: { organization },
  } = await program.methods
    .initializeOrganizationV0({
      name,
      authority: me,
      defaultProposalConfig: PublicKey.default,
      proposalProgram: proposalProgram.programId,
      uri: "https://example.com",
    })
    .rpcAndKeys({ skipPreflight: true });

  const acct = await program.account.organizationV0.fetch(organization!);
  expect(acct.defaultProposalConfig.toBase58()).to.eq(
    PublicKey.default.toBase58()
  );
  expect(acct.authority.toBase58()).to.eq(me.toBase58());
  expect(acct.name).to.eq(name);
  expect(acct.uri).to.eq("https://example.com");
});


```

### with org and proposal config

```typescript
let organization: PublicKey | undefined
let proposalConfig: PublicKey | undefined

beforeEach(async () => {
  ;({
    pubkeys: { proposalConfig },
  } = await proposalProgram.methods
    .initializeProposalConfigV0({
      name,
      voteController: me,
      stateController: me,
      onVoteHook: PublicKey.default,
    })
    .rpcAndKeys())

  ;({
    pubkeys: { organization },
  } = await program.methods
    .initializeOrganizationV0({
      name,
      authority: me,
      defaultProposalConfig: proposalConfig!,
      proposalProgram: proposalProgram.programId,
      uri: "https://example.com",
    })
    .rpcAndKeys({ skipPreflight: true }))
})
```

#### creates a proposal with the default config

```typescript
      const {
      pubkeys: { proposal },
    } = await program.methods
      .initializeProposalV0({
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
      .accounts({ organization })
      .rpcAndKeys({ skipPreflight: true });

    const acct = await proposalProgram.account.proposalV0.fetch(proposal!);

    expect(acct.seed.readUint32LE()).to.eq(0);
    expect(acct.name).to.eq(name);
    expect(acct.uri).to.eq("https://example.com");
    expect(acct.choices[0].name).to.eq("Yes");
    expect(acct.choices[1].name).to.eq("No");
    expect(acct.maxChoicesPerVoter).to.eq(1);
    expect(acct.tags[0]).to.eq("test");
    expect(acct.tags[1]).to.eq("tags");

    expect(proposal?.toBase58()).to.eq(proposalKey(organization!, 0)[0].toBase58())
  });
});
});

```
