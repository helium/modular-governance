import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "@helium/proposal-sdk";

export function proposalKey(organization: PublicKey, index: number, programId: PublicKey = PROGRAM_ID): [PublicKey, number] {
  const buf = Buffer.alloc(4)
  buf.writeUInt32LE(index, 0)
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("proposal", "utf-8"),
      organization.toBuffer(),
      buf
    ],
    programId
  );
}

