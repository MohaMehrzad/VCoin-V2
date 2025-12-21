import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { StakingProtocol } from "../target/types/staking_protocol";
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
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

describe("staking-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.StakingProtocol as Program<StakingProtocol>;
  
  // Test accounts
  const authority = Keypair.generate();
  const user1 = Keypair.generate();
  const user2 = Keypair.generate();
  
  let vcoinMint: PublicKey;
  let vevcoinMint: PublicKey;
  let poolPda: PublicKey;
  let poolVaultPda: PublicKey;
  let user1StakePda: PublicKey;
  let user2StakePda: PublicKey;
  let user1VcoinAccount: PublicKey;
  let user2VcoinAccount: PublicKey;

  // Constants
  const ONE_WEEK = 7 * 24 * 60 * 60;
  const ONE_YEAR = 365 * 24 * 60 * 60;
  const FOUR_YEARS = 4 * 365 * 24 * 60 * 60;

  before(async () => {
    // Airdrop SOL
    for (const account of [authority, user1, user2]) {
      const signature = await provider.connection.requestAirdrop(
        account.publicKey,
        10 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(signature);
    }

    // Find PDAs
    [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("staking-pool")],
      program.programId
    );

    [poolVaultPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool-vault")],
      program.programId
    );

    [user1StakePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user-stake"), user1.publicKey.toBuffer()],
      program.programId
    );

    [user2StakePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user-stake"), user2.publicKey.toBuffer()],
      program.programId
    );

    // Create VCoin mock mint
    vcoinMint = await createMint(
      provider.connection,
      authority,
      authority.publicKey,
      null,
      9,
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    // Create veVCoin mock mint
    vevcoinMint = await createMint(
      provider.connection,
      authority,
      poolPda, // Pool will be mint authority
      null,
      9,
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    // Create user VCoin accounts and fund them
    for (const user of [user1, user2]) {
      const account = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        authority,
        vcoinMint,
        user.publicKey,
        false,
        undefined,
        undefined,
        TOKEN_2022_PROGRAM_ID
      );

      // Mint 100,000 VCoin to each user
      await mintTo(
        provider.connection,
        authority,
        vcoinMint,
        account.address,
        authority,
        100_000 * 1e9,
        [],
        undefined,
        TOKEN_2022_PROGRAM_ID
      );

      if (user === user1) {
        user1VcoinAccount = account.address;
      } else {
        user2VcoinAccount = account.address;
      }
    }

    console.log("VCoin Mint:", vcoinMint.toBase58());
    console.log("veVCoin Mint:", vevcoinMint.toBase58());
    console.log("Pool PDA:", poolPda.toBase58());
  });

  it("Initializes the staking pool", async () => {
    const vevcoinProgram = Keypair.generate().publicKey; // Mock veVCoin program

    await program.methods
      .initializePool(vevcoinProgram)
      .accounts({
        authority: authority.publicKey,
        pool: poolPda,
        vcoinMint: vcoinMint,
        vevcoinMint: vevcoinMint,
        poolVault: poolVaultPda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([authority])
      .rpc();

    const pool = await program.account.stakingPool.fetch(poolPda);
    expect(pool.authority.toBase58()).to.equal(authority.publicKey.toBase58());
    expect(pool.vcoinMint.toBase58()).to.equal(vcoinMint.toBase58());
    expect(pool.vevcoinMint.toBase58()).to.equal(vevcoinMint.toBase58());
    expect(pool.totalStaked.toNumber()).to.equal(0);
    expect(pool.totalStakers.toNumber()).to.equal(0);
    expect(pool.paused).to.equal(false);

    console.log("Staking pool initialized!");
  });

  it("Stakes VCoin with 1 year lock (Silver tier)", async () => {
    const stakeAmount = new anchor.BN(10_000 * 1e9); // 10,000 VCoin (Silver tier)
    const lockDuration = new anchor.BN(ONE_YEAR);

    await program.methods
      .stake(stakeAmount, lockDuration)
      .accounts({
        user: user1.publicKey,
        pool: poolPda,
        userStake: user1StakePda,
        vcoinMint: vcoinMint,
        userVcoinAccount: user1VcoinAccount,
        poolVault: poolVaultPda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user1])
      .rpc();

    const userStake = await program.account.userStake.fetch(user1StakePda);
    expect(userStake.stakedAmount.toString()).to.equal(stakeAmount.toString());
    expect(userStake.tier).to.equal(2); // Silver tier
    
    // veVCoin = 10,000 * (1/4) * 1.2 = 3,000
    const expectedVeVCoin = (10_000 * (ONE_YEAR / FOUR_YEARS) * 1.2) * 1e9;
    expect(userStake.veVcoinAmount.toNumber()).to.be.closeTo(expectedVeVCoin, 1e9);

    const pool = await program.account.stakingPool.fetch(poolPda);
    expect(pool.totalStaked.toString()).to.equal(stakeAmount.toString());
    expect(pool.totalStakers.toNumber()).to.equal(1);

    console.log("User1 staked 10,000 VCoin for 1 year (Silver tier)");
    console.log("veVCoin received:", userStake.veVcoinAmount.toNumber() / 1e9);
  });

  it("Stakes VCoin with 4 year lock (Platinum tier)", async () => {
    const stakeAmount = new anchor.BN(100_000 * 1e9); // 100,000 VCoin (Platinum tier)
    const lockDuration = new anchor.BN(FOUR_YEARS);

    await program.methods
      .stake(stakeAmount, lockDuration)
      .accounts({
        user: user2.publicKey,
        pool: poolPda,
        userStake: user2StakePda,
        vcoinMint: vcoinMint,
        userVcoinAccount: user2VcoinAccount,
        poolVault: poolVaultPda,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user2])
      .rpc();

    const userStake = await program.account.userStake.fetch(user2StakePda);
    expect(userStake.stakedAmount.toString()).to.equal(stakeAmount.toString());
    expect(userStake.tier).to.equal(4); // Platinum tier
    
    // veVCoin = 100,000 * (4/4) * 1.4 = 140,000
    const expectedVeVCoin = (100_000 * 1 * 1.4) * 1e9;
    expect(userStake.veVcoinAmount.toNumber()).to.be.closeTo(expectedVeVCoin, 1e9);

    console.log("User2 staked 100,000 VCoin for 4 years (Platinum tier)");
    console.log("veVCoin received:", userStake.veVcoinAmount.toNumber() / 1e9);
  });

  it("Extends lock duration", async () => {
    const newLockDuration = new anchor.BN(2 * ONE_YEAR); // Extend to 2 years

    const stakeBeforeExtend = await program.account.userStake.fetch(user1StakePda);
    const veVCoinBefore = stakeBeforeExtend.veVcoinAmount;

    await program.methods
      .extendLock(newLockDuration)
      .accounts({
        user: user1.publicKey,
        userStake: user1StakePda,
      })
      .signers([user1])
      .rpc();

    const userStake = await program.account.userStake.fetch(user1StakePda);
    expect(userStake.lockDuration.toNumber()).to.equal(2 * ONE_YEAR);
    expect(userStake.veVcoinAmount.toNumber()).to.be.greaterThan(veVCoinBefore.toNumber());

    console.log("User1 extended lock to 2 years");
    console.log("New veVCoin:", userStake.veVcoinAmount.toNumber() / 1e9);
  });

  it("Fails to unstake while tokens are locked", async () => {
    try {
      await program.methods
        .unstake(new anchor.BN(1000 * 1e9))
        .accounts({
          user: user1.publicKey,
          pool: poolPda,
          userStake: user1StakePda,
          vcoinMint: vcoinMint,
          userVcoinAccount: user1VcoinAccount,
          poolVault: poolVaultPda,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([user1])
        .rpc();
      
      expect.fail("Should have thrown locked error");
    } catch (error: any) {
      expect(error.message).to.include("TokensStillLocked");
    }

    console.log("Early unstake correctly rejected");
  });

  it("Pauses and unpauses the pool", async () => {
    // Pause
    await program.methods
      .setPaused(true)
      .accounts({
        authority: authority.publicKey,
        pool: poolPda,
      })
      .signers([authority])
      .rpc();

    let pool = await program.account.stakingPool.fetch(poolPda);
    expect(pool.paused).to.equal(true);

    // Try to stake while paused
    try {
      await program.methods
        .stake(new anchor.BN(1000 * 1e9), new anchor.BN(ONE_WEEK))
        .accounts({
          user: user1.publicKey,
          pool: poolPda,
          userStake: user1StakePda,
          vcoinMint: vcoinMint,
          userVcoinAccount: user1VcoinAccount,
          poolVault: poolVaultPda,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([user1])
        .rpc();
      
      expect.fail("Should have thrown paused error");
    } catch (error: any) {
      expect(error.message).to.include("PoolPaused");
    }

    // Unpause
    await program.methods
      .setPaused(false)
      .accounts({
        authority: authority.publicKey,
        pool: poolPda,
      })
      .signers([authority])
      .rpc();

    pool = await program.account.stakingPool.fetch(poolPda);
    expect(pool.paused).to.equal(false);

    console.log("Pool pause/unpause working correctly");
  });

  it("Gets stake info", async () => {
    const stakeInfo = await program.methods
      .getStakeInfo()
      .accounts({
        user: user1.publicKey,
        userStake: user1StakePda,
      })
      .view();

    console.log("User1 Stake Info:");
    console.log("  Staked Amount:", stakeInfo.stakedAmount.toNumber() / 1e9, "VCoin");
    console.log("  Tier:", stakeInfo.tier);
    console.log("  veVCoin:", stakeInfo.veVcoinAmount.toNumber() / 1e9);
    console.log("  Is Locked:", stakeInfo.isLocked);
    console.log("  Fee Discount:", stakeInfo.feeDiscountBps / 100, "%");
  });
});

