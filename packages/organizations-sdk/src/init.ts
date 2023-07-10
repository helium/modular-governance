import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { Organization } from "@helium/modular-governance-idls/lib/types/organization";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { organizationsResolvers } from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<Organization>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const organizations = new Program<Organization>(
    idl as Organization,
    programId,
    provider,
    undefined,
    () => organizationsResolvers
  ) as Program<Organization>;

  return organizations;
}
