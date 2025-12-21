import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VilinkProtocol } from "../target/types/vilink_protocol";
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

describe("vilink-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VilinkProtocol as Program<VilinkProtocol>;
  
  // Test accounts
  let vcoinMint: PublicKey;
  let configPda: PublicKey;
  let treasuryKeypair: Keypair;
  let treasuryTokenAccount: PublicKey;
  let creatorKeypair: Keypair;
  let creatorTokenAccount: PublicKey;
  let targetKeypair: Keypair;
  let targetTokenAccount: PublicKey;
  let executorKeypair: Keypair;
  let executorTokenAccount: PublicKey;
  let dappAuthorityKeypair: Keypair;
  
  // PDAs
  let actionPda: PublicKey;
  let creatorStatsPda: PublicKey;
  let executorStatsPda: PublicKey;
  let targetStatsPda: PublicKey;
  let dappPda: PublicKey;
  
  // Seeds
  const CONFIG_SEED = "vilink-config";
  const ACTION_SEED = "action";
  const USER_STATS_SEED = "user-stats";
  const DAPP_REGISTRY_SEED = "dapp";
  const BATCH_SEED = "batch";
  
  // Action types
  const ACTION_TIP = 0;
  const ACTION_VOUCH = 1;
  const ACTION_FOLLOW = 2;
  
  before(async () => {
    // Generate keypairs
    treasuryKeypair = Keypair.generate();
    creatorKeypair = Keypair.generate();
    targetKeypair = Keypair.generate();
    executorKeypair = Keypair.generate();
    dappAuthorityKeypair = Keypair.generate();
    
    // Airdrop SOL
    const airdropAmount = 10 * LAMPORTS_PER_SOL;
    const accounts = [creatorKeypair, targetKeypair, executorKeypair, treasuryKeypair, dappAuthorityKeypair];
    
    for (const account of accounts) {
      await provider.connection.requestAirdrop(account.publicKey, airdropAmount);
    }
    
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
      [Buffer.from(CONFIG_SEED)],
      program.programId
    );
    
    [creatorStatsPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_STATS_SEED), creatorKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    [executorStatsPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_STATS_SEED), executorKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    [targetStatsPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_STATS_SEED), targetKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    [dappPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(DAPP_REGISTRY_SEED), dappAuthorityKeypair.publicKey.toBuffer()],
      program.programId
    );
    
    console.log("Config PDA:", configPda.toBase58());
    
    // Create token accounts
    const treasuryAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      treasuryKeypair.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    treasuryTokenAccount = treasuryAta.address;
    
    const creatorAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      creatorKeypair.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    creatorTokenAccount = creatorAta.address;
    
    const targetAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      targetKeypair.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    targetTokenAccount = targetAta.address;
    
    const executorAta = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      executorKeypair.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    executorTokenAccount = executorAta.address;
    
    // Mint VCoin to executor for testing
    await mintTo(
      provider.connection,
      (provider.wallet as any).payer,
      vcoinMint,
      executorTokenAccount,
      provider.wallet.publicKey,
      100_000_000_000_000, // 100,000 VCoin
      [],
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    
    console.log("Executor Token Account:", executorTokenAccount.toBase58());
    console.log("✓ Test accounts and tokens created");
  });

  describe("Initialization", () => {
    it("Should initialize ViLink configuration", async () => {
      const tx = await program.methods
        .initialize(treasuryKeypair.publicKey)
        .accounts({
          config: configPda,
          vcoinMint: vcoinMint,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Initialize tx:", tx);
      
      const config = await program.account.viLinkConfig.fetch(configPda);
      expect(config.authority.toBase58()).to.equal(provider.wallet.publicKey.toBase58());
      expect(config.vcoinMint.toBase58()).to.equal(vcoinMint.toBase58());
      expect(config.treasury.toBase58()).to.equal(treasuryKeypair.publicKey.toBase58());
      expect(config.enabledActions).to.equal(255); // All actions enabled
      expect(config.paused).to.be.false;
      expect(config.platformFeeBps).to.equal(250); // 2.5%
      
      console.log("✓ ViLink configuration initialized");
    });
  });

  describe("Action Creation", () => {
    let actionCreationTime: number;
    
    it("Should create a tip action", async () => {
      actionCreationTime = Math.floor(Date.now() / 1000);
      
      // Derive action PDA with timestamp
      [actionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(ACTION_SEED),
          creatorKeypair.publicKey.toBuffer(),
          new anchor.BN(actionCreationTime).toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );
      
      const tipAmount = new anchor.BN(1_000_000_000); // 1 VCoin
      const expirySeconds = new anchor.BN(86400); // 1 day
      const metadataHash = new Array(32).fill(0);
      
      const tx = await program.methods
        .createAction(
          ACTION_TIP,
          tipAmount,
          targetKeypair.publicKey,
          metadataHash,
          expirySeconds,
          true, // one_time
          new anchor.BN(1), // max_executions
          null // no content_id
        )
        .accounts({
          config: configPda,
          action: actionPda,
          userStats: creatorStatsPda,
          creator: creatorKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([creatorKeypair])
        .rpc();
      
      console.log("Create action tx:", tx);
      
      const action = await program.account.viLinkAction.fetch(actionPda);
      expect(action.creator.toBase58()).to.equal(creatorKeypair.publicKey.toBase58());
      expect(action.target.toBase58()).to.equal(targetKeypair.publicKey.toBase58());
      expect(action.actionType).to.equal(ACTION_TIP);
      expect(action.amount.toNumber()).to.equal(1_000_000_000);
      expect(action.executed).to.be.false;
      expect(action.oneTime).to.be.true;
      
      console.log("✓ Tip action created");
      console.log("  Action ID:", Buffer.from(action.actionId).toString("hex").slice(0, 16) + "...");
    });

    it("Should create a vouch action", async () => {
      const vouchCreationTime = Math.floor(Date.now() / 1000) + 1;
      
      const [vouchActionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(ACTION_SEED),
          creatorKeypair.publicKey.toBuffer(),
          new anchor.BN(vouchCreationTime).toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );
      
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const tx = await program.methods
        .createAction(
          ACTION_VOUCH,
          new anchor.BN(0), // no amount for vouch
          targetKeypair.publicKey,
          new Array(32).fill(0),
          new anchor.BN(86400),
          true,
          new anchor.BN(1),
          null
        )
        .accounts({
          config: configPda,
          action: vouchActionPda,
          userStats: creatorStatsPda,
          creator: creatorKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([creatorKeypair])
        .rpc();
      
      console.log("Create vouch action tx:", tx);
      console.log("✓ Vouch action created");
    });

    it("Should create a follow action", async () => {
      const followCreationTime = Math.floor(Date.now() / 1000) + 2;
      
      const [followActionPda] = PublicKey.findProgramAddressSync(
        [
          Buffer.from(ACTION_SEED),
          creatorKeypair.publicKey.toBuffer(),
          new anchor.BN(followCreationTime).toArrayLike(Buffer, "le", 8)
        ],
        program.programId
      );
      
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      const tx = await program.methods
        .createAction(
          ACTION_FOLLOW,
          new anchor.BN(0),
          targetKeypair.publicKey,
          new Array(32).fill(0),
          new anchor.BN(86400),
          false, // reusable
          new anchor.BN(100), // max 100 follows
          null
        )
        .accounts({
          config: configPda,
          action: followActionPda,
          userStats: creatorStatsPda,
          creator: creatorKeypair.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .signers([creatorKeypair])
        .rpc();
      
      console.log("Create follow action tx:", tx);
      console.log("✓ Follow action created (reusable, max 100 executions)");
    });
  });

  describe("dApp Registration", () => {
    it("Should register a dApp", async () => {
      const name = Buffer.alloc(32);
      name.write("TestDApp", 0);
      
      const webhookHash = new Array(32).fill(1);
      const allowedActions = 0xFF; // All actions allowed
      const feeShareBps = 100; // 1% affiliate fee
      
      const tx = await program.methods
        .registerDapp(
          Array.from(name) as number[],
          webhookHash,
          allowedActions,
          feeShareBps
        )
        .accounts({
          config: configPda,
          dapp: dappPda,
          dappAuthority: dappAuthorityKeypair.publicKey,
          authority: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        })
        .rpc();
      
      console.log("Register dApp tx:", tx);
      
      const dapp = await program.account.registeredDApp.fetch(dappPda);
      expect(dapp.isActive).to.be.true;
      expect(dapp.authority.toBase58()).to.equal(dappAuthorityKeypair.publicKey.toBase58());
      expect(dapp.allowedActions).to.equal(0xFF);
      expect(dapp.feeShareBps).to.equal(100);
      
      console.log("✓ dApp registered");
    });
  });

  describe("Configuration Updates", () => {
    it("Should update platform fee", async () => {
      const newFeeBps = 300; // 3%
      
      await program.methods
        .setPlatformFee(newFeeBps)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.viLinkConfig.fetch(configPda);
      expect(config.platformFeeBps).to.equal(300);
      
      // Reset to default
      await program.methods
        .setPlatformFee(250)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      console.log("✓ Platform fee updated");
    });

    it("Should enable/disable action types", async () => {
      // Disable vouch action (bit 1)
      const disabledActions = 0xFF ^ (1 << ACTION_VOUCH); // All except vouch
      
      await program.methods
        .setEnabledActions(disabledActions)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      let config = await program.account.viLinkConfig.fetch(configPda);
      expect(config.enabledActions).to.equal(disabledActions);
      
      // Re-enable all
      await program.methods
        .setEnabledActions(0xFF)
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      config = await program.account.viLinkConfig.fetch(configPda);
      expect(config.enabledActions).to.equal(0xFF);
      
      console.log("✓ Action types configuration updated");
    });

    it("Should update protocol programs", async () => {
      const fakeProgram = Keypair.generate().publicKey;
      
      await program.methods
        .updatePrograms(
          fakeProgram, // 5A
          fakeProgram, // staking
          fakeProgram, // content
          fakeProgram, // governance
          fakeProgram  // gasless
        )
        .accounts({
          config: configPda,
          authority: provider.wallet.publicKey,
        })
        .rpc();
      
      const config = await program.account.viLinkConfig.fetch(configPda);
      expect(config.fiveAProgram.toBase58()).to.equal(fakeProgram.toBase58());
      
      console.log("✓ Protocol programs updated");
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
      
      const config = await program.account.viLinkConfig.fetch(configPda);
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
      
      const config = await program.account.viLinkConfig.fetch(configPda);
      expect(config.paused).to.be.false;
      
      console.log("✓ Protocol unpaused");
    });
  });

  describe("Statistics", () => {
    it("Should fetch user stats", async () => {
      const stats = await program.account.userActionStats.fetch(creatorStatsPda);
      
      expect(stats.user.toBase58()).to.equal(creatorKeypair.publicKey.toBase58());
      expect(stats.actionsCreated.toNumber()).to.be.greaterThan(0);
      
      console.log("✓ User stats retrieved");
      console.log("  Actions created:", stats.actionsCreated.toNumber());
    });

    it("Should fetch config stats", async () => {
      const config = await program.account.viLinkConfig.fetch(configPda);
      
      console.log("✓ Config stats retrieved");
      console.log("  Total actions created:", config.totalActionsCreated.toNumber());
      console.log("  Total actions executed:", config.totalActionsExecuted.toNumber());
      console.log("  Total tip volume:", config.totalTipVolume.toNumber());
    });
  });
});

// Helper to generate URI
function generateViLinkUri(actionId: string, baseUrl: string = "viwo://action"): string {
  return `${baseUrl}/${actionId}`;
}

// Helper to parse action ID from URI
function parseViLinkUri(uri: string): string | null {
  const match = uri.match(/viwo:\/\/action\/([a-f0-9]+)/);
  return match ? match[1] : null;
}

