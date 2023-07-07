import { TokenVoter } from "@helium/modular-governance-idls/lib/types/token_voter";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { tokenVoterResolvers } from "./resolvers";


export * from "./constants";
export * from "./pdas";
export * from "./resolvers";
export { deposit } from "./functions/deposit";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<TokenVoter>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const tokenVoter = new Program<TokenVoter>(
    idl as TokenVoter,
    programId,
    provider,
    undefined,
    () => tokenVoterResolvers
  ) as Program<TokenVoter>;

  return tokenVoter;
}
