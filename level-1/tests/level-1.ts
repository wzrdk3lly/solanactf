import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Level1 } from "../target/types/level_1";

describe("level-1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Level1 as Program<Level1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
