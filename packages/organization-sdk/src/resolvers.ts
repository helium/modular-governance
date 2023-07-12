import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";
import { init } from "./init";
import { PublicKey } from "@solana/web3.js";
import { proposalKey } from "./pdas";

export const organizationsProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "organizationProgram") {
      return PROGRAM_ID;
    }
  });

export const organizationsResolvers: anchor.CustomAccountResolver<any> =
  combineResolvers(
    organizationsProgramResolver,
    resolveIndividual(async ({ path, accounts, provider, programId }) => {
      if (path[path.length - 1] === "proposal" && accounts.organization) {
        const program = await init(
          provider as anchor.AnchorProvider,
          programId
        );
        const org = await program.account.organizationV0.fetch(
          accounts.organization as PublicKey
        );
        return proposalKey(
          accounts.organization as PublicKey,
          org.numProposals
        )[0];
      } else if (path[path.length - 1] === "proposalConfig") {
        const program = await init(
          provider as anchor.AnchorProvider,
          programId
        );
        const org = await program.account.organizationV0.fetch(
          accounts.organization as PublicKey
        );
        return org.defaultProposalConfig;
      } else if (path[path.length - 1] == "owner") {
        if ((provider as anchor.AnchorProvider).wallet) {
          return (provider as anchor.AnchorProvider).wallet.publicKey;
        }
      }
    })
  );
