import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export function delegationKey(mint: PublicKey, owner: PublicKey, programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("delegation", "utf-8"),
      mint.toBuffer(),
      owner.toBuffer()
    ],
    programId
  );
}

