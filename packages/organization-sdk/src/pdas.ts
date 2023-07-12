import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID  } from "./constants";
import { PROGRAM_ID as PROPOSAL_PROGRAM_ID } from "@helium/proposal-sdk";

export function proposalKey(
  organization: PublicKey,
  index: number,
  programId: PublicKey = PROPOSAL_PROGRAM_ID
): [PublicKey, number] {
  const buf = Buffer.alloc(4);
  buf.writeUInt32LE(index, 0);
  return PublicKey.findProgramAddressSync(
    [Buffer.from("proposal", "utf-8"), organization.toBuffer(), buf],
    programId
  );
}

export function organizationKey(
  name: String,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("organization", "utf-8"), Buffer.from(name, "utf-8")],
    programId
  );
}

