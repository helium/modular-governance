import { IdlTypes } from "@coral-xyz/anchor";
import { OrganizationWallet } from "@helium/modular-governance-idls/lib/types/organization_wallet";
import { AccountMeta, PublicKey, Transaction, TransactionInstruction } from "@solana/web3.js";

export * from "./constants";
export { init } from "./init";
export * from "./pdas";
export * from "./resolvers";
export { executeTransaction } from "./functions/executeTransaction";

export type CompiledTransactionArgV0 =
  IdlTypes<OrganizationWallet>["CompiledTransactionArgV0"];

export function compileTransaction(
  instructions: TransactionInstruction[],
  signerSeeds: Buffer[][]
): { transaction: CompiledTransactionArgV0, remainingAccounts: AccountMeta[] } {
  let pubkeysToMetadata: Record<
    string,
    { isSigner: boolean; isWritable: boolean }
  > = {};
  instructions.forEach((ix) => {
    pubkeysToMetadata[ix.programId.toBase58()] ||= {
      isSigner: false,
      isWritable: false
    }
    ix.keys.forEach((k) => {
      pubkeysToMetadata[k.pubkey.toBase58()] = {
        isWritable:
          pubkeysToMetadata[k.pubkey.toBase58()]?.isWritable || k.isWritable,
        isSigner:
          pubkeysToMetadata[k.pubkey.toBase58()]?.isSigner || k.isSigner,
      };
    });
  });

  // Writable signers first. Then ro signers. Then rw non signers. Then ro
  const sortedAccounts = Object.keys(pubkeysToMetadata).sort((a, b) => {
    const aMeta = pubkeysToMetadata[a];
    const bMeta = pubkeysToMetadata[b];

    if (aMeta.isSigner && bMeta.isSigner) {
      if (aMeta.isWritable) {
        return -1;
      } else if (bMeta.isWritable) {
        return 1;
      } else {
        return 0;
      }
    } else if (bMeta.isSigner) {
      return 1;
    } else if (aMeta.isSigner) {
      return -1;
    } else if (aMeta.isWritable && bMeta.isWritable) {
      return 0;
    } else if (aMeta.isWritable) {
      return -1;
    } else if (bMeta.isWritable) {
      1;
    } else {
      return 0;
    }
  });

  let numRwSigners = 0;
  let numRoSigners = 0;
  let numRw = 0;
  sortedAccounts.forEach((k) => {
    const { isWritable, isSigner } = pubkeysToMetadata[k];
    if (isSigner && isWritable) {
      numRwSigners++;
    } else if (isSigner && !isWritable) {
      numRoSigners++;
    } else if (isWritable) {
      numRw++;
    }
  });
  const accountsToIndex = sortedAccounts.reduce((acc, k, i) => {
    acc[k] = i;
    return acc;
  }, {} as Record<string, number>);

  return {
    remainingAccounts: sortedAccounts.map((k) => {
      return {
        pubkey: new PublicKey(k),
        isSigner: false,
        isWritable: false
      }
    }),
    transaction: {
      numRoSigners,
      numRwSigners,
      numRw,
      instructions: instructions.map((ix) => {
        return {
          programIdIndex: accountsToIndex[ix.programId.toBase58()],
          accounts: Buffer.from(
            ix.keys.map((k) => accountsToIndex[k.pubkey.toBase58()])
          ),
          data: Buffer.from(ix.data),
        };
      }),
      signerSeeds,
    },
  };
}
