import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IdentityProtocol } from "../target/types/identity_protocol";
import { expect } from "chai";

describe("identity-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.IdentityProtocol as Program<IdentityProtocol>;
  const authority = provider.wallet;

  // PDAs
  let identityConfigPda: anchor.web3.PublicKey;
  let userIdentityPda: anchor.web3.PublicKey;
  let subscriptionPda: anchor.web3.PublicKey;

  // Test accounts
  const sasProgram = anchor.web3.Keypair.generate().publicKey;
  const usdcMint = anchor.web3.Keypair.generate().publicKey;
  const treasury = anchor.web3.Keypair.generate().publicKey;
  const testUser = anchor.web3.Keypair.generate();

  before(async () => {
    // Derive PDAs
    [identityConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity-config")],
      program.programId
    );

    [userIdentityPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("identity"), authority.publicKey.toBuffer()],
      program.programId
    );

    [subscriptionPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("subscription"), authority.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("Initialization", () => {
    it("should initialize identity protocol", async () => {
      try {
        await program.methods
          .initialize(sasProgram, usdcMint)
          .accounts({
            identityConfig: identityConfigPda,
            treasury: treasury,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const config = await program.account.identityConfig.fetch(identityConfigPda);
        
        expect(config.authority.toString()).to.equal(authority.publicKey.toString());
        expect(config.sasProgram.toString()).to.equal(sasProgram.toString());
        expect(config.usdcMint.toString()).to.equal(usdcMint.toString());
        expect(config.paused).to.be.false;
        expect(config.totalIdentities.toNumber()).to.equal(0);
      } catch (error) {
        console.log("Init error (may already exist):", error.message);
      }
    });
  });

  describe("Identity Creation", () => {
    it("should create a new identity", async () => {
      const didHash = Buffer.alloc(32);
      didHash.write("test-did-hash");
      const username = "testuser123";

      await program.methods
        .createIdentity(Array.from(didHash), username)
        .accounts({
          identityConfig: identityConfigPda,
          identity: userIdentityPda,
          owner: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const identity = await program.account.identity.fetch(userIdentityPda);
      
      expect(identity.owner.toString()).to.equal(authority.publicKey.toString());
      expect(identity.verificationLevel).to.equal(0); // None
      expect(identity.isActive).to.be.true;
    });

    it("should update DID hash", async () => {
      const newDidHash = Buffer.alloc(32);
      newDidHash.write("updated-did-hash");

      await program.methods
        .updateDidHash(Array.from(newDidHash))
        .accounts({
          identity: userIdentityPda,
          owner: authority.publicKey,
        })
        .rpc();

      const identity = await program.account.identity.fetch(userIdentityPda);
      expect(Buffer.from(identity.didHash).toString()).to.include("updated");
    });
  });

  describe("Verification Levels", () => {
    it("should update verification level (admin)", async () => {
      const verificationHash = Buffer.alloc(32);
      verificationHash.write("kyc-verification");

      await program.methods
        .updateVerification(2, Array.from(verificationHash)) // KYC level
        .accounts({
          identityConfig: identityConfigPda,
          identity: userIdentityPda,
          authority: authority.publicKey,
        })
        .rpc();

      const identity = await program.account.identity.fetch(userIdentityPda);
      expect(identity.verificationLevel).to.equal(2);
    });

    it("should not allow verification downgrade", async () => {
      const verificationHash = Buffer.alloc(32);

      try {
        await program.methods
          .updateVerification(1, Array.from(verificationHash)) // Try to downgrade to Basic
          .accounts({
            identityConfig: identityConfigPda,
            identity: userIdentityPda,
            authority: authority.publicKey,
          })
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        expect(error.message).to.include("CannotDowngradeVerification");
      }
    });
  });

  describe("Trusted Attesters", () => {
    it("should add trusted attester", async () => {
      const attester = anchor.web3.Keypair.generate().publicKey;

      await program.methods
        .addTrustedAttester(attester)
        .accounts({
          identityConfig: identityConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const config = await program.account.identityConfig.fetch(identityConfigPda);
      expect(config.attesterCount).to.equal(1);
      expect(config.trustedAttesters[0].toString()).to.equal(attester.toString());
    });

    it("should remove trusted attester", async () => {
      const config = await program.account.identityConfig.fetch(identityConfigPda);
      const attester = config.trustedAttesters[0];

      await program.methods
        .removeTrustedAttester(attester)
        .accounts({
          identityConfig: identityConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const updatedConfig = await program.account.identityConfig.fetch(identityConfigPda);
      expect(updatedConfig.attesterCount).to.equal(0);
    });
  });

  describe("Subscriptions", () => {
    it("should create subscription", async () => {
      await program.methods
        .subscribe(1) // Verified tier
        .accounts({
          identityConfig: identityConfigPda,
          subscription: subscriptionPda,
          user: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const subscription = await program.account.subscription.fetch(subscriptionPda);
      expect(subscription.tier).to.equal(1);
      expect(subscription.user.toString()).to.equal(authority.publicKey.toString());
    });
  });

  describe("Protocol Pause", () => {
    it("should pause and unpause protocol", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          identityConfig: identityConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      let config = await program.account.identityConfig.fetch(identityConfigPda);
      expect(config.paused).to.be.true;

      await program.methods
        .setPaused(false)
        .accounts({
          identityConfig: identityConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      config = await program.account.identityConfig.fetch(identityConfigPda);
      expect(config.paused).to.be.false;
    });
  });
});

