import { Program } from "@coral-xyz/anchor";
import { OrganizationWallet } from "@helium/modular-governance-idls/lib/types/organization_wallet";
import { PublicKey } from "@solana/web3.js";

export async function executeTransaction({
  program,
  choiceTransaction,
  refund,
}: {
  program: Program<OrganizationWallet>;
  choiceTransaction: PublicKey;
  refund?: PublicKey;
}) {
  const {
    organizationWallet,
    proposal,
    transaction: { numRwSigners, numRoSigners, numRw, accounts, signerSeeds },
  } = await program.account.choiceTransactionV0.fetch(choiceTransaction);

  const signers = new Set((signerSeeds as Buffer[][]).map((seed) => {
    return PublicKey.createProgramAddressSync(
      seed,
      program.programId
    )[0].toBase58();
  }));
  const { wallet } = await program.account.organizationWalletV0.fetch(
    organizationWallet
  );

  const remainingAccounts = accounts.map((acc, index) => {
    return {
      pubkey: acc,
      isWritable:
        index < numRwSigners ||
        (index >= numRwSigners + numRoSigners &&
          index < numRwSigners + numRoSigners + numRw),
      isSigner: !signers.has(acc) && !acc.equals(wallet) && index < numRwSigners + numRoSigners,
    };
  });

  return program.methods
    .executeTransactionV0()
    .accounts({
      choiceTransaction,
      organizationWallet,
      proposal,
      wallet,
      // @ts-ignore
      refund: refund || program.provider.wallet.publicKey,
    })
    .remainingAccounts(remainingAccounts);
}
