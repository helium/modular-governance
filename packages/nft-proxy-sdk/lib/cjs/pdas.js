"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.proxyAssignmentKey = exports.proxyConfigKey = void 0;
const web3_js_1 = require("@solana/web3.js");
const constants_1 = require("./constants");
function proxyConfigKey(name, programId = constants_1.PROGRAM_ID) {
    return web3_js_1.PublicKey.findProgramAddressSync([Buffer.from("proxy_config", "utf-8"), Buffer.from(name, "utf8")], programId);
}
exports.proxyConfigKey = proxyConfigKey;
function proxyAssignmentKey(proxyConfig, mint, voter, programId = constants_1.PROGRAM_ID) {
    return web3_js_1.PublicKey.findProgramAddressSync([
        Buffer.from("proxy_assignment", "utf-8"),
        proxyConfig.toBuffer(),
        mint.toBuffer(),
        voter.toBuffer(),
    ], programId);
}
exports.proxyAssignmentKey = proxyAssignmentKey;
//# sourceMappingURL=pdas.js.map