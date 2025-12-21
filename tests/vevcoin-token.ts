import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VevcoinToken } from "../target/types/vevcoin_token";
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
  getAccount,
} from "@solana/spl-token";
import { expect } from "chai";

describe("vevcoin-token", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.VevcoinToken as Program<VevcoinToken>;
  
  // Test accounts
  const authority = Keypair.generate();
  const stakingProtocol = Keypair.generate();
  const user1 = Keypair.generate();
  
  let vevcoinMint: PublicKey;
  let user1TokenAccount: PublicKey;
  let configPda: PublicKey;
  let user1AccountPda: PublicKey;

  before(async () => {
    // Airdrop SOL
    for (const account of [authority, stakingProtocol, user1]) {
      const signature = await provider.connection.requestAirdrop(
        account.publicKey,
        5 * LAMPORTS_PER_SOL
      );
      await provider.connection.confirmTransaction(signature);
    }

    // Find PDAs
    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vevcoin-config")],
      program.programId
    );

    [user1AccountPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user-vevcoin"), user1.publicKey.toBuffer()],
      program.programId
    );

    console.log("Authority:", authority.publicKey.toBase58());
    console.log("Staking Protocol:", stakingProtocol.publicKey.toBase58());
    console.log("User1:", user1.publicKey.toBase58());
  });

  it("Initializes the veVCoin mint", async () => {
    // Create veVCoin mint with Token-2022 (soulbound - non-transferable)
    vevcoinMint = await createMint(
      provider.connection,
      authority,
      configPda, // Mint authority is config PDA
      null,
      9,
      Keypair.generate(),
      undefined,
      TOKEN_2022_PROGRAM_ID
    );

    console.log("veVCoin Mint:", vevcoinMint.toBase58());

    // Initialize config
    await program.methods
      .initializeMint(stakingProtocol.publicKey)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
        mint: vevcoinMint,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      })
      .signers([authority])
      .rpc();

    const config = await program.account.veVCoinConfig.fetch(configPda);
    expect(config.authority.toBase58()).to.equal(authority.publicKey.toBase58());
    expect(config.mint.toBase58()).to.equal(vevcoinMint.toBase58());
    expect(config.stakingProtocol.toBase58()).to.equal(stakingProtocol.publicKey.toBase58());
    expect(config.totalSupply.toNumber()).to.equal(0);

    console.log("veVCoin config initialized!");
  });

  it("Mints veVCoin when called by staking protocol", async () => {
    // Create user token account
    const userAccount = await getOrCreateAssociatedTokenAccount(
      provider.connection,
      user1,
      vevcoinMint,
      user1.publicKey,
      false,
      undefined,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    user1TokenAccount = userAccount.address;

    const mintAmount = new anchor.BN(10000 * 1e9); // 10,000 veVCoin

    await program.methods
      .mintVevcoin(mintAmount)
      .accounts({
        stakingProtocol: stakingProtocol.publicKey,
        user: user1.publicKey,
        config: configPda,
        userAccount: user1AccountPda,
        mint: vevcoinMint,
        userTokenAccount: user1TokenAccount,
        payer: stakingProtocol.publicKey,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([stakingProtocol])
      .rpc();

    // Verify balances
    const config = await program.account.veVCoinConfig.fetch(configPda);
    expect(config.totalSupply.toString()).to.equal(mintAmount.toString());
    expect(config.totalHolders.toNumber()).to.equal(1);

    const userVeVCoin = await program.account.userVeVCoin.fetch(user1AccountPda);
    expect(userVeVCoin.balance.toString()).to.equal(mintAmount.toString());

    console.log("Minted", mintAmount.toString(), "veVCoin to user1");
  });

  it("Burns veVCoin when called by staking protocol", async () => {
    const burnAmount = new anchor.BN(5000 * 1e9); // Burn 5,000

    await program.methods
      .burnVevcoin(burnAmount)
      .accounts({
        stakingProtocol: stakingProtocol.publicKey,
        user: user1.publicKey,
        config: configPda,
        userAccount: user1AccountPda,
        mint: vevcoinMint,
        userTokenAccount: user1TokenAccount,
        tokenProgram: TOKEN_2022_PROGRAM_ID,
      })
      .signers([stakingProtocol])
      .rpc();

    const userVeVCoin = await program.account.userVeVCoin.fetch(user1AccountPda);
    expect(userVeVCoin.balance.toNumber()).to.equal(5000 * 1e9);

    console.log("Burned 5000 veVCoin from user1");
  });

  it("Fails when non-staking protocol tries to mint", async () => {
    const unauthorizedMinter = Keypair.generate();
    const sig = await provider.connection.requestAirdrop(
      unauthorizedMinter.publicKey,
      LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    try {
      await program.methods
        .mintVevcoin(new anchor.BN(1000))
        .accounts({
          stakingProtocol: unauthorizedMinter.publicKey,
          user: user1.publicKey,
          config: configPda,
          userAccount: user1AccountPda,
          mint: vevcoinMint,
          userTokenAccount: user1TokenAccount,
          payer: unauthorizedMinter.publicKey,
          tokenProgram: TOKEN_2022_PROGRAM_ID,
          systemProgram: SystemProgram.programId,
        })
        .signers([unauthorizedMinter])
        .rpc();
      
      expect.fail("Should have thrown unauthorized error");
    } catch (error: any) {
      expect(error.message).to.include("Unauthorized");
    }

    console.log("Unauthorized mint correctly rejected");
  });

  it("Updates staking protocol address", async () => {
    const newStakingProtocol = Keypair.generate();

    await program.methods
      .updateStakingProtocol(newStakingProtocol.publicKey)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
      })
      .signers([authority])
      .rpc();

    const config = await program.account.veVCoinConfig.fetch(configPda);
    expect(config.stakingProtocol.toBase58()).to.equal(newStakingProtocol.publicKey.toBase58());

    // Revert for other tests
    await program.methods
      .updateStakingProtocol(stakingProtocol.publicKey)
      .accounts({
        authority: authority.publicKey,
        config: configPda,
      })
      .signers([authority])
      .rpc();

    console.log("Staking protocol update working correctly");
  });
});

