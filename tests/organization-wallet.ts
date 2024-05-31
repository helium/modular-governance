import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Proposal } from "../target/types/proposal";
import { Organization } from "../target/types/organization";
import { OrganizationWallet } from "../target/types/organization_wallet";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  PROGRAM_ID as PROPOSAL_PID,
  init as initProposal,
} from "@helium/proposal-sdk";
import {
  PROGRAM_ID as ORG_PROGRAM_ID,
  init as initOrg,
} from "@helium/organization-sdk";
import {
  compileTransaction,
  executeTransaction,
  init,
  PROGRAM_ID,
  walletKey,
} from "@helium/organization-wallet-sdk";
import { expect } from "chai";
import { ensureIdls, makeid } from "./utils";
import {
  getAssociatedTokenAddressSync,
  createAssociatedTokenAccountInstruction,
  createTransferInstruction,
} from "@solana/spl-token";
import {
  createAtaAndMint,
  createMint,
  sendInstructions,
} from "@helium/spl-utils";

describe("organization wallet", () => {
  anchor.setProvider(anchor.AnchorProvider.local("http://127.0.0.1:8899"));
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const me = provider.wallet.publicKey;
  let proposalProgram: Program<Proposal>;
  let organizationProgram: Program<Organization>;
  let program: Program<OrganizationWallet>;

  let name: string;
  beforeEach(async () => {
    await ensureIdls();

    name = makeid(10);
    organizationProgram = await initOrg(
      provider,
      ORG_PROGRAM_ID,
      anchor.workspace.Organization.idl
    );
    program = await init(
      provider,
      PROGRAM_ID,
      anchor.workspace.OrganizationWallet.idl
    );

    // @ts-ignore
    proposalProgram = await initProposal(
      provider,
      PROPOSAL_PID,
      anchor.workspace.Proposal.idl
    );
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
      } = await organizationProgram.methods
        .initializeOrganizationV0({
          name,
          authority: me,
          defaultProposalConfig: proposalConfig!,
          proposalProgram: proposalProgram.programId,
          uri: "https://example.com",
        })
        .rpcAndKeys({ skipPreflight: true }));
    });

    it("initializes an organization wallet", async () => {
      const {
        pubkeys: { organizationWallet },
      } = await program.methods
        .initializeOrganizationWalletV0({
          index: 0,
          name: "My Wallet",
          proposalConfigs: [proposalConfig!],
        })
        .accounts({
          organization,
        })
        .rpcAndKeys({ skipPreflight: true });

      const acct = await program.account.organizationWalletV0.fetch(
        organizationWallet!
      );
      expect(acct.index).to.eq(0);
      expect(acct.organization.toBase58()).to.eq(organization?.toBase58());
      expect(acct.wallet.toBase58()).to.eq(
        walletKey(organization!, 0)[0].toBase58()
      );
      expect(acct.proposalConfigs[0].toBase58()).to.eq(
        proposalConfig?.toBase58()
      );
      expect(acct.name).to.eq("My Wallet");
    });

    describe("with organization wallet and proposal", async () => {
      let organizationWallet: PublicKey | undefined;
      let proposal: PublicKey | undefined;

      beforeEach(async () => {
        ({
          pubkeys: { organizationWallet },
        } = await program.methods
          .initializeOrganizationWalletV0({
            index: 0,
            name: "My Wallet",
            proposalConfigs: [proposalConfig!],
          })
          .accounts({
            organization,
          })
          .rpcAndKeys({ skipPreflight: true }));
        ({
          pubkeys: { proposal },
        } = await organizationProgram.methods
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
      });

      it("allows attaching and executing instructions on proposals", async () => {
        const wallet = walletKey(organization!, 0)[0];
        // to pay for my ata
        await sendInstructions(provider, [
          SystemProgram.transfer({
            fromPubkey: me,
            toPubkey: wallet,
            lamports: 500000000000,
          }),
        ]);

        const mint = await createMint(provider, 0, me, me);
        const lazySignerAta = await createAtaAndMint(
          provider,
          mint,
          10,
          wallet
        );
        const myAta = getAssociatedTokenAddressSync(mint, me);

        // Transfer some tokens from lazy signer to me
        const instructions: TransactionInstruction[] = [
          createAssociatedTokenAccountInstruction(wallet, myAta, me, mint),
          createTransferInstruction(lazySignerAta, myAta, wallet, 10),
        ];

        const { transaction, remainingAccounts } = await compileTransaction(
          instructions,
          []
        );
        const {
          pubkeys: { choiceTransaction },
        } = await program.methods
          .setTransactionsV0({
            choiceIndex: 0,
            transactionIndex: 0,
            transaction,
            allowExecutionOffset: 0,
            // Stop allowing execution after 1 week
            disableExecutionOffset: 60 * 60 * 24 * 7,
          })
          .remainingAccounts(remainingAccounts)
          .accounts({
            proposal,
            organizationWallet,
          })
          .rpcAndKeys({ skipPreflight: true });

        await proposalProgram.methods
          .updateStateV0({
            newState: {
              resolved: {
                choices: [0],
                endTs: new anchor.BN(
                  Math.floor(new Date().valueOf() / 1000) - 100
                ),
              },
            },
          })
          .accounts({
            proposal,
          })
          .rpc({ skipPreflight: true });

        await (
          await executeTransaction({
            program,
            choiceTransaction: choiceTransaction!,
          })
        ).rpc({ skipPreflight: true });
      });

      describe("update_organization_wallet_v0", () => {
        let otherName;
        let otherProposalConfig: PublicKey | undefined;

        beforeEach(async () => {
          otherName = makeid(10);

          ({
            pubkeys: { proposalConfig: otherProposalConfig },
          } = await proposalProgram.methods
            .initializeProposalConfigV0({
              name: otherName,
              voteController: me,
              stateController: me,
              onVoteHook: PublicKey.default,
            })
            .rpcAndKeys({ skipPreflight: true }));
        });

        it("should update the name", async () => {
          await program.methods
            .updateOrganizationWalletV0({
              name: otherName,
              proposalConfigs: null,
            })
            .accounts({
              organizationWallet,
              organization,
              authority: me,
            })
            .rpc({ skipPreflight: true });

          const acct = await program.account.organizationWalletV0.fetch(
            organizationWallet!
          );

          expect(acct.index).to.eq(0);
          expect(acct.name).to.eq(otherName);
          expect(acct.proposalConfigs).to.have.length(1);
          expect(acct.proposalConfigs[0].equals(proposalConfig!)).to.be;
        });

        it("should update proposal configs", async () => {
          await program.methods
            .updateOrganizationWalletV0({
              name: null,
              proposalConfigs: [otherProposalConfig!],
            })
            .accounts({
              organizationWallet,
              organization,
              authority: me,
            })
            .rpc({ skipPreflight: true });

          const acct = await program.account.organizationWalletV0.fetch(
            organizationWallet!
          );

          expect(acct.index).to.eq(0);
          expect(acct.name).to.eq("My Wallet");
          expect(acct.proposalConfigs).to.have.length(1);
          expect(acct.proposalConfigs[0].equals(otherProposalConfig!)).to.be;
        });

        it("should fail if not the authority", async () => {
          const authority = Keypair.generate().publicKey;

          await organizationProgram.methods
            .updateOrganizationV0({
              authority,
              defaultProposalConfig: null,
              proposalProgram: null,
              uri: null,
            })
            .accounts({
              organization,
              authority: me,
            })
            .rpc({ skipPreflight: true });

          try {
            await program.methods
              .updateOrganizationWalletV0({
                name: otherName,
                proposalConfigs: [otherProposalConfig!],
              })
              .accounts({
                organizationWallet,
                organization,
                authority: me,
              })
              .simulate();
          } catch (err) {
            expect(err.simulationResponse?.logs).to.match(
              /caused by account: organization\..*ConstraintHasOne/
            );
          }
        });

        it("should fail if wrong organization", async () => {
          const {
            pubkeys: { organization: otherOrganization },
          } = await organizationProgram.methods
            .initializeOrganizationV0({
              name: otherName,
              authority: me,
              defaultProposalConfig: proposalConfig!,
              proposalProgram: proposalProgram.programId,
              uri: "https://example.com",
            })
            .rpcAndKeys({ skipPreflight: true });

          try {
            await program.methods
              .updateOrganizationWalletV0({
                name: otherName,
                proposalConfigs: [otherProposalConfig!],
              })
              .accounts({
                organizationWallet,
                organization: otherOrganization,
                authority: me,
              })
              .simulate();
          } catch (err) {
            expect(err.simulationResponse?.logs).to.match(
              /caused by account: organization_wallet\..*InvalidOrganization/
            );
          }
        });
      });
    });
  });
});
