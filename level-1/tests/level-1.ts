import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Level1 } from "../target/types/level_1";


describe("level-1", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.local();

  const program = anchor.workspace.Level1 as Program<Level1>;
  const increment = anchor.web3.Keypair.generate();

  it("Initialize the data", async () => {
    // Use this
    const tx = await program.rpc.initialize({
      accounts:{
        myAccountBuffer: increment.publicKey,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      },
      signers:[increment]
      
    });
    let incrementAccount = await program.account.myAccountBuffer.fetch(increment.publicKey)
    console.log("initialize the account --> ", incrementAccount)
  });

});

