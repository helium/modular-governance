import { NftDelegation } from "@helium/modular-governance-idls/lib/types/nft_delegation";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { nftDelegationResolvers } from "./resolvers";

export * from "./constants";
export * from "./pdas";
export * from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<NftDelegation>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const tokenVoter = new Program<NftDelegation>(
    idl as NftDelegation,
    programId,
    provider,
    undefined,
    () => nftDelegationResolvers
  ) as Program<NftDelegation>;

  return tokenVoter;
}
