import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { choiceTransactionKey, organizationWalletKey, walletProposalKey } from "./pdas";
import { PublicKey } from "@solana/web3.js";
import { CustomAccountResolverFactory } from ".";

export const organizationWalletProgramResolver: CustomAccountResolverFactory<any> =
  (programId: PublicKey) =>
    resolveIndividual(async ({ path }) => {
      if (path[path.length - 1] === "organizationWalletProgram") {
        return programId;
      }
    });

export const organizationWalletResolvers: CustomAccountResolverFactory<any> =
  (programId: PublicKey) =>
    combineResolvers(
      organizationWalletProgramResolver(programId),
      resolveIndividual(async ({ path, accounts, args }) => {
        switch (path[path.length - 1]) {
          case "choiceTransaction":
            if (accounts.walletProposal) {
              return choiceTransactionKey(
                accounts.walletProposal as PublicKey,
                args[0].choiceIndex,
                args[0].transactionIndex,
                programId,
              )[0];
            }
            break;
          case "organizationWallet":
            if (accounts.organization) {
              return organizationWalletKey(
                accounts.organization as PublicKey,
                args[0].index,
                programId,
              )[0];
            }
          case "walletProposal":
            if (accounts.organizationWallet && accounts.proposal) {
              return walletProposalKey(
                accounts.organizationWallet as PublicKey,
                accounts.proposal as PublicKey,
                programId,
              )[0];
            }
        }
      })
    );
