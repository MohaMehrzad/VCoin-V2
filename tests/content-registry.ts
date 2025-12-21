import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ContentRegistry } from "../target/types/content_registry";
import { expect } from "chai";
import { createHash } from "crypto";

describe("content-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ContentRegistry as Program<ContentRegistry>;
  const authority = provider.wallet;

  // PDAs
  let registryConfigPda: anchor.web3.PublicKey;
  let energyConfigPda: anchor.web3.PublicKey;
  let userEnergyPda: anchor.web3.PublicKey;
  let contentRecordPda: anchor.web3.PublicKey;
  let rateLimitPda: anchor.web3.PublicKey;

  // Test data
  const identityProgram = anchor.web3.Keypair.generate().publicKey;
  const stakingProgram = anchor.web3.Keypair.generate().publicKey;
  
  const trackingId = createHash("sha256")
    .update("test-content-" + Date.now())
    .digest();

  before(async () => {
    // Derive PDAs
    [registryConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("registry-config")],
      program.programId
    );

    [energyConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("energy-config")],
      program.programId
    );

    [userEnergyPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user-energy"), authority.publicKey.toBuffer()],
      program.programId
    );

    [contentRecordPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("content-record"), trackingId],
      program.programId
    );

    [rateLimitPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("rate-limit"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("Initialization", () => {
    it("should initialize content registry", async () => {
      try {
        await program.methods
          .initialize(identityProgram, stakingProgram)
          .accounts({
            registryConfig: registryConfigPda,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const config = await program.account.registryConfig.fetch(registryConfigPda);
        
        expect(config.authority.toString()).to.equal(authority.publicKey.toString());
        expect(config.paused).to.be.false;
        expect(config.totalContentCount.toNumber()).to.equal(0);
      } catch (error) {
        console.log("Init error (may already exist):", error.message);
      }
    });

    it("should initialize energy system", async () => {
      try {
        await program.methods
          .initializeEnergy()
          .accounts({
            energyConfig: energyConfigPda,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const config = await program.account.energyConfig.fetch(energyConfigPda);
        expect(config.paused).to.be.false;
      } catch (error) {
        console.log("Energy init error (may already exist):", error.message);
      }
    });
  });

  describe("User Energy", () => {
    it("should initialize user energy account", async () => {
      await program.methods
        .initializeUserEnergy(2) // Silver tier
        .accounts({
          userEnergy: userEnergyPda,
          user: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const energy = await program.account.userEnergy.fetch(userEnergyPda);
      
      expect(energy.user.toString()).to.equal(authority.publicKey.toString());
      expect(energy.tier).to.equal(2); // Silver
      expect(energy.maxEnergy).to.equal(800); // Silver max
      expect(energy.currentEnergy).to.equal(800); // Starts full
      expect(energy.regenRate).to.equal(80); // Silver regen
    });

    it("should update user tier", async () => {
      await program.methods
        .updateUserTier(3) // Gold tier
        .accounts({
          userEnergy: userEnergyPda,
          registryConfig: registryConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const energy = await program.account.userEnergy.fetch(userEnergyPda);
      expect(energy.tier).to.equal(3);
      expect(energy.maxEnergy).to.equal(1200); // Gold max
      expect(energy.regenRate).to.equal(120); // Gold regen
    });
  });

  describe("Content Creation", () => {
    it("should create content record", async () => {
      const contentHash = createHash("sha256")
        .update("This is test content")
        .digest();
      const contentUri = "ipfs://QmTest123456789";

      await program.methods
        .createContent(
          Array.from(trackingId),
          Array.from(contentHash),
          contentUri,
          0 // Post type
        )
        .accounts({
          registryConfig: registryConfigPda,
          contentRecord: contentRecordPda,
          userEnergy: userEnergyPda,
          rateLimit: rateLimitPda,
          author: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const content = await program.account.contentRecord.fetch(contentRecordPda);
      
      expect(content.author.toString()).to.equal(authority.publicKey.toString());
      expect(content.contentType).to.equal(0); // Post
      expect(content.state).to.equal(0); // Active
      expect(content.version).to.equal(1);
      expect(content.energySpent).to.equal(10); // Text post cost

      // Check energy was spent
      const energy = await program.account.userEnergy.fetch(userEnergyPda);
      expect(energy.currentEnergy).to.be.lessThan(1200);
    });

    it("should track content count", async () => {
      const config = await program.account.registryConfig.fetch(registryConfigPda);
      expect(config.totalContentCount.toNumber()).to.be.greaterThan(0);
      expect(config.activeContentCount.toNumber()).to.be.greaterThan(0);
    });
  });

  describe("Content Editing", () => {
    it("should edit content (free within 1 hour)", async () => {
      const newContentHash = createHash("sha256")
        .update("Updated content")
        .digest();
      const newUri = "ipfs://QmUpdated123";

      await program.methods
        .editContent(Array.from(newContentHash), newUri)
        .accounts({
          contentRecord: contentRecordPda,
          userEnergy: userEnergyPda,
          author: authority.publicKey,
        })
        .rpc();

      const content = await program.account.contentRecord.fetch(contentRecordPda);
      
      expect(content.version).to.equal(2);
      expect(content.state).to.equal(1); // Edited
      expect(content.previousHash).to.not.deep.equal(new Array(32).fill(0));
    });
  });

  describe("Content Deletion", () => {
    it("should soft delete content", async () => {
      // Create another piece of content to delete
      const deleteTrackingId = createHash("sha256")
        .update("delete-content-" + Date.now())
        .digest();

      const [deleteContentPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("content-record"), deleteTrackingId],
        program.programId
      );

      const contentHash = createHash("sha256").update("delete me").digest();

      await program.methods
        .createContent(
          Array.from(deleteTrackingId),
          Array.from(contentHash),
          "ipfs://delete",
          0
        )
        .accounts({
          registryConfig: registryConfigPda,
          contentRecord: deleteContentPda,
          userEnergy: userEnergyPda,
          rateLimit: rateLimitPda,
          author: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await program.methods
        .deleteContent()
        .accounts({
          registryConfig: registryConfigPda,
          contentRecord: deleteContentPda,
          author: authority.publicKey,
        })
        .rpc();

      const content = await program.account.contentRecord.fetch(deleteContentPda);
      expect(content.state).to.equal(2); // Deleted
    });

    it("should not allow editing deleted content", async () => {
      const deleteTrackingId = createHash("sha256")
        .update("already-deleted-" + Date.now())
        .digest();

      const [deleteContentPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("content-record"), deleteTrackingId],
        program.programId
      );

      const contentHash = createHash("sha256").update("test").digest();

      // Create and delete
      await program.methods
        .createContent(
          Array.from(deleteTrackingId),
          Array.from(contentHash),
          "ipfs://test",
          0
        )
        .accounts({
          registryConfig: registryConfigPda,
          contentRecord: deleteContentPda,
          userEnergy: userEnergyPda,
          rateLimit: rateLimitPda,
          author: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      await program.methods
        .deleteContent()
        .accounts({
          registryConfig: registryConfigPda,
          contentRecord: deleteContentPda,
          author: authority.publicKey,
        })
        .rpc();

      // Try to edit deleted content
      try {
        await program.methods
          .editContent(Array.from(contentHash), "ipfs://new")
          .accounts({
            contentRecord: deleteContentPda,
            userEnergy: userEnergyPda,
            author: authority.publicKey,
          })
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        expect(error.message).to.include("CannotEditDeleted");
      }
    });
  });

  describe("Engagement and Refunds", () => {
    it("should update engagement count", async () => {
      await program.methods
        .updateEngagement(150) // 150 likes
        .accounts({
          contentRecord: contentRecordPda,
          registryConfig: registryConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const content = await program.account.contentRecord.fetch(contentRecordPda);
      expect(content.engagementCount).to.equal(150);
    });
  });

  describe("Protocol Pause", () => {
    it("should pause and unpause registry", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          registryConfig: registryConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      let config = await program.account.registryConfig.fetch(registryConfigPda);
      expect(config.paused).to.be.true;

      await program.methods
        .setPaused(false)
        .accounts({
          registryConfig: registryConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      config = await program.account.registryConfig.fetch(registryConfigPda);
      expect(config.paused).to.be.false;
    });
  });
});

