import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VcoinToken } from "../target/types/vcoin_token";
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

describe("vcoin-token", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VcoinToken as Program<VcoinToken>;
  
  // Test accounts
  const authority = Keypair.generate();
  const permanentDelegate = Keypair.generate();
  const treasury = Keypair.generate();
  
  let vcoinMint: PublicKey;
  let treasuryTokenAccount: PublicKey;
  let configPda: PublicKey;
  let configBump: number;

  before(async () => {
    // Airdrop SOL to authority for transaction fees
    const signature = await provider.connection.requestAirdrop(
      authority.publicKey,
      10 * LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);

    // Find the config PDA
    [configPda, configBump] = PublicKey.findProgramAddressSync(
      [Buffer.from("vcoin-config")],
      program.programId
    );

    console.log("Authority:", authority.publicKey.toBase58());
    console.log("Config PDA:", configPda.toBase58());
    console.log("Permanent Delegate:", permanentDelegate.publicKey.toBase58());
  });

  it("Initializes the VCoin mint", async () => {
    // Create the VCoin mint with Token-2022
    vcoinMint = await createMint(
      provider.connection,
      authority,
      configPda, // Mint authority is the config PDA
      null, // No freeze authority
      9, // 9 decimals
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    console.log("VCoin Mint:", vcoinMint.toBase58());

    // Create treasury token account
    const treasuryAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      authority,
      vcoinMint,
      treasury.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    treasuryTokenAccount = treasuryAccount.address;

    // Initialize the VCoin config
    await program.methods
      .initializeMint(permanentDelegate.publicKey)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
        mint: vcoinMint,
        treasury: treasuryTokenAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([authority])
      .rpc();

    // Verify config was initialized
    const config = await program.account.vCoinConfig.fetch(configPda);
    expect(config.authority.toBase58()).to.equal(authority.publicKey.toBase58());
    expect(config.mint.toBase58()).to.equal(vcoinMint.toBase58());
    expect(config.permanentDelegate.toBase58()).to.equal(permanentDelegate.publicKey.toBase58());
    expect(config.totalMinted.toNumber()).to.equal(0);
    expect(config.paused).to.equal(false);

    console.log("VCoin config initialized successfully!");
  });

  it("Mints VCoin tokens to treasury", async () => {
    const mintAmount = new anchor.BN(1_000_000_000 * 1e9); // 1B tokens

    await program.methods
      .mintTokens(mintAmount)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
        mint: vcoinMint,
        destination: treasuryTokenAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([authority])
      .rpc();

    // Verify tokens were minted
    const config = await program.account.vCoinConfig.fetch(configPda);
    expect(config.totalMinted.toString()).to.equal(mintAmount.toString());

    // Check treasury balance
    const treasuryBalance = await getAccount(
      provider.connection,
      treasuryTokenAccount,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    expect(treasuryBalance.amount.toString()).to.equal(mintAmount.toString());

    console.log("Minted", mintAmount.toString(), "tokens to treasury");
  });

  it("Pauses and unpauses the token", async () => {
    // Pause
    await program.methods
      .setPaused(true)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
      })
      .signers([authority])
      .rpc();

    let config = await program.account.vCoinConfig.fetch(configPda);
    expect(config.paused).to.equal(true);

    // Unpause
    await program.methods
      .setPaused(false)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
      })
      .signers([authority])
      .rpc();

    config = await program.account.vCoinConfig.fetch(configPda);
    expect(config.paused).to.equal(false);

    console.log("Pause/unpause working correctly");
  });

  it("Updates the authority", async () => {
    const newAuthority = Keypair.generate();

    await program.methods
      .updateAuthority(newAuthority.publicKey)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
      })
      .signers([authority])
      .rpc();

    const config = await program.account.vCoinConfig.fetch(configPda);
    expect(config.authority.toBase58()).to.equal(newAuthority.publicKey.toBase58());

    // Update back for other tests
    await program.methods
      .updateAuthority(authority.publicKey)
      .accounts({
        authority: newAuthority.publicKey,
        config: configPda,
      })
      .signers([newAuthority])
      .rpc();

    console.log("Authority update working correctly");
  });

  it("Fails to mint when unauthorized", async () => {
    const unauthorizedUser = Keypair.generate();
    const signature = await provider.connection.requestAirdrop(
      unauthorizedUser.publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(signature);

    try {
      await program.methods
        .mintTokens(new anchor.BN(1000))
        .accounts({
          authority: unauthorizedUser.publicKey,
          config: configPda,
          mint: vcoinMint,
          destination: treasuryTokenAccount,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
        })
        .signers([unauthorizedUser])
        .rpc();
      
      expect.fail("Should have thrown unauthorized error");
    } catch (error: any) {
      expect(error.message).to.include("Unauthorized");
    }

    console.log("Unauthorized mint correctly rejected");
  });
});

