import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Proposal } from "../target/types/proposal";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID, init } from "@helium/proposal-sdk";
import { expect } from "chai";
import { makeid } from "./utils";

describe("proposal", () => {
  anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  let program: Program<Proposal>;

  let name: string;
  beforeEach(async () => {
    name = makeid(10);
    // @ts-ignore
    program = await init(provider, PROGRAM_ID, anchor.workspace.Proposal.idl);
  });

  it("Creates a proposal config", async () => {
    const {
      pubkeys: { proposalConfig },
    } = await program.methods
      .initializeProposalConfigV0({
        name,
        voteController: me,
        stateController: me,
        onVoteHook: PublicKey.default,
      })
      .rpcAndKeys();

    const acct = await program.account.proposalConfigV0.fetch(proposalConfig!);

    expect(acct.voteController.toBase58()).to.eq(me.toBase58());
    expect(acct.stateController.toBase58()).to.eq(me.toBase58());
    expect(acct.onVoteHook.toBase58()).to.eq(PublicKey.default.toBase58());
  });

  describe("with proposal config", () => {
    let proposalConfig: PublicKey | undefined;
    beforeEach(async () => {
      ({
        pubkeys: { proposalConfig },
      } = await program.methods
        .initializeProposalConfigV0({
          name,
          voteController: me,
          stateController: me,
          onVoteHook: PublicKey.default,
        })
        .rpcAndKeys());
    });

    it("Creates a proposal", async () => {
      const {
        pubkeys: { proposal },
      } = await program.methods
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
        .rpcAndKeys();

      const acct = await program.account.proposalV0.fetch(proposal!);

      expect(acct.seed.toString("utf-8")).to.eq(name);
      expect(acct.name).to.eq(name);
      expect(acct.uri).to.eq("https://example.com");
      expect(acct.choices[0].name).to.eq("Yes");
      expect(acct.choices[1].name).to.eq("No");
      expect(acct.maxChoicesPerVoter).to.eq(1);
      expect(acct.tags[0]).to.eq("test");
      expect(acct.tags[1]).to.eq("tags");
    });

    describe("with proposal", async () => {
      let proposal: PublicKey;
      beforeEach(async () => {
        proposal = (
          await program.methods
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
            .rpcAndKeys()
        ).pubkeys.proposal!;
      });

      it("allows the vote controller to vote", async () => {
        await program.methods
          .updateStateV0({
            newState: {
              voting: {
                startTs: new anchor.BN(0),
              },
            },
          })
          .accounts({ proposal })
          .rpc();
        await program.methods
          .voteV0({
            choice: 1,
            weight: new anchor.BN(2),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        let acct = await program.account.proposalV0.fetch(proposal);
        expect(acct.choices[1].weight.toString()).to.eq("2");

        await program.methods
          .voteV0({
            choice: 1,
            weight: new anchor.BN(1),
            removeVote: true,
          })
          .accounts({ proposal, voter: me })
          .rpc();

        acct = await program.account.proposalV0.fetch(proposal);
        expect(acct.choices[1].weight.toString()).to.eq("1");
      });

      it("allows the state controller to prorgess the state", async () => {
        await program.methods
          .updateStateV0({
            newState: { custom: { name: "hello", bin: Buffer.from([]) } },
          })
          .accounts({ proposal })
          .rpc({ skipPreflight: true });

        const acct = await program.account.proposalV0.fetch(proposal);
        expect(acct.state.custom?.name).to.eq("hello");
      });
    });
  });
});
