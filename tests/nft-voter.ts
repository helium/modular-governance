import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PROGRAM_ID, init } from "@helium/nft-voter-sdk";
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk";
import {
  PROGRAM_ID as DEL_PID,
  init as initNftDelegation,
  delegationKey,
} from "@helium/nft-delegation-sdk";
import { Metaplex, walletAdapterIdentity } from "@metaplex-foundation/js";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { expect } from "chai";
import { NftVoter } from "../target/types/nft_voter";
import { NftDelegation } from "../target/types/nft_delegation";
import { Proposal } from "../target/types/proposal";
import { ensureIdls, makeid } from "./utils";

describe("nft-voter", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  let proposalProgram: Program<Proposal>;
  let program: Program<NftVoter>;
  let delegateProgram: Program<NftDelegation>;

  const metaplex = new Metaplex(provider.connection);
  metaplex.use(walletAdapterIdentity(provider.wallet));

  let name: string;
  beforeEach(async () => {
    name = makeid(10);
    program = await init(provider, PROGRAM_ID, anchor.workspace.NftVoter.idl);

    // @ts-ignore
    proposalProgram = await initProposal(
      provider,
      PROPOSAL_PID,
      anchor.workspace.Proposal.idl
    );
    delegateProgram = await initNftDelegation(
      provider,
      DEL_PID,
      anchor.workspace.NftDelegation.idl
    );
  });

  describe("with proposal", () => {
    let proposalConfig: PublicKey | undefined;
    let proposal: PublicKey | undefined;
    let nftVoter: PublicKey | undefined;
    let collection: PublicKey;
    let mint: PublicKey;
    const collectionAuthority = Keypair.generate();

    beforeEach(async () => {
      await ensureIdls();
      collection = (
        await metaplex.nfts().create({
          uri: "https://example.com",
          name: "test",
          symbol: "test",
          sellerFeeBasisPoints: 0,
          updateAuthority: collectionAuthority,
          tokenOwner: collectionAuthority.publicKey,
        })
      ).nft.address;

      mint = (
        await metaplex.nfts().create({
          uri: "https://example.com",
          name: "test",
          symbol: "test",
          sellerFeeBasisPoints: 0,
          collection,
          collectionAuthority,
        })
      ).nft.address;

      ({
        pubkeys: { nftVoter },
      } = await program.methods
        .initializeNftVoterV0({
          name,
          authority: me,
        })
        .accounts({
          collection,
        })
        .rpcAndKeys({ skipPreflight: true }));
      ({
        pubkeys: { proposalConfig },
      } = await proposalProgram.methods
        .initializeProposalConfigV0({
          name,
          voteController: nftVoter!,
          stateController: me,
          onVoteHook: PublicKey.default,
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
          ],
          tags: ["test", "tags"],
        })
        .accounts({ proposalConfig })
        .rpcAndKeys({ skipPreflight: true }));

      await proposalProgram.methods
        .updateStateV0({
          newState: {
            voting: {
              startTs: new BN(0),
            },
          },
        })
        .accounts({ proposal })
        .rpc();
    });

    it("allows voting on and relinquishing votes on the proposal", async () => {
      const {
        pubkeys: { marker },
      } = await program.methods
        .voteV0({
          choice: 0,
        })
        .accounts({ mint, proposal, nftVoter })
        .rpcAndKeys({ skipPreflight: true });

      let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.choices[0].weight.toNumber()).to.eq(1);
      let markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
      expect(markerA?.choices).to.deep.eq([0]);

      await program.methods
        .relinquishVoteV0({
          choice: 0,
        })
        .accounts({ mint, proposal, refund: me, nftVoter })
        .rpc({ skipPreflight: true });

      acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.choices[0].weight.toNumber()).to.eq(0);
      markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
      expect(markerA).to.be.null;
    });

    describe("with delegation", () => {
      let delegatee = Keypair.generate();

      beforeEach(async () => {
        await delegateProgram.methods
          .delegateV0()
          .accounts({
            mint,
            recipient: delegatee.publicKey,
          })
          .rpc({ skipPreflight: true });
      });

      it("allows voting on and relinquishing votes on the proposal", async () => {
        const {
          pubkeys: { marker },
        } = await program.methods
          .delegatedVoteV0({
            choice: 0,
          })
          .accounts({ mint, proposal, nftVoter, owner: delegatee.publicKey })
          .signers([delegatee])
          .rpcAndKeys({ skipPreflight: true });

        let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[0].weight.toNumber()).to.eq(1);
        let markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA?.choices).to.deep.eq([0]);

        await program.methods
          .delegatedRelinquishVoteV0({
            choice: 0,
          })
          .accounts({
            mint,
            proposal,
            refund: me,
            nftVoter,
            owner: delegatee.publicKey,
          })
          .signers([delegatee])
          .rpc({ skipPreflight: true });

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[0].weight.toNumber()).to.eq(0);
        markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA).to.be.null;
      });

      it("allows the original owner to undelegate", async () => {
        const toUndelegate = delegationKey(mint, delegatee.publicKey)[0];
        const myDelegation = delegationKey(mint, me)[0];
        await delegateProgram.methods
          .undelegateV0()
          .accounts({
            delegation: toUndelegate,
            prevDelegation: myDelegation,
            currentDelegation: myDelegation,
          })
          .rpc({ skipPreflight: true });

        expect(
          (
            await delegateProgram.account.delegationV0.fetch(myDelegation)
          ).nextOwner.toBase58()
        ).to.eq(PublicKey.default.toBase58());
        expect(
          await delegateProgram.account.delegationV0.fetchNullable(toUndelegate)
        ).to.be.null;
      });
    });
  });
});
