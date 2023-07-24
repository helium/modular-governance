import { NftVoter } from "@helium/modular-governance-idls/lib/types/nft_voter";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { nftVoterResolvers } from "./resolvers";

export * from "./constants";
export * from "./pdas";
export * from "./resolvers";
export { deposit } from "./functions/deposit";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<NftVoter>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const tokenVoter = new Program<NftVoter>(
    idl as NftVoter,
    programId,
    provider,
    undefined,
    () => nftVoterResolvers
  ) as Program<NftVoter>;

  return tokenVoter;
}
