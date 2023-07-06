import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export function proposalKey(
  owner: PublicKey,
  seed: Buffer,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("proposal", "utf-8"), owner.toBuffer(), seed],
    programId
  );
}

