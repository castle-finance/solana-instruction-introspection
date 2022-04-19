import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SolInsIntrospection } from "../target/types/sol_ins_introspection";

describe("sol-ins-introspection", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.SolInsIntrospection as Program<SolInsIntrospection>;

  it("Is initialized!", async () => {
    const [basicPda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("basic"), program.provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    console.log("State account", basicPda.toString());

    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        authority: program.provider.wallet.publicKey,
        stateAccount: basicPda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const ic = await program.methods
      .increment()
      .accounts({
        authority: program.provider.wallet.publicKey,
        stateAccount: basicPda,
        instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
      })
      .postInstructions([
        await program.methods
          .updateTimestamp()
          .accounts({
            authority: program.provider.wallet.publicKey,
            stateAccount: basicPda,
          })
          .instruction(),
      ])
      .rpc();
  });
});
