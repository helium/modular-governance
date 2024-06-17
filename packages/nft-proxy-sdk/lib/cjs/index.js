"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
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
exports.init = void 0;
const anchor_1 = require("@coral-xyz/anchor");
const constants_1 = require("./constants");
const resolvers_1 = require("./resolvers");
__exportStar(require("./constants"), exports);
__exportStar(require("./pdas"), exports);
__exportStar(require("./resolvers"), exports);
function init(provider, programId = constants_1.PROGRAM_ID, idl) {
    return __awaiter(this, void 0, void 0, function* () {
        if (!idl) {
            idl = yield anchor_1.Program.fetchIdl(programId, provider);
        }
        const tokenVoter = new anchor_1.Program(idl, programId, provider, undefined, () => resolvers_1.nftProxyResolvers);
        return tokenVoter;
    });
}
exports.init = init;
//# sourceMappingURL=index.js.map