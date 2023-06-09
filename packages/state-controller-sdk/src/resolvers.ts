import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";

export const stateControllerProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "stateControllerProgram") {
      return PROGRAM_ID;
    }
  });

export const stateControllerResolvers: anchor.CustomAccountResolver<any> =
  combineResolvers(stateControllerProgramResolver);
