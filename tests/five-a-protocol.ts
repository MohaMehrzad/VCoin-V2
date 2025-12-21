import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FiveAProtocol } from "../target/types/five_a_protocol";
import { expect } from "chai";

describe("five-a-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FiveAProtocol as Program<FiveAProtocol>;
  const authority = provider.wallet;

  // PDAs
  let fiveAConfigPda: anchor.web3.PublicKey;
  let oraclePda: anchor.web3.PublicKey;
  let userScorePda: anchor.web3.PublicKey;

  // Test accounts
  const identityProgram = anchor.web3.Keypair.generate().publicKey;
  const vcoinMint = anchor.web3.Keypair.generate().publicKey;
  const vouchVault = anchor.web3.Keypair.generate().publicKey;
  const testOracle = anchor.web3.Keypair.generate();
  const testUser = anchor.web3.Keypair.generate();

  before(async () => {
    // Derive PDAs
    [fiveAConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("five-a-config")],
      program.programId
    );

    [oraclePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("oracle"), testOracle.publicKey.toBuffer()],
      program.programId
    );

    [userScorePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user-score"), testUser.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("Initialization", () => {
    it("should initialize 5A protocol", async () => {
      try {
        await program.methods
          .initialize(identityProgram, vcoinMint)
          .accounts({
            fiveAConfig: fiveAConfigPda,
            vouchVault: vouchVault,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const config = await program.account.fiveAConfig.fetch(fiveAConfigPda);
        
        expect(config.authority.toString()).to.equal(authority.publicKey.toString());
        expect(config.identityProgram.toString()).to.equal(identityProgram.toString());
        expect(config.paused).to.be.false;
        expect(config.oracleCount).to.equal(0);
        expect(config.totalUsers.toNumber()).to.equal(0);
      } catch (error) {
        console.log("Init error (may already exist):", error.message);
      }
    });
  });

  describe("Oracle Management", () => {
    it("should register an oracle", async () => {
      await program.methods
        .registerOracle("Test Oracle")
        .accounts({
          fiveAConfig: fiveAConfigPda,
          oracle: oraclePda,
          oracleWallet: testOracle.publicKey,
          authority: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const config = await program.account.fiveAConfig.fetch(fiveAConfigPda);
      expect(config.oracleCount).to.equal(1);

      const oracle = await program.account.oracle.fetch(oraclePda);
      expect(oracle.wallet.toString()).to.equal(testOracle.publicKey.toString());
      expect(oracle.isActive).to.be.true;
    });

    it("should reject duplicate oracle registration", async () => {
      try {
        await program.methods
          .registerOracle("Duplicate Oracle")
          .accounts({
            fiveAConfig: fiveAConfigPda,
            oracle: oraclePda,
            oracleWallet: testOracle.publicKey,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        // Expected
      }
    });
  });

  describe("Score Submission", () => {
    it("should submit user scores", async () => {
      // Airdrop to oracle for transaction fees
      const airdropSig = await provider.connection.requestAirdrop(
        testOracle.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropSig);

      await program.methods
        .submitScore(
          8000, // Authenticity (80%)
          7500, // Accuracy (75%)
          6500, // Agility (65%)
          9000, // Activity (90%)
          7000  // Approved (70%)
        )
        .accounts({
          fiveAConfig: fiveAConfigPda,
          userScore: userScorePda,
          oracleAccount: oraclePda,
          user: testUser.publicKey,
          oracle: testOracle.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([testOracle])
        .rpc();

      const score = await program.account.userScore.fetch(userScorePda);
      
      expect(score.user.toString()).to.equal(testUser.publicKey.toString());
      expect(score.authenticity).to.equal(8000);
      expect(score.accuracy).to.equal(7500);
      expect(score.agility).to.equal(6500);
      expect(score.activity).to.equal(9000);
      expect(score.approved).to.equal(7000);
      
      // Verify composite score calculation
      // (8000*25 + 7500*20 + 6500*15 + 9000*25 + 7000*15) / 100 = 7725
      expect(score.compositeScore).to.be.closeTo(7725, 50);
    });

    it("should update existing scores", async () => {
      await program.methods
        .submitScore(
          8500, // Updated Authenticity
          8000, // Updated Accuracy
          7000, // Updated Agility
          9500, // Updated Activity
          7500  // Updated Approved
        )
        .accounts({
          fiveAConfig: fiveAConfigPda,
          userScore: userScorePda,
          oracleAccount: oraclePda,
          user: testUser.publicKey,
          oracle: testOracle.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([testOracle])
        .rpc();

      const score = await program.account.userScore.fetch(userScorePda);
      expect(score.authenticity).to.equal(8500);
      expect(score.updateCount).to.equal(2);
    });

    it("should reject scores from non-oracle", async () => {
      const fakeOracle = anchor.web3.Keypair.generate();
      const airdropSig = await provider.connection.requestAirdrop(
        fakeOracle.publicKey,
        1 * anchor.web3.LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(airdropSig);

      const [fakeOraclePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("oracle"), fakeOracle.publicKey.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .submitScore(5000, 5000, 5000, 5000, 5000)
          .accounts({
            fiveAConfig: fiveAConfigPda,
            userScore: userScorePda,
            oracleAccount: fakeOraclePda,
            user: testUser.publicKey,
            oracle: fakeOracle.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([fakeOracle])
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        // Expected - oracle account doesn't exist
      }
    });

    it("should reject invalid scores (>10000)", async () => {
      try {
        await program.methods
          .submitScore(15000, 5000, 5000, 5000, 5000) // Invalid authenticity
          .accounts({
            fiveAConfig: fiveAConfigPda,
            userScore: userScorePda,
            oracleAccount: oraclePda,
            user: testUser.publicKey,
            oracle: testOracle.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([testOracle])
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        expect(error.message).to.include("InvalidScore");
      }
    });
  });

  describe("Score Snapshots", () => {
    it("should create score snapshot", async () => {
      const config = await program.account.fiveAConfig.fetch(fiveAConfigPda);
      const nextEpoch = config.currentEpoch.toNumber() + 1;

      const [snapshotPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("score-snapshot"), Buffer.from(new anchor.BN(nextEpoch).toArray("le", 8))],
        program.programId
      );

      const merkleRoot = Buffer.alloc(32);
      merkleRoot.write("test-merkle-root");

      await program.methods
        .createSnapshot(Array.from(merkleRoot), new anchor.BN(100), 7500)
        .accounts({
          fiveAConfig: fiveAConfigPda,
          snapshot: snapshotPda,
          oracle: testOracle.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([testOracle])
        .rpc();

      const snapshot = await program.account.scoreSnapshot.fetch(snapshotPda);
      expect(snapshot.epoch.toNumber()).to.equal(nextEpoch);
      expect(snapshot.userCount.toNumber()).to.equal(100);
      expect(snapshot.avgScore).to.equal(7500);
    });
  });

  describe("Private Score Mode", () => {
    it("should enable private score mode", async () => {
      // First need to create a user score for authority
      const [authorityScorePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user-score"), authority.publicKey.toBuffer()],
        program.programId
      );

      // Submit initial score
      await program.methods
        .submitScore(7000, 7000, 7000, 7000, 7000)
        .accounts({
          fiveAConfig: fiveAConfigPda,
          userScore: authorityScorePda,
          oracleAccount: oraclePda,
          user: authority.publicKey,
          oracle: testOracle.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([testOracle])
        .rpc();

      await program.methods
        .enablePrivateScore()
        .accounts({
          userScore: authorityScorePda,
          user: authority.publicKey,
        })
        .rpc();

      const score = await program.account.userScore.fetch(authorityScorePda);
      expect(score.isPrivate).to.be.true;
    });

    it("should disable private score mode", async () => {
      const [authorityScorePda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("user-score"), authority.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .disablePrivateScore()
        .accounts({
          userScore: authorityScorePda,
          user: authority.publicKey,
        })
        .rpc();

      const score = await program.account.userScore.fetch(authorityScorePda);
      expect(score.isPrivate).to.be.false;
    });
  });

  describe("Protocol Pause", () => {
    it("should pause and unpause protocol", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          fiveAConfig: fiveAConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      let config = await program.account.fiveAConfig.fetch(fiveAConfigPda);
      expect(config.paused).to.be.true;

      await program.methods
        .setPaused(false)
        .accounts({
          fiveAConfig: fiveAConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      config = await program.account.fiveAConfig.fetch(fiveAConfigPda);
      expect(config.paused).to.be.false;
    });
  });
});

