import { Organizations } from "@helium/modular-governance-idls/lib/types/organizations";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { PROGRAM_ID } from "./constants";
import { organizationsResolvers } from "./resolvers";


export * from "./constants";
export * from "./pdas";
export * from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<Organizations>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const organizations = new Program<Organizations>(
    idl as Organizations,
    programId,
    provider,
    undefined,
    () => organizationsResolvers
  ) as Program<Organizations>;

  return organizations;
}
