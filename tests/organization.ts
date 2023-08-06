import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Proposal } from "../target/types/proposal";
import { Organization } from "../target/types/organization";
import { PublicKey } from "@solana/web3.js";
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk";
import { PROGRAM_ID, init, proposalKey } from "@helium/organization-sdk";
import { expect } from "chai";
import { ensureIdls, makeid } from "./utils";

describe("organization", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  let proposalProgram: Program<Proposal>;
  let program: Program<Organization>;

  let name: string;
  beforeEach(async () => {
    await ensureIdls();

    name = makeid(10);
    program = await init(
      provider,
      PROGRAM_ID,
      anchor.workspace.Organization.idl
    );

    // @ts-ignore
    proposalProgram = await initProposal(
      provider,
      PROPOSAL_PID,
      anchor.workspace.Proposal.idl
    );
  });

  it("initializes an organization by name", async () => {
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

  describe("with org and proposal config", () => {
    let organization: PublicKey | undefined;
    let proposalConfig: PublicKey | undefined;

    beforeEach(async () => {
      ({
        pubkeys: { proposalConfig },
      } = await proposalProgram.methods
        .initializeProposalConfigV0({
          name,
          voteController: me,
          stateController: me,
          onVoteHook: PublicKey.default,
        })
        .rpcAndKeys());

      ({
        pubkeys: { organization },
      } = await program.methods
        .initializeOrganizationV0({
          name,
          authority: me,
          defaultProposalConfig: proposalConfig!,
          proposalProgram: proposalProgram.programId,
          uri: "https://example.com",
        })
        .rpcAndKeys({ skipPreflight: true }));
    });

    it("allows updating the organization", async () => {
      await program.methods
        .updateOrganizationV0({
          uri: "https://foo.com",
          defaultProposalConfig: me,
          proposalProgram: me,
          authority: PublicKey.default,
        })
        .accounts({ organization })
        .rpc({ skipPreflight: true });

      const acct = await program.account.organizationV0.fetch(organization!);
      expect(acct.defaultProposalConfig.toBase58()).to.eq(
        me.toBase58()
      );
      expect(acct.authority.toBase58()).to.eq(PublicKey.default.toBase58());
      expect(acct.name).to.eq(name);
      expect(acct.uri).to.eq("https://foo.com");
    });

    it("creates a proposal with the default config", async () => {
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

      expect(proposal?.toBase58()).to.eq(
        proposalKey(organization!, 0)[0].toBase58()
      );
    });

    describe("with proposal", () => {
      let proposal: PublicKey | undefined;
      beforeEach(async () => {
        ({
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
          .rpcAndKeys({ skipPreflight: true }));
        await proposalProgram.methods
          .updateStateV0({
            newState: {
              voting: {
                startTs: new anchor.BN(0),
              },
            },
          })
          .accounts({ proposal })
          .rpc();
      });
      it("allows voting on the proposal", async () => {
        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(1),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });
      });
    });
  });
});
