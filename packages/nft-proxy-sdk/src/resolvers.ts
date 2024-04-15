import * as anchor from "@coral-xyz/anchor";
import { combineResolvers, resolveIndividual } from "@helium/anchor-resolvers";
import { PublicKey } from "@solana/web3.js";
import { getAccount } from "@solana/spl-token";

export const nftProxyResolvers: anchor.CustomAccountResolver<any> =
  combineResolvers(
    resolveIndividual(
      async ({ path, provider, accounts, idlIx, programId }) => {
        if (
          path[path.length - 1] === "tokenAccount" &&
          accounts.asset
        ) {
          return (
            await provider.connection.getTokenLargestAccounts(
              accounts.asset as PublicKey
            )
          ).value[0].address;
        } else if (
          path[path.length - 1] === "owner" &&
          (idlIx.name === "assignProxyV0" || idlIx.name === "unassignProxyV0") &&
          accounts.tokenAccount &&
          accounts.approver
        ) {
          const ta = await getAccount(
            provider.connection,
            accounts.tokenAccount as PublicKey
          );
          // Primary proxy, owner is default pubkey
          if (ta.owner.equals(accounts.approver as PublicKey)) {
            return PublicKey.default;
          }

          return accounts.approver as PublicKey;
        }
      }
    )
  );
