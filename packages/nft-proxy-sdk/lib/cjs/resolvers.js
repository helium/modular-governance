"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.nftProxyResolvers = void 0;
const anchor_resolvers_1 = require("@helium/anchor-resolvers");
const web3_js_1 = require("@solana/web3.js");
const spl_token_1 = require("@solana/spl-token");
exports.nftProxyResolvers = (0, anchor_resolvers_1.combineResolvers)((0, anchor_resolvers_1.resolveIndividual)(({ path, provider, accounts, idlIx, programId }) => __awaiter(void 0, void 0, void 0, function* () {
    if (path[path.length - 1] === "tokenAccount" &&
        accounts.asset) {
        return (yield provider.connection.getTokenLargestAccounts(accounts.asset)).value[0].address;
    }
    else if (path[path.length - 1] === "voter" &&
        (idlIx.name === "assignProxyV0" || idlIx.name === "unassignProxyV0") &&
        accounts.tokenAccount &&
        accounts.approver) {
        const ta = yield (0, spl_token_1.getAccount)(provider.connection, accounts.tokenAccount);
        // Primary proxy, owner is default pubkey
        if (ta.owner.equals(accounts.approver)) {
            return web3_js_1.PublicKey.default;
        }
        return accounts.approver;
    }
})));
//# sourceMappingURL=resolvers.js.map