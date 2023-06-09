import { StateController } from "@helium/modular-governance-idls/lib/types/state_controller";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";
import { stateControllerResolvers } from "./resolvers";


export * from "./constants";
export * from "./pdas";
export * from "./resolvers";

export async function init(
  provider: AnchorProvider,
  programId: PublicKey = PROGRAM_ID,
  idl?: Idl | null
): Promise<Program<StateController>> {
  if (!idl) {
    idl = await Program.fetchIdl(programId, provider);
  }

  const stateController = new Program<StateController>(
    idl as StateController,
    programId,
    provider,
    undefined,
    () => stateControllerResolvers
  ) as Program<StateController>;

  return stateController;
}
