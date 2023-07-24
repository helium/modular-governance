import { PublicKey } from "@solana/web3.js";
import { PROGRAM_ID  } from "./constants";

export function walletProposalKey(
  organizationWallet: PublicKey,
  proposal: PublicKey,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("wallet_proposal", "utf-8"), organizationWallet.toBuffer(), proposal.toBuffer()],
    programId
  );
}

export function choiceTransactionKey(
  walletProposal: PublicKey,
  choiceIndex: number,
  transactionIndex: number,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  const choiceIndexBuf = Buffer.alloc(2);
  const transactionIndexBuf = Buffer.alloc(2);
  choiceIndexBuf.writeUInt16LE(choiceIndex);
  transactionIndexBuf.writeUInt16LE(transactionIndex);
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from("choice_transaction", "utf-8"),
      walletProposal.toBuffer(),
      choiceIndexBuf,
      transactionIndexBuf,
    ],
    programId
  );
}

export function organizationWalletKey(
  organization: PublicKey,
  index: number,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  const ix = Buffer.alloc(2);
  ix.writeUInt16LE(index, 0);
  return PublicKey.findProgramAddressSync(
    [Buffer.from("organization_wallet", "utf-8"), organization.toBuffer(), ix],
    programId
  );
}

export function walletKey(
  organization: PublicKey,
  index: number,
  programId: PublicKey = PROGRAM_ID
): [PublicKey, number] {
  const ix = Buffer.alloc(2);
  ix.writeUInt16LE(index, 0);
  return PublicKey.findProgramAddressSync(
    [Buffer.from("wallet", "utf-8"), organization.toBuffer(), ix],
    programId
  );
}


