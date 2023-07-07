import { ataResolver, combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PROGRAM_ID } from "./constants";
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export const tokenVoterProgramResolver: anchor.CustomAccountResolver<any> =
  resolveIndividual(async ({ path }) => {
    if (path[path.length - 1] === "tokenVoterProgram") {
      return PROGRAM_ID;
    } else if (path[path.length - 1] == "tokenMetadataProgram") {
      return new PublicKey("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
    }
  });

export const tokenVoterResolvers: anchor.CustomAccountResolver<any> = combineResolvers(
  tokenVoterProgramResolver,
  resolveIndividual(async ({ path, provider, accounts, programId }) => {
    if (path[path.length - 1] == "proposalProgram" && accounts.proposal) {
      const acct = await provider.connection.getAccountInfo(
        accounts.proposal as PublicKey
      );
      return acct.owner;
    }
  }),
  ataResolver({
    instruction: "initializeTokenVoterV0",
    account: "tokenAccount",
    mint: "collection",
    owner: "tokenVoter",
  }),
  ataResolver({
    instruction: "depositV0",
    account: "receiptTokenAccount",
    mint: "mint",
    owner: "recipient",
  }),
  ataResolver({
    instruction: "depositV0",
    account: "tokenAccount",
    mint: "depositMint",
    owner: "payer",
  }),
  ataResolver({
    instruction: "depositV0",
    account: "vault",
    mint: "depositMint",
    owner: "receipt",
  }),
  ataResolver({
    instruction: "withdrawV0",
    account: "tokenAccount",
    mint: "depositMint",
    owner: "owner",
  }),
  ataResolver({
    instruction: "withdrawV0",
    account: "vault",
    mint: "depositMint",
    owner: "receipt",
  }),
  ataResolver({
    instruction: "withdrawV0",
    account: "receiptTokenAccount",
    mint: "mint",
    owner: "owner",
  }),
  ataResolver({
    instruction: "voteV0",
    account: "tokenAccount",
    mint: "mint",
    owner: "voter",
  }),
  ataResolver({
    instruction: "relinquishVoteV0",
    account: "tokenAccount",
    mint: "mint",
    owner: "voter",
  })
);
