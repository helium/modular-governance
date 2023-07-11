import { TokenVoter } from "@helium/modular-governance-idls/lib/types/token_voter";
import { BN, Program } from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { receiptKey } from "../pdas";
import {
  TOKEN_PROGRAM_ID,
  createInitializeMintInstruction,
} from "@solana/spl-token";

async function createMintInstructions(
  provider,
  decimals,
  mintAuthority,
  freezeAuthority = null,
  mintKeypair = Keypair.generate()
) {
  const mintKey = mintKeypair.publicKey;
  return [
    SystemProgram.createAccount({
      fromPubkey: provider.wallet.publicKey,
      newAccountPubkey: mintKey,
      space: 82,
      lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
      programId: TOKEN_PROGRAM_ID,
    }),
    await createInitializeMintInstruction(
      mintKeypair.publicKey,
      decimals,
      mintAuthority,
      freezeAuthority
    ),
  ];
}

/// Anchor wrapper function to facilitate deposits
export async function deposit({
  program,
  amount,
  metadataUri,
  // @ts-ignore
  recipient = program.provider.wallet.publicKey,
  tokenVoter,
  ...rest
}: {
  program: Program<TokenVoter>;
  metadataUri: string;
  amount: BN;
  tokenVoter: PublicKey;
  recipient?: PublicKey;
} | any) {
  const mintKeypair = Keypair.generate();
  const receipt = receiptKey(mintKeypair.publicKey)[0];
  return program.methods
    .depositV0({
      amount,
      metadataUri,
    })
    .preInstructions(
      await createMintInstructions(program.provider, 0, receipt, receipt, mintKeypair)
    )
    .accounts({
      tokenVoter,
      mint: mintKeypair.publicKey,
      recipient,
      ...rest
    })
    .signers([mintKeypair])
}
