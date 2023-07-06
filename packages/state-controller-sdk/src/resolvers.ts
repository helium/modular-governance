import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export const stateControllerProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "stateControllerProgram") {
      return PROGRAM_ID;
    }
  });

export const stateControllerResolvers: anchor.CustomAccountResolver<any> =
  combineResolvers(
    stateControllerProgramResolver,
    resolveIndividual(async ({ path, provider, accounts, programId }) => {
      if (path[path.length - 1] == "proposalProgram" && accounts.proposal) {
        const acct = await provider.connection.getAccountInfo(accounts.proposal as PublicKey);
        return acct.owner
      }
    })
  );
