import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID } from "./constants";

export function proxyConfigKey(
  name: string,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("proxy_config", "utf-8"), Buffer.from(name, "utf8")],
    programId
  );
}

export function proxyKey(
  proxyConfig: PublicKey,
  mint: PublicKey,
  owner: PublicKey,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("proxy", "utf-8"),
      proxyConfig.toBuffer(),
      mint.toBuffer(),
      owner.toBuffer(),
    ],
    programId
  );
}
