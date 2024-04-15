import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { OrganizationWallet } from "@helium/modular-governance-idls/lib/types/organization_wallet";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { organizationWalletResolvers } from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<OrganizationWallet>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const organizationWalletProgram = new Program<OrganizationWallet>(
    idl as OrganizationWallet,
    programId,
    provider,
    undefined,
    () => organizationWalletResolvers(programId)
  ) as Program<OrganizationWallet>;

  return organizationWalletProgram;
}
