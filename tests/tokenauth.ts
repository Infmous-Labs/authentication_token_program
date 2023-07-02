import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Tokenauth } from "../target/types/tokenauth";

describe("tokenauth", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Tokenauth as Program<Tokenauth>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
