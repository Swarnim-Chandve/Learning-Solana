import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day_2";

describe("day_2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day2 as Program<Day2>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(
        new anchor.BN(777), new anchor.BN(888), "hello Swarnim").rpc();
    console.log("Your transaction signature", tx);
  });

  // it("Array test", async () => {
  //   const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888)]).rpc();
  //   console.log("Your transaction signature", tx);
  // });

    // it("subtraction overflow", async() => {
    //   const tx = await program.methods.sub(
    //     new anchor.BN(0), new anchor.BN(1)
    //   ).rpc();
    //   console.log("Your transaction signature", tx);
    // });

    it("Addition", async () => {
      const tx = await program.methods.add(
        new anchor.BN(10), new anchor.BN(12)
      ).rpc();
      console.log("Your transaction signature", tx);
    })

    it("subtraction overflow", async () => {
      const tx = await program.methods.sub(
        new anchor.BN(0), new anchor.BN(1)
      ).rpc();
      console.log("Your transaction signature", tx);
    });


    it("Multiplication", async () => {
      const tx = await program.methods.mul(
        new anchor.BN(45), new anchor.BN(12)
      ).rpc();
      console.log("Your transaction signature", tx);
    });


    it("Division", async () => {
      const tx = await program.methods.div(
        new anchor.BN(35), new anchor.BN(12)
      ).rpc();
      console.log("Your transaction signature", tx);
    });


    // it("Square Root", async () => {
    //   const tx = await program.methods.sqrt(
    //     new anchor.BN(36)
    //   ).rpc();
    //   console.log("Your transaction signature", tx);
    // });

    it("Square Root", async () => {
      const input = new anchor.BN(36);
      const tx = await program.methods.sqrt(input).rpc();
      console.log(`Computed sqrt of ${input.toString()}, TX: ${tx}`);
    });



    it("Log10", async () => {
      const tx = await program.methods.log10(new anchor.BN(1000)).rpc();
      console.log("Log10 TX signature:", tx);
    });


    
});
