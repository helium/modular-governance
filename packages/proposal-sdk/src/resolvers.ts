import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { proposalKey } from "./pdas";

export const proposalProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "proposalProgram") {
      return PROGRAM_ID;
    }
  });

export const proposalResolvers: anchor.CustomAccountResolver<any> = combineResolvers(
  proposalProgramResolver,
  resolveIndividual(async ({ path, args, accounts, programId }) => {
    if (path[path.length - 1] === "proposal" && accounts.owner) {
      return proposalKey(accounts.owner as PublicKey, args[0].seed, programId)[0]
    }
  })
);
