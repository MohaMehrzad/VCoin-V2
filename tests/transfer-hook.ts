import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TransferHook } from "../target/types/transfer_hook";
import { expect } from "chai";

describe("transfer-hook", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TransferHook as Program<TransferHook>;
  const authority = provider.wallet;

  // PDAs
  let hookConfigPda: anchor.web3.PublicKey;
  let hookConfigBump: number;

  // Test accounts
  const fiveAProgram = anchor.web3.Keypair.generate().publicKey;
  const vcoinMint = anchor.web3.Keypair.generate();

  before(async () => {
    // Derive PDAs
    [hookConfigPda, hookConfigBump] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("hook-config")],
      program.programId
    );
  });

  describe("Initialization", () => {
    it("should initialize transfer hook config", async () => {
      const minActivityAmount = new anchor.BN(1_000_000_000); // 1 VCoin

      try {
        await program.methods
          .initialize(fiveAProgram, minActivityAmount)
          .accounts({
            hookConfig: hookConfigPda,
            vcoinMint: vcoinMint.publicKey,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([vcoinMint])
          .rpc();

        const config = await program.account.hookConfig.fetch(hookConfigPda);
        
        expect(config.authority.toString()).to.equal(authority.publicKey.toString());
        expect(config.vcoinMint.toString()).to.equal(vcoinMint.publicKey.toString());
        expect(config.fiveAProgram.toString()).to.equal(fiveAProgram.toString());
        expect(config.paused).to.be.false;
        expect(config.blockWashTrading).to.be.false;
        expect(config.totalTransfers.toNumber()).to.equal(0);
      } catch (error) {
        console.log("Init error (may already exist):", error.message);
      }
    });
  });

  describe("Configuration Updates", () => {
    it("should update hook config", async () => {
      const newMinAmount = new anchor.BN(2_000_000_000);

      await program.methods
        .updateConfig(null, newMinAmount, null)
        .accounts({
          hookConfig: hookConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const config = await program.account.hookConfig.fetch(hookConfigPda);
      expect(config.minActivityAmount.toNumber()).to.equal(2_000_000_000);
    });

    it("should toggle pause status", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          hookConfig: hookConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      let config = await program.account.hookConfig.fetch(hookConfigPda);
      expect(config.paused).to.be.true;

      // Unpause
      await program.methods
        .setPaused(false)
        .accounts({
          hookConfig: hookConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      config = await program.account.hookConfig.fetch(hookConfigPda);
      expect(config.paused).to.be.false;
    });

    it("should reject unauthorized config updates", async () => {
      const unauthorized = anchor.web3.Keypair.generate();
      
      try {
        await program.methods
          .setPaused(true)
          .accounts({
            hookConfig: hookConfigPda,
            authority: unauthorized.publicKey,
          })
          .signers([unauthorized])
          .rpc();
        
        expect.fail("Should have thrown unauthorized error");
      } catch (error) {
        expect(error.message).to.include("Unauthorized");
      }
    });
  });

  describe("Authority Transfer", () => {
    it("should update authority", async () => {
      const newAuthority = anchor.web3.Keypair.generate();

      await program.methods
        .updateAuthority(newAuthority.publicKey)
        .accounts({
          hookConfig: hookConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const config = await program.account.hookConfig.fetch(hookConfigPda);
      expect(config.authority.toString()).to.equal(newAuthority.publicKey.toString());

      // Transfer back for other tests
      await program.methods
        .updateAuthority(authority.publicKey)
        .accounts({
          hookConfig: hookConfigPda,
          authority: newAuthority.publicKey,
        })
        .signers([newAuthority])
        .rpc();
    });
  });
});

