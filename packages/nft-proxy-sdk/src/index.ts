import { NftProxy } from "@helium/modular-governance-idls/lib/types/nft_proxy";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { nftProxyResolvers } from "./resolvers";

export * from "./constants";
export * from "./pdas";
export * from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<NftProxy>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const tokenVoter = new Program<NftProxy>(
    idl as NftProxy,
    programId,
    provider,
    undefined,
    () => nftProxyResolvers
  ) as Program<NftProxy>;

  return tokenVoter;
}
