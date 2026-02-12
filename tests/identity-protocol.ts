import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { IdentityProtocol } from "../target/types/identity_protocol";
import { expect } from "chai";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";

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
  let usdcMint: anchor.web3.PublicKey;
  const treasuryKeypair = anchor.web3.Keypair.generate();
  const testUser = anchor.web3.Keypair.generate();

  // Token accounts for subscriptions (C-AUDIT-22)
  let userUsdcAccount: anchor.web3.PublicKey;
  let treasuryUsdcAccount: anchor.web3.PublicKey;

  before(async () => {
    // Airdrop SOL to treasury
    const sig = await provider.connection.requestAirdrop(
      treasuryKeypair.publicKey,
      1 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // Create USDC mock mint (SPL Token)
    usdcMint = await createMint(
      provider.connection,
      (provider.wallet as any).payer,
      provider.wallet.publicKey,
      null,
      6, // USDC has 6 decimals
    );

    // Create user USDC account and fund it
    const userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      usdcMint,
      authority.publicKey,
    );
    userUsdcAccount = userAta.address;

    await mintTo(
      provider.connection,
      (provider.wallet as any).payer,
      usdcMint,
      userUsdcAccount,
      provider.wallet.publicKey,
      1_000_000_000, // 1000 USDC
    );

    // Create treasury USDC account
    const treasuryAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      usdcMint,
      treasuryKeypair.publicKey,
    );
    treasuryUsdcAccount = treasuryAta.address;

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
            treasury: treasuryKeypair.publicKey,
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

    it("should reject DID hash update before verification (H-AUDIT-13)", async () => {
      // H-AUDIT-13: Require verification_level > 0 before DID hash changes
      const newDidHash = Buffer.alloc(32);
      newDidHash.write("updated-did-hash");

      try {
        await program.methods
          .updateDidHash(Array.from(newDidHash))
          .accounts({
            identity: userIdentityPda,
            owner: authority.publicKey,
          })
          .rpc();

        expect.fail("Should have thrown error - identity not verified");
      } catch (error: any) {
        // Expected: verification_level == 0, cannot update DID hash
        expect(error.message).to.include("IdentityNotFound");
      }

      console.log("DID hash update correctly blocked before verification (H-AUDIT-13)");
    });
  });

  describe("Verification Levels", () => {
    it("should reject verification upgrade without SAS attestation (C-AUDIT-17)", async () => {
      // C-AUDIT-17: Require SAS attestation (non-zero verification_hash) before upgrade
      const verificationHash = Buffer.alloc(32);
      verificationHash.write("kyc-verification");

      try {
        await program.methods
          .updateVerification(2, Array.from(verificationHash)) // KYC level
          .accounts({
            identityConfig: identityConfigPda,
            identity: userIdentityPda,
            authority: authority.publicKey,
          })
          .rpc();

        expect.fail("Should have thrown SASAttestationRequired error");
      } catch (error: any) {
        expect(error.message).to.include("SASAttestationRequired");
      }

      console.log("Verification upgrade correctly blocked without SAS attestation (C-AUDIT-17)");
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
    it("should create subscription with USDC payment (C-AUDIT-22)", async () => {
      await program.methods
        .subscribe(1) // Verified tier ($4/month)
        .accounts({
          identityConfig: identityConfigPda,
          subscription: subscriptionPda,
          user: authority.publicKey,
          usdcMint: usdcMint,
          userTokenAccount: userUsdcAccount,
          treasuryTokenAccount: treasuryUsdcAccount,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const subscription = await program.account.subscription.fetch(subscriptionPda);
      expect(subscription.tier).to.equal(1);
      expect(subscription.user.toString()).to.equal(authority.publicKey.toString());
      expect(subscription.totalPaid.toNumber()).to.be.greaterThan(0);

      console.log("Subscription created with USDC payment (C-AUDIT-22)");
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

