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
  resolveIndividual(async ({ path, provider, args, accounts, programId }) => {
    if (path[path.length - 1] === "proposal" && accounts.owner) {
      return proposalKey(
        accounts.namespace as PublicKey,
        args[0].seed,
        programId
      )[0];
    } else if (path[path.length - 1] == "owner") {
      if ((provider as anchor.AnchorProvider).wallet) {
        return (provider as anchor.AnchorProvider).wallet.publicKey;
      }
    }
  })
);
