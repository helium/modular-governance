import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";

export const organizationsProgramResolver: anchor.CustomAccountResolver<any> = resolveIndividual(
  async ({ path }) => {
    if (
      path[path.length - 1] === "organizationsProgram"
    ) {
      return PROGRAM_ID;
    }
  }
);

export const organizationsResolvers: anchor.CustomAccountResolver<any> =
  combineResolvers(organizationsProgramResolver);
