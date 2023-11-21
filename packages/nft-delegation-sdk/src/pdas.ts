import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export function delegationConfigKey(
  name: string,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("delegation_config", "utf-8"), Buffer.from(name, "utf8")],
    programId
  );
}

export function delegationKey(
  delegationConfig: PublicKey,
  mint: PublicKey,
  owner: PublicKey,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("delegation", "utf-8"),
      delegationConfig.toBuffer(),
      mint.toBuffer(),
      owner.toBuffer(),
    ],
    programId
  );
}
