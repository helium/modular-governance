import { Proposal } from "@helium/modular-governance-idls/lib/types/proposal";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import BN from "bn.js";
import { PROGRAM_ID } from "./constants";
import { proposalResolvers } from "./resolvers";


export * from "./constants";
export * from "./pdas";
export * from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<Proposal>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const proposal = new Program<Proposal>(
    idl as Proposal,
    programId,
    provider,
    undefined,
    () => proposalResolvers
  ) as Program<Proposal>;

  return proposal;
}
