import { NftProxy } from "@helium/modular-governance-idls/lib/types/nft_proxy";
import { AnchorProvider, Idl, Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
export * from "./constants";
export * from "./pdas";
export * from "./resolvers";
export declare function init(provider: AnchorProvider, programId?: PublicKey, idl?: Idl | null): Promise<Program<NftProxy>>;
//# sourceMappingURL=index.d.ts.map