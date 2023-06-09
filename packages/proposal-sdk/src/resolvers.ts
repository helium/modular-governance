import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";

export const proposalProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "proposalProgram") {
      return PROGRAM_ID;
    }
  });

export const proposalResolvers: anchor.CustomAccountResolver<any> = combineResolvers(
  proposalProgramResolver
);
