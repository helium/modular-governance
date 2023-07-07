import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export function resolutionSettingsKey(
  name: String,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("resolution_settings", "utf-8"), Buffer.from(name, "utf-8")],
    programId
  );
}
