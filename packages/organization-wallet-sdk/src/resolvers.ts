import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";
import { choiceTransactionKey } from "./pdas";
import { PublicKey } from "@solana/web3.js";

export const organizationsProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "organizationWalletProgram") {
      return PROGRAM_ID;
    }
  });

export const organizationsResolvers: anchor.CustomAccountResolver<any> =
  combineResolvers(
    organizationsProgramResolver,
    resolveIndividual(async ({ path, accounts, programId, args }) => {
      if (
        path[path.length - 1] == "choiceTransaction" &&
        accounts.walletProposal
      ) {
        return choiceTransactionKey(
          accounts.walletProposal as PublicKey,
          args[0].choiceIndex,
          args[0].transactionIndex,
          programId
        )[0];
      }
    })
  );
