import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Proposal } from "../target/types/proposal";
import { StateController } from "../target/types/state_controller";
import { PublicKey } from "@solana/web3.js";
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk";
import { PROGRAM_ID, init, settings } from "@helium/state-controller-sdk";
import { expect } from "chai";
import { ensureIdls, makeid } from "./utils";

describe("state-controller", () => {
  anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  let proposalProgram: Program<Proposal>;
  let program: Program<StateController>;

  let name: string;
  beforeEach(async () => {
    name = makeid(10);
    // @ts-ignore
    program = await init(
      provider,
      PROGRAM_ID,
      anchor.workspace.StateController.idl
    );

    // @ts-ignore
    proposalProgram = await initProposal(
      provider,
      PROPOSAL_PID,
      anchor.workspace.Proposal.idl
    );
  });

  describe("with proposal", () => {
    let nodes = settings().resolved([1]).build();
    let proposalConfig: PublicKey | undefined;
    let proposal: PublicKey | undefined;
    let resolutionSettings: PublicKey | undefined;
    beforeEach(async () => {
      await ensureIdls();

      ({
        pubkeys: { resolutionSettings },
      } = await program.methods
        .initializeResolutionSettingsV0({
          name,
          settings: {
            nodes,
          },
        })
        .rpcAndKeys({ skipPreflight: true }));
      ({
        pubkeys: { proposalConfig },
      } = await proposalProgram.methods
        .initializeProposalConfigV0({
          name,
          voteController: me,
          stateController: resolutionSettings!,
          onVoteHook: PROGRAM_ID,
          authority: me,
        })
        .rpcAndKeys({ skipPreflight: true }));
      ({
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
            {
              name: "Abstain",
              uri: null,
            },
          ],
          tags: ["test", "tags"],
        })
        .accounts({ proposalConfig })
        .rpcAndKeys({ skipPreflight: true }));

      await program.methods
        .updateStateV0({
          newState: { voting: {} },
        })
        .accounts({ proposal })
        .rpc();
    });

    describe("with resolved", () => {
      before(async () => {
        nodes = settings().resolved([1]).build();
      });

      it("resolves to the choice selected", async () => {
        await program.methods.resolveV0().accounts({ proposal }).rpc();

        const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([1]);
      });
    });

    describe("with end passed", () => {
      before(async () => {
        nodes = settings().endTimestamp(new anchor.BN(1)).build();
      });

      it("resolves to all choices", async () => {
        await program.methods.resolveV0().accounts({ proposal }).rpc();

        const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([0, 1, 2]);
      });
    });

    describe("with end not passed", () => {
      before(async () => {
        nodes = settings().endTimestamp(new anchor.BN(2688657226)).build();
      });

      it("not resolve", async () => {
        await program.methods.resolveV0().accounts({ proposal }).rpc();

        const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(Boolean(acct.state.voting)).to.be.true;
      });
    });

    describe("with offset", () => {
      before(async () => {
        nodes = settings().offsetFromStartTs(new anchor.BN(1)).build();
      });

      it("resolves to all choices", async () => {
        await sleep(3000);
        await program.methods.resolveV0().accounts({ proposal }).rpc();

        const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([0, 1, 2]);
      });
    });

    describe("with choice vote weight", () => {
      before(async () => {
        nodes = settings()
          .and(
            settings().choiceVoteWeight(new anchor.BN(1)),
            settings().numResolved()
          )
          .build();
      });

      it("resolves to the choices with that weight", async () => {
        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(1),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        const acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([0]);
      });
    });

    describe("with choice percentage", () => {
      before(async () => {
        nodes = settings()
          .and(settings().choicePercentage(51), settings().numResolved(1))
          .build();
      });

      it("resolves to the choices with that percentage", async () => {
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

    describe("with top", () => {
      before(async () => {
        // Note that top would immediately resolve.
        // To prevent that, we need to ensure at least 1 choice
        // has been voted on.
        nodes = settings()
          .and(
            settings().and(
              settings().choicePercentage(51),
              settings().numResolved(1)
            ),
            settings().top()
          )
          .build();
      });

      it("resolves to the max choice", async () => {
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

    describe("top choice with minimum vote weight of 5 at the end time", () => {
      before(async () => {
        nodes = settings()
          .and(
            settings().offsetFromStartTs(new anchor.BN(5)),
            settings().and(
              settings().and(
                settings().choiceVoteWeight(new anchor.BN(5)),
                settings().numResolved(1)
              ),
              settings().top()
            )
          )
          .build();
      });

      it("resolves to the max choice when there is enough weight", async () => {
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

        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(3),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        await sleep(10000);

        await program.methods.resolveV0().accounts({ proposal }).rpc();

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([1]);
      });

      it("resolves to no values when not enough weight", async () => {
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

        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(3),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        await sleep(10000);
        await program.methods.resolveV0().accounts({ proposal }).rpc();

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([]);
      });
    });

    describe("resolved by choice vote weight or offset", () => {
      before(async () => {
        nodes = settings()
          .and(
            settings().choiceVoteWeight(new anchor.BN(5)),
            settings().or(
              settings().numResolved(1),
              settings().offsetFromStartTs(new anchor.BN(5))
            )
          )
          .build();
      });

      it("resolves to the max choice when there is enough weight", async () => {
        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(3),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        await program.methods.resolveV0().accounts({ proposal }).rpc();

        let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(Boolean(acct.state.voting)).to.be.true;

        await proposalProgram.methods
          .voteV0({
            choice: 1,
            weight: new anchor.BN(5),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([1]);
      });

      it("resolves to no choices after offset", async () => {
        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(3),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        await program.methods.resolveV0().accounts({ proposal }).rpc();

        let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(Boolean(acct.state.voting)).to.be.true;

        await sleep(6000);

        await program.methods.resolveV0().accounts({ proposal }).rpc();

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([]);
      });
    });

    describe("helium flavor governance", () => {
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
          .build();
      });

      it("resolves to the max choice when there is enough weight", async () => {
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

        await proposalProgram.methods
          .voteV0({
            choice: 0,
            weight: new anchor.BN(1),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        await sleep(10000);
        await program.methods.resolveV0().accounts({ proposal }).rpc();

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([1]);
      });
    });

    describe("helium IOT flavor governance", () => {
      before(async () => {
        nodes = settings()
          .and(
            settings().offsetFromStartTs(new anchor.BN(5)),
            settings().and(
              settings().and(
                settings().totalWeight(new anchor.BN(5)),
                settings().and(
                  settings().not("Abstain"),
                  settings().choicePercentageOfCurrent(66.6)
                )
              ),
              settings().top()
            )
          )
          .build();
      });

      it("resolves to the max choice when there is enough weight", async () => {
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

        await proposalProgram.methods
          .voteV0({
            choice: 2,
            weight: new anchor.BN(5),
            removeVote: false,
          })
          .accounts({ proposal, voter: me })
          .rpc({ skipPreflight: true });

        await sleep(10000);
        console.log("txid", await program.methods.resolveV0().accounts({ proposal }).rpc());

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.state.resolved?.choices).to.deep.eq([1]);
      });
    });
  });
});

function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
