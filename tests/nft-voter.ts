import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PROGRAM_ID, init } from "@helium/nft-voter-sdk";
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk";
import {
  PROGRAM_ID as DEL_PID,
  init as initNftProxy,
  proxyAssignmentKey,
} from "@helium/nft-proxy-sdk";
import { Metaplex, walletAdapterIdentity } from "@metaplex-foundation/js";
import { Keypair, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { expect } from "chai";
import { NftVoter } from "../target/types/nft_voter";
import { NftProxy } from "../target/types/nft_proxy";
import { Proposal } from "../target/types/proposal";
import { ensureIdls, makeid } from "./utils";

describe("nft-voter", () => {
  anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  let proposalProgram: Program<Proposal>;
  let program: Program<NftVoter>;
  let proxyProgram: Program<NftProxy>;

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
    proxyProgram = await initNftProxy(
      provider,
      DEL_PID,
      anchor.workspace.NftProxy.idl
    );
  });

  describe("with proposal", () => {
    let proxyConfig: PublicKey | undefined;
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
        pubkeys: { proxyConfig },
      } = await proxyProgram.methods
        .initializeProxyConfigV0({
          maxProxyTime: new BN(1000000000000),
          name: makeid(10),
          seasons: [
            {
              start: new BN(0),
              end: new BN(new Date().valueOf() / 1000 + 100000),
            },
          ],
        })
        .accounts({
          authority: me,
        })
        .rpcAndKeys());

      ({
        pubkeys: { nftVoter },
      } = await program.methods
        .initializeNftVoterV0({
          name,
          authority: me,
        })
        .accounts({
          collection,
          proxyConfig,
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
        .accounts({ mint, proposal, nftVoter })
        .rpc({ skipPreflight: true });

      acct = await proposalProgram.account.proposalV0.fetch(proposal!);
      expect(acct.choices[0].weight.toNumber()).to.eq(0);
      markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
      expect(markerA).to.be.null;
    });

    describe("with proxy", () => {
      let proxy = Keypair.generate();

      beforeEach(async () => {
        await proxyProgram.methods
          .assignProxyV0({
            expirationTime: new BN(new Date().valueOf() / 1000 + 10000),
          })
          .accounts({
            proxyConfig,
            asset: mint,
            recipient: proxy.publicKey,
          })
          .rpc({ skipPreflight: true });
      });

      it("allows voting on and relinquishing votes on the proposal", async () => {
        const proxyAssignment = proxyAssignmentKey(
          proxyConfig!,
          mint,
          proxy.publicKey
        )[0];
        const {
          pubkeys: { marker },
        } = await program.methods
          .proxiedVoteV0({
            choice: 0,
          })
          .accounts({
            mint,
            proposal,
            nftVoter,
            voter: proxy.publicKey,
            proxyAssignment,
          })
          .signers([proxy])
          .rpcAndKeys({ skipPreflight: true });

        let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[0].weight.toNumber()).to.eq(1);
        let markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA?.choices).to.deep.eq([0]);

        await program.methods
          .proxiedRelinquishVoteV0({
            choice: 0,
          })
          .accounts({
            mint,
            proposal,
            nftVoter,
            voter: proxy.publicKey,
            proxyAssignment,
          })
          .signers([proxy])
          .rpc({ skipPreflight: true });

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[0].weight.toNumber()).to.eq(0);
        markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA).to.be.null;
      });

      it("allows earlier proxies to change the vote", async () => {
        const {
          pubkeys: { marker },
        } = await program.methods
          .proxiedVoteV0({
            choice: 0,
          })
          .accounts({
            mint,
            proposal,
            nftVoter,
            voter: proxy.publicKey,
            proxyAssignment: proxyAssignmentKey(
              proxyConfig!,
              mint,
              proxy.publicKey
            )[0],
          })
          .signers([proxy])
          .rpcAndKeys({ skipPreflight: true });

        let acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[0].weight.toNumber()).to.eq(1);
        let markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA?.choices).to.deep.eq([0]);
        expect(markerA?.proxyIndex).to.eq(1);

        await program.methods
          .relinquishVoteV0({
            choice: 0,
          })
          .accounts({
            mint,
            proposal,
            nftVoter,
          })
          .rpc({ skipPreflight: true });

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[0].weight.toNumber()).to.eq(0);
        markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA).to.be.null;

        await program.methods
          .voteV0({
            choice: 1,
          })
          .accounts({
            mint,
            proposal,
            nftVoter,
          })
          .rpcAndKeys({ skipPreflight: true });

        acct = await proposalProgram.account.proposalV0.fetch(proposal!);
        expect(acct.choices[1].weight.toNumber()).to.eq(1);
        markerA = await program.account.voteMarkerV0.fetchNullable(marker!);
        expect(markerA?.choices).to.deep.eq([1]);
        expect(markerA?.proxyIndex).to.eq(0);
      });

      it("allows the original owner to unassign proxy", async () => {
        const toUnassignProxy = proxyAssignmentKey(
          proxyConfig!,
          mint,
          proxy.publicKey
        )[0];
        const myProxy = proxyAssignmentKey(
          proxyConfig!,
          mint,
          PublicKey.default
        )[0];
        await proxyProgram.methods
          .unassignProxyV0()
          .accounts({
            proxyAssignment: toUnassignProxy,
            prevProxyAssignment: myProxy,
            currentProxyAssignment: myProxy,
          })
          .rpc({ skipPreflight: true });

        expect(
          (
            await proxyProgram.account.proxyAssignmentV0.fetch(myProxy)
          ).nextVoter.toBase58()
        ).to.eq(PublicKey.default.toBase58());
        expect(
          await proxyProgram.account.proxyAssignmentV0.fetchNullable(
            toUnassignProxy
          )
        ).to.be.null;
      });
    });
  });
});
