import * as anchor from "@coral-xyz/anchor";
import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PublicKey } from "@solana/web3.js";
import { delegationKey } from "./pdas";

export const nftDelegationResolvers: anchor.CustomAccountResolver<any> = combineResolvers(
  resolveIndividual(async ({ path, provider, accounts, idlIx }) => {
    if (path[path.length - 1] === "tokenAccount" && idlIx.name === "delegateV0" && accounts.mint) {
       return (await provider.connection.getTokenLargestAccounts(
        accounts.mint as PublicKey,
      )).value[0].address;
    } else if (path[path.length - 1] === "delegation" && accounts.owner && accounts.mint) {
      return delegationKey(accounts.mint as PublicKey, accounts.owner as PublicKey)[0]
    }
  }),
);
