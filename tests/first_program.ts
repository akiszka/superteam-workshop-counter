import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FirstProgram } from "../target/types/first_program";
import { expect } from "chai";

describe("first_program", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.firstProgram as Program<FirstProgram>;

  function getCounterPubkey(signer: anchor.web3.PublicKey) {
    const [pubkey, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("counter"), signer.toBuffer()],
      program.programId,
    );

    return pubkey;
  }

  it("Can initialize", async () => {
    const tx = await program.methods.initialize().accounts({}).rpc();
    console.log("Your transaction signature", tx);
  });

  it("Can increment/decrement", async () => {
    await program.methods.increment().accounts({}).rpc();
  });

  it("Has correct account", async () => {
    const counterPubkey = getCounterPubkey(provider.publicKey);
    const counterState = await program.account.counterState.fetch(
      counterPubkey,
    );

    expect(counterState.authority.toBase58()).to.equal(
      provider.publicKey.toBase58(),
    );
    expect(counterState.version).to.equal(1);
    expect(counterState.value.toString()).to.equal("1");
  });

  it("Increment costs money", async () => {
    const preBalance = await provider.connection.getBalance(provider.publicKey);
    await program.methods.increment().accounts({}).rpc();
    const postBalance = await provider.connection.getBalance(provider.publicKey);

    const expectedFee = 1000000;
    const delta = preBalance - postBalance;
    expect(Math.abs(delta - expectedFee)).to.be.lessThan(10_000);
  });
});
