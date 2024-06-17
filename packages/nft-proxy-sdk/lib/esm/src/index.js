import { Program } from "@coral-xyz/anchor";
import { PROGRAM_ID } from "./constants";
import { nftProxyResolvers } from "./resolvers";
export * from "./constants";
export * from "./pdas";
export * from "./resolvers";
export async function init(provider, programId = PROGRAM_ID, idl) {
    if (!idl) {
        idl = await Program.fetchIdl(programId, provider);
    }
    const tokenVoter = new Program(idl, programId, provider, undefined, () => nftProxyResolvers);
    return tokenVoter;
}
//# sourceMappingURL=index.js.map