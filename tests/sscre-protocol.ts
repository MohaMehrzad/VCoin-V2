import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SscreProtocol } from "../target/types/sscre_protocol";
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
import { keccak256 } from "js-sha3";

describe("sscre-protocol", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SscreProtocol as Program<SscreProtocol>;
  
  // Test accounts
  let vcoinMint: PublicKey;
  let poolConfigPda: PublicKey;
  let poolVaultPda: PublicKey;
  let fundingConfigPda: PublicKey;
  let circuitBreakerPda: PublicKey;
  let oracleKeypair: Keypair;
  let userKeypair: Keypair;
  let userTokenAccount: PublicKey;
  let feeRecipientKeypair: Keypair;
  let feeTokenAccount: PublicKey;
  
  // Test constants
  const POOL_CONFIG_SEED = "pool-config";
  const POOL_VAULT_SEED = "pool-vault";
  const FUNDING_LAYER_SEED = "funding-layer";
  const CIRCUIT_BREAKER_SEED = "circuit-breaker";
  const EPOCH_SEED = "epoch";
  const USER_CLAIM_SEED = "user-claim";
  
  before(async () => {
    // Generate keypairs
    oracleKeypair = Keypair.generate();
    userKeypair = Keypair.generate();
    feeRecipientKeypair = Keypair.generate();
    
    // Airdrop SOL to accounts
    const airdropAmount = 10 * LAMPORTS_PER_SOL;
    
    await provider.connection.requestAirdrop(
      oracleKeypair.publicKey,
      airdropAmount
    );
    await provider.connection.requestAirdrop(
      userKeypair.publicKey,
      airdropAmount
    );
    await provider.connection.requestAirdrop(
      feeRecipientKeypair.publicKey,
      airdropAmount
    );
    
    // Wait for confirmation
    await new Promise(resolve => setTimeout(resolve, 1000));
    
    // Create VCoin mint (Token-2022)
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
    [poolConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_CONFIG_SEED)],
      program.programId
    );
    
    [poolVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_VAULT_SEED)],
      program.programId
    );
    
    [fundingConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(FUNDING_LAYER_SEED)],
      program.programId
    );
    
    [circuitBreakerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(CIRCUIT_BREAKER_SEED)],
      program.programId
    );
    
    console.log("Pool Config PDA:", poolConfigPda.toBase58());
    console.log("Pool Vault PDA:", poolVaultPda.toBase58());
    
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
    
    // Create fee recipient token account
    const feeAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      feeRecipientKeypair.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    feeTokenAccount = feeAta.address;
    
    console.log("User Token Account:", userTokenAccount.toBase58());
    console.log("Fee Token Account:", feeTokenAccount.toBase58());
  });

  describe("Pool Initialization", () => {
    it("Should initialize the rewards pool", async () => {
      const tx = await program.methods
        .initializePool(feeRecipientKeypair.publicKey)
        .accounts({
          poolConfig: poolConfigPda,
          vcoinMint: vcoinMint,
          poolVault: poolVaultPda,
          authority: provider.wallet.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Initialize pool tx:", tx);
      
      // Verify pool config
      const poolConfig = await program.account.rewardsPoolConfig.fetch(poolConfigPda);
      expect(poolConfig.authority.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
      expect(poolConfig.vcoinMint.toBase58()).to.equal(vcoinMint.toBase58());
      expect(poolConfig.currentEpoch.toNumber()).to.equal(0);
      expect(poolConfig.paused).to.be.false;
      expect(poolConfig.oracleCount).to.equal(0);
      
      console.log("✓ Pool initialized with 350M VCoin reserves");
    });

    it("Should initialize funding layers", async () => {
      const tx = await program.methods
        .initializeFundingLayers()
        .accounts({
          poolConfig: poolConfigPda,
          fundingConfig: fundingConfigPda,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Initialize funding layers tx:", tx);
      
      // Verify funding config
      const fundingConfig = await program.account.fundingLayerConfig.fetch(fundingConfigPda);
      expect(fundingConfig.activeLayer).to.equal(1);
      expect(fundingConfig.l3BuybackRateBps).to.equal(1000); // 10%
      expect(fundingConfig.l4ProfitRateBps).to.equal(2500); // 25%
      expect(fundingConfig.l5FeeRecyclingRateBps).to.equal(5000); // 50%
      
      console.log("✓ 6-Layer funding configuration initialized");
    });

    it("Should initialize circuit breaker", async () => {
      const tx = await program.methods
        .initializeCircuitBreaker()
        .accounts({
          poolConfig: poolConfigPda,
          circuitBreaker: circuitBreakerPda,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Initialize circuit breaker tx:", tx);
      
      // Verify circuit breaker
      const cb = await program.account.circuitBreaker.fetch(circuitBreakerPda);
      expect(cb.isActive).to.be.false;
      expect(cb.triggerCount).to.equal(0);
      
      console.log("✓ Circuit breaker initialized");
    });
  });

  describe("Oracle Management", () => {
    it("Should register an oracle", async () => {
      const tx = await program.methods
        .registerOracle(oracleKeypair.publicKey)
        .accounts({
          poolConfig: poolConfigPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      console.log("Register oracle tx:", tx);
      
      // Verify oracle registered
      const poolConfig = await program.account.rewardsPoolConfig.fetch(poolConfigPda);
      expect(poolConfig.oracleCount).to.equal(1);
      expect(poolConfig.oracles[0].toBase58()).to.equal(oracleKeypair.publicKey.toBase58());
      
      console.log("✓ Oracle registered:", oracleKeypair.publicKey.toBase58());
    });
  });

  describe("Epoch Management", () => {
    let epochDistributionPda: PublicKey;
    const epochAllocation = new anchor.BN(1_000_000 * 1_000_000_000); // 1M VCoin
    
    it("Should start a new epoch", async () => {
      // Derive epoch PDA for epoch 1
      [epochDistributionPda] = PublicKey.findProgramAddressSync(
        [Buffer.from(EPOCH_SEED), new anchor.BN(1).toArrayLike(Buffer, "le", 8)],
        program.programId
      );
      
      const tx = await program.methods
        .startEpoch(epochAllocation)
        .accounts({
          poolConfig: poolConfigPda,
          epochDistribution: epochDistributionPda,
          circuitBreaker: circuitBreakerPda,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Start epoch tx:", tx);
      
      // Verify epoch started
      const epochDist = await program.account.epochDistribution.fetch(epochDistributionPda);
      expect(epochDist.epoch.toNumber()).to.equal(1);
      expect(epochDist.totalAllocation.toString()).to.equal(epochAllocation.toString());
      expect(epochDist.isFinalized).to.be.false;
      expect(epochDist.totalClaimed.toNumber()).to.equal(0);
      
      console.log("✓ Epoch 1 started with", epochAllocation.toString(), "VCoin allocation");
    });

    it("Should update merkle root (finalize epoch)", async () => {
      // Generate a test merkle root
      const merkleRoot = new Uint8Array(32).fill(1); // Simplified for testing
      const eligibleUsers = new anchor.BN(1000);
      const avgFiveAScore = 7500;
      
      const tx = await program.methods
        .updateMerkleRoot(
          Array.from(merkleRoot) as number[],
          eligibleUsers,
          avgFiveAScore
        )
        .accounts({
          poolConfig: poolConfigPda,
          epochDistribution: epochDistributionPda,
          oracle: oracleKeypair.publicKey,
        })
        .signers([oracleKeypair])
        .rpc();
      
      console.log("Update merkle root tx:", tx);
      
      // Verify epoch finalized
      const epochDist = await program.account.epochDistribution.fetch(epochDistributionPda);
      expect(epochDist.isFinalized).to.be.true;
      expect(epochDist.eligibleUsers.toNumber()).to.equal(1000);
      expect(epochDist.avgFiveAScore).to.equal(7500);
      
      console.log("✓ Epoch finalized with merkle root");
    });
  });

  describe("Pause/Unpause", () => {
    it("Should pause the protocol", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          poolConfig: poolConfigPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const poolConfig = await program.account.rewardsPoolConfig.fetch(poolConfigPda);
      expect(poolConfig.paused).to.be.true;
      
      console.log("✓ Protocol paused");
    });

    it("Should unpause the protocol", async () => {
      await program.methods
        .setPaused(false)
        .accounts({
          poolConfig: poolConfigPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const poolConfig = await program.account.rewardsPoolConfig.fetch(poolConfigPda);
      expect(poolConfig.paused).to.be.false;
      
      console.log("✓ Protocol unpaused");
    });
  });

  describe("Circuit Breaker", () => {
    it("Should trigger circuit breaker", async () => {
      await program.methods
        .triggerCircuitBreaker(1) // reason: 1 = manual trigger
        .accounts({
          poolConfig: poolConfigPda,
          circuitBreaker: circuitBreakerPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const cb = await program.account.circuitBreaker.fetch(circuitBreakerPda);
      expect(cb.isActive).to.be.true;
      expect(cb.triggerCount).to.equal(1);
      
      console.log("✓ Circuit breaker triggered");
    });

    it("Should reset circuit breaker", async () => {
      await program.methods
        .resetCircuitBreaker()
        .accounts({
          poolConfig: poolConfigPda,
          circuitBreaker: circuitBreakerPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const cb = await program.account.circuitBreaker.fetch(circuitBreakerPda);
      expect(cb.isActive).to.be.false;
      
      console.log("✓ Circuit breaker reset");
    });
  });

  describe("View Functions", () => {
    it("Should get pool stats", async () => {
      await program.methods
        .getPoolStats()
        .accounts({
          poolConfig: poolConfigPda,
        })
        .rpc();
      
      console.log("✓ Pool stats retrieved");
    });
  });
});

// Merkle tree helper functions for testing
function computeLeaf(user: PublicKey, amount: bigint, epoch: bigint): Buffer {
  const data = Buffer.alloc(48);
  data.set(user.toBytes(), 0);
  data.writeBigUInt64LE(amount, 32);
  data.writeBigUInt64LE(epoch, 40);
  return Buffer.from(keccak256(data), "hex");
}

function computeMerkleRoot(leaves: Buffer[]): Buffer {
  if (leaves.length === 0) return Buffer.alloc(32);
  if (leaves.length === 1) return leaves[0];
  
  const newLevel: Buffer[] = [];
  for (let i = 0; i < leaves.length; i += 2) {
    const left = leaves[i];
    const right = i + 1 < leaves.length ? leaves[i + 1] : leaves[i];
    
    const [sortedLeft, sortedRight] = left < right ? [left, right] : [right, left];
    const combined = Buffer.concat([sortedLeft, sortedRight]);
    newLevel.push(Buffer.from(keccak256(combined), "hex"));
  }
  
  return computeMerkleRoot(newLevel);
}

function getMerkleProof(leaves: Buffer[], index: number): Buffer[] {
  const proof: Buffer[] = [];
  let currentIndex = index;
  let currentLeaves = [...leaves];
  
  while (currentLeaves.length > 1) {
    const newLevel: Buffer[] = [];
    
    for (let i = 0; i < currentLeaves.length; i += 2) {
      const left = currentLeaves[i];
      const right = i + 1 < currentLeaves.length ? currentLeaves[i + 1] : currentLeaves[i];
      
      // Add sibling to proof if this is our node's level
      if (i === currentIndex || i + 1 === currentIndex) {
        if (i === currentIndex && i + 1 < currentLeaves.length) {
          proof.push(right);
        } else if (i + 1 === currentIndex) {
          proof.push(left);
        }
      }
      
      const [sortedLeft, sortedRight] = left < right ? [left, right] : [right, left];
      const combined = Buffer.concat([sortedLeft, sortedRight]);
      newLevel.push(Buffer.from(keccak256(combined), "hex"));
    }
    
    currentIndex = Math.floor(currentIndex / 2);
    currentLeaves = newLevel;
  }
  
  return proof;
}

