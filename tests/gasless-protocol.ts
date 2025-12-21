import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GaslessProtocol } from "../target/types/gasless_protocol";
import { 
  Keypair, 
  PublicKey, 
  SystemProgram, 
  LAMPORTS_PER_SOL 
} from "@solana/web3.js";
import {
  TOKEN_2022_PROGRAM_ID,
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { expect } from "chai";

describe("gasless-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GaslessProtocol as Program<GaslessProtocol>;
  
  // Test accounts
  let vcoinMint: PublicKey;
  let configPda: PublicKey;
  let feeVaultPda: PublicKey;
  let feePayerKeypair: Keypair;
  let userKeypair: Keypair;
  let userTokenAccount: PublicKey;
  let sessionKeypair: Keypair;
  
  // PDAs
  let sessionKeyPda: PublicKey;
  let userStatsPda: PublicKey;
  
  // Seeds
  const GASLESS_CONFIG_SEED = "gasless-config";
  const FEE_VAULT_SEED = "fee-vault";
  const SESSION_KEY_SEED = "session-key";
  const USER_GASLESS_SEED = "user-gasless";
  
  // Action scopes
  const SCOPE_TIP = 1 << 0;
  const SCOPE_VOUCH = 1 << 1;
  const SCOPE_CONTENT = 1 << 2;
  const SCOPE_GOVERNANCE = 1 << 3;
  const SCOPE_ALL = 0xFFFF;
  
  // Fee methods
  const FEE_METHOD_SUBSIDIZED = 0;
  const FEE_METHOD_VCOIN = 1;
  const FEE_METHOD_SSCRE = 2;
  
  before(async () => {
    // Generate keypairs
    feePayerKeypair = Keypair.generate();
    userKeypair = Keypair.generate();
    sessionKeypair = Keypair.generate();
    
    // Airdrop SOL
    const airdropAmount = 10 * LAMPORTS_PER_SOL;
    await provider.connection.requestAirdrop(feePayerKeypair.publicKey, airdropAmount);
    await provider.connection.requestAirdrop(userKeypair.publicKey, airdropAmount);
    
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Create VCoin mint
    vcoinMint = await createMint(
      provider.connection,
      (provider.wallet as any).payer,
      provider.wallet.publicKey,
      null,
      9,
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    
    console.log("VCoin Mint:", vcoinMint.toBase58());
    
    // Derive PDAs
    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(GASLESS_CONFIG_SEED)],
      program.programId
    );
    
    [feeVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(FEE_VAULT_SEED)],
      program.programId
    );
    
    [userStatsPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_GASLESS_SEED), userKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    [sessionKeyPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from(SESSION_KEY_SEED),
        userKeypair.publicKey.toBuffer(),
        sessionKeypair.publicKey.toBuffer()
      ],
      program.programId
    );
    
    console.log("Config PDA:", configPda.toBase58());
    console.log("Fee Vault PDA:", feeVaultPda.toBase58());
    
    // Create user token account
    const userAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      userKeypair.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    userTokenAccount = userAta.address;
    
    // Mint VCoin to user
    await mintTo(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      userTokenAccount,
      provider.wallet.publicKey,
      10_000_000_000_000, // 10,000 VCoin
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    
    console.log("User Token Account:", userTokenAccount.toBase58());
    console.log("✓ Test accounts and tokens created");
  });

  describe("Initialization", () => {
    it("Should initialize gasless configuration", async () => {
      const dailyBudget = new anchor.BN(10 * LAMPORTS_PER_SOL); // 10 SOL
      
      const tx = await program.methods
        .initialize(feePayerKeypair.publicKey, dailyBudget)
        .accounts({
          config: configPda,
          vcoinMint: vcoinMint,
          feeVault: feeVaultPda,
          authority: provider.wallet.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Initialize tx:", tx);
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.authority.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
      expect(config.feePayer.toBase58()).to.equal(feePayerKeypair.publicKey.toBase58());
      expect(config.vcoinMint.toBase58()).to.equal(vcoinMint.toBase58());
      expect(config.dailySubsidyBudget.toString()).to.equal(dailyBudget.toString());
      expect(config.paused).to.be.false;
      expect(config.solFeePerTx.toNumber()).to.equal(5000);
      expect(config.vcoinFeeMultiplier.toNumber()).to.equal(100);
      
      console.log("✓ Gasless configuration initialized");
      console.log("  Daily budget:", dailyBudget.toString(), "lamports");
      console.log("  SOL fee per tx:", config.solFeePerTx.toString());
    });
  });

  describe("Session Key Management", () => {
    it("Should create a session key with full scope", async () => {
      const duration = new anchor.BN(24 * 60 * 60); // 24 hours
      const maxActions = 1000;
      const maxSpend = new anchor.BN(100_000_000_000_000); // 100,000 VCoin
      
      const tx = await program.methods
        .createSessionKey(
          sessionKeypair.publicKey,
          SCOPE_ALL, // Full scope
          duration,
          maxActions,
          maxSpend,
          FEE_METHOD_SUBSIDIZED
        )
        .accounts({
          config: configPda,
          sessionKey: sessionKeyPda,
          userStats: userStatsPda,
          user: userKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([userKeypair])
        .rpc();
      
      console.log("Create session key tx:", tx);
      
      const session = await program.account.sessionKey.fetch(sessionKeyPda);
      expect(session.user.toBase58()).to.equal(userKeypair.publicKey.toBase58());
      expect(session.sessionPubkey.toBase58()).to.equal(sessionKeypair.publicKey.toBase58());
      expect(session.scope).to.equal(SCOPE_ALL);
      expect(session.actionsUsed).to.equal(0);
      expect(session.maxActions).to.equal(maxActions);
      expect(session.isRevoked).to.be.false;
      
      console.log("✓ Session key created");
      console.log("  Session:", sessionKeypair.publicKey.toBase58());
      console.log("  Scope: 0x" + SCOPE_ALL.toString(16));
      console.log("  Max actions:", maxActions);
    });

    it("Should create a limited scope session key", async () => {
      const limitedSessionKeypair = Keypair.generate();
      
      const [limitedSessionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(SESSION_KEY_SEED),
          userKeypair.publicKey.toBuffer(),
          limitedSessionKeypair.publicKey.toBuffer()
        ],
        program.programId
      );
      
      const limitedScope = SCOPE_TIP | SCOPE_VOUCH; // Only tip and vouch
      
      const tx = await program.methods
        .createSessionKey(
          limitedSessionKeypair.publicKey,
          limitedScope,
          new anchor.BN(3600), // 1 hour
          100, // 100 actions max
          new anchor.BN(10_000_000_000_000), // 10,000 VCoin max
          FEE_METHOD_VCOIN
        )
        .accounts({
          config: configPda,
          sessionKey: limitedSessionPda,
          userStats: userStatsPda,
          user: userKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([userKeypair])
        .rpc();
      
      console.log("Create limited session tx:", tx);
      
      const session = await program.account.sessionKey.fetch(limitedSessionPda);
      expect(session.scope).to.equal(limitedScope);
      expect(session.maxActions).to.equal(100);
      
      console.log("✓ Limited scope session key created");
      console.log("  Scope: Tip + Vouch only");
    });
  });

  describe("Session Action Execution", () => {
    it("Should execute a session action (tip)", async () => {
      const spendAmount = new anchor.BN(1_000_000_000); // 1 VCoin
      
      const tx = await program.methods
        .executeSessionAction(SCOPE_TIP, spendAmount)
        .accounts({
          config: configPda,
          sessionKey: sessionKeyPda,
          userStats: userStatsPda,
          user: userKeypair.publicKey,
        })
        .signers([userKeypair])
        .rpc();
      
      console.log("Execute session action tx:", tx);
      
      const session = await program.account.sessionKey.fetch(sessionKeyPda);
      expect(session.actionsUsed).to.equal(1);
      expect(session.vcoinSpent.toNumber()).to.equal(spendAmount.toNumber());
      
      console.log("✓ Session action executed (tip)");
      console.log("  Actions used: 1");
      console.log("  VCoin spent:", spendAmount.toString());
    });

    it("Should execute multiple session actions", async () => {
      // Execute 5 more actions
      for (let i = 0; i < 5; i++) {
        await program.methods
          .executeSessionAction(SCOPE_VOUCH, new anchor.BN(0))
          .accounts({
            config: configPda,
            sessionKey: sessionKeyPda,
            userStats: userStatsPda,
            user: userKeypair.publicKey,
          })
          .signers([userKeypair])
          .rpc();
      }
      
      const session = await program.account.sessionKey.fetch(sessionKeyPda);
      expect(session.actionsUsed).to.equal(6);
      
      console.log("✓ Multiple session actions executed");
      console.log("  Total actions: 6");
    });
  });

  describe("VCoin Fee Deduction", () => {
    it("Should deduct VCoin fee", async () => {
      const feeAmount = new anchor.BN(500_000_000); // 0.5 VCoin
      
      const tx = await program.methods
        .deductVcoinFee(feeAmount)
        .accounts({
          config: configPda,
          userStats: userStatsPda,
          vcoinMint: vcoinMint,
          userTokenAccount: userTokenAccount,
          feeVault: feeVaultPda,
          user: userKeypair.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([userKeypair])
        .rpc();
      
      console.log("Deduct VCoin fee tx:", tx);
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.totalVcoinCollected.toNumber()).to.be.greaterThan(0);
      
      const userStats = await program.account.userGaslessStats.fetch(userStatsPda);
      expect(userStats.totalVcoinFees.toNumber()).to.be.greaterThan(0);
      
      console.log("✓ VCoin fee deducted");
      console.log("  Fee amount:", feeAmount.toString());
    });
  });

  describe("Session Key Revocation", () => {
    it("Should revoke a session key", async () => {
      const tx = await program.methods
        .revokeSessionKey()
        .accounts({
          sessionKey: sessionKeyPda,
          userStats: userStatsPda,
          user: userKeypair.publicKey,
        })
        .signers([userKeypair])
        .rpc();
      
      console.log("Revoke session key tx:", tx);
      
      const session = await program.account.sessionKey.fetch(sessionKeyPda);
      expect(session.isRevoked).to.be.true;
      
      console.log("✓ Session key revoked");
      console.log("  Total actions before revoke:", session.actionsUsed);
    });

    it("Should fail to execute action with revoked session", async () => {
      try {
        await program.methods
          .executeSessionAction(SCOPE_TIP, new anchor.BN(0))
          .accounts({
            config: configPda,
            sessionKey: sessionKeyPda,
            userStats: userStatsPda,
            user: userKeypair.publicKey,
          })
          .signers([userKeypair])
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        expect(error.toString()).to.include("SessionExpired");
        console.log("✓ Correctly rejected action with revoked session");
      }
    });
  });

  describe("Configuration Updates", () => {
    it("Should update fee configuration", async () => {
      const newSolFee = new anchor.BN(10_000); // 0.00001 SOL
      const newMultiplier = new anchor.BN(200);
      const newSSCREBps = 150;
      
      await program.methods
        .updateFeeConfig(newSolFee, newMultiplier, newSSCREBps)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.solFeePerTx.toNumber()).to.equal(10_000);
      expect(config.vcoinFeeMultiplier.toNumber()).to.equal(200);
      expect(config.sscreDeductionBps).to.equal(150);
      
      console.log("✓ Fee configuration updated");
    });

    it("Should update daily budget", async () => {
      const newBudget = new anchor.BN(20 * LAMPORTS_PER_SOL); // 20 SOL
      const newMaxPerUser = 100;
      
      await program.methods
        .updateDailyBudget(newBudget, newMaxPerUser)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.dailySubsidyBudget.toString()).to.equal(newBudget.toString());
      expect(config.maxSubsidizedPerUser).to.equal(100);
      
      console.log("✓ Daily budget updated");
    });

    it("Should update fee payer", async () => {
      const newFeePayer = Keypair.generate().publicKey;
      
      await program.methods
        .setFeePayer(newFeePayer)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.feePayer.toBase58()).to.equal(newFeePayer.toBase58());
      
      // Reset to original
      await program.methods
        .setFeePayer(feePayerKeypair.publicKey)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      console.log("✓ Fee payer updated");
    });
  });

  describe("Pause/Unpause", () => {
    it("Should pause the protocol", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.paused).to.be.true;
      
      console.log("✓ Protocol paused");
    });

    it("Should unpause the protocol", async () => {
      await program.methods
        .setPaused(false)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.gaslessConfig.fetch(configPda);
      expect(config.paused).to.be.false;
      
      console.log("✓ Protocol unpaused");
    });
  });

  describe("Statistics", () => {
    it("Should fetch user gasless stats", async () => {
      const stats = await program.account.userGaslessStats.fetch(userStatsPda);
      
      console.log("✓ User gasless stats retrieved");
      console.log("  User:", stats.user.toBase58());
      console.log("  Total gasless tx:", stats.totalGaslessTx.toNumber());
      console.log("  Total subsidized:", stats.totalSubsidized.toNumber());
      console.log("  Total VCoin fees:", stats.totalVcoinFees.toNumber());
      console.log("  Sessions created:", stats.sessionsCreated);
    });

    it("Should fetch config stats", async () => {
      const config = await program.account.gaslessConfig.fetch(configPda);
      
      console.log("✓ Config stats retrieved");
      console.log("  Total subsidized tx:", config.totalSubsidizedTx.toNumber());
      console.log("  Total SOL spent:", config.totalSolSpent.toNumber());
      console.log("  Total VCoin collected:", config.totalVcoinCollected.toNumber());
      console.log("  Daily budget remaining:", 
        (config.dailySubsidyBudget.toNumber() - config.daySpent.toNumber()), "lamports");
    });
  });
});

// Helper functions for session scope
function createScope(actions: string[]): number {
  let scope = 0;
  const scopeMap: { [key: string]: number } = {
    tip: 1 << 0,
    vouch: 1 << 1,
    content: 1 << 2,
    governance: 1 << 3,
    transfer: 1 << 4,
    stake: 1 << 5,
    claim: 1 << 6,
    follow: 1 << 7,
  };
  
  for (const action of actions) {
    if (scopeMap[action.toLowerCase()]) {
      scope |= scopeMap[action.toLowerCase()];
    }
  }
  return scope;
}

function getScopeActions(scope: number): string[] {
  const actions: string[] = [];
  const scopeNames = ["tip", "vouch", "content", "governance", "transfer", "stake", "claim", "follow"];
  
  for (let i = 0; i < 8; i++) {
    if (scope & (1 << i)) {
      actions.push(scopeNames[i]);
    }
  }
  return actions;
}

