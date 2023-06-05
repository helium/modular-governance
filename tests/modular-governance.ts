import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ModularGovernance } from "../target/types/modular_governance";

describe("proposal", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Proposal as Program<ModularGovernance>;

});
