import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export function receiptKey(mint: PublicKey, programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("receipt", "utf-8"),
      mint.toBuffer()
    ],
    programId
  );
}

