import { NftVoter } from "@helium/modular-governance-idls/lib/types/nft_voter";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { nftVoterResolvers } from "./resolvers";

export * from "./constants";
export * from "./pdas";
export * from "./resolvers";
export { deposit } from "./functions/deposit";

/**
 * Init the NFT Voter Sdk example
 *
 * ```ts
 * import * as anchor from "@coral-xyz/anchor";
 * import { Program } from "@coral-xyz/anchor";
 * import { PROGRAM_ID, init } from "@helium/nft-voter-sdk";
 *
 * // run typedoc --help for a list of supported languages
 * // Configure the client to use the local cluster.
 * anchor.setProvider(anchor.AnchorProvider.env());
 * const provider = anchor.getProvider() as anchor.AnchorProvider;
 * const me = provider.wallet.publicKey;
 * let proposalProgram: Program<Proposal>;
 * let program: Program<NftVoter>;
 * program = await init(provider, PROGRAM_ID, anchor.workspace.NftVoter.idl);
 * ```
 */
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
