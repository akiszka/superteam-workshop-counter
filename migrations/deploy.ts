// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FirstProgram } from "../target/types/first_program";

module.exports = async function (provider: anchor.AnchorProvider) {
  anchor.setProvider(provider);

  const program = anchor.workspace.firstProgram as Program<FirstProgram>;

  const initializeIx = await program.methods
    .initialize()
    .accounts({
      signer: provider.publicKey,
    })
    .instruction();

  console.log(initializeIx);

  const tx = await program.methods
    .increment()
    .accounts({
      signer: provider.publicKey,
    })
    .preInstructions([initializeIx])
    .rpc({
      commitment: "confirmed",
      preflightCommitment: "confirmed"
    });

  console.log("tx:", tx);
};
