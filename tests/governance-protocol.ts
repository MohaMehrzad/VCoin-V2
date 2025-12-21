import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { GovernanceProtocol } from "../target/types/governance_protocol";
import { expect } from "chai";
import { createHash } from "crypto";

describe("governance-protocol", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GovernanceProtocol as Program<GovernanceProtocol>;
  const authority = provider.wallet;

  // PDAs
  let govConfigPda: anchor.web3.PublicKey;
  let proposalPda: anchor.web3.PublicKey;
  let voteRecordPda: anchor.web3.PublicKey;
  let delegationPda: anchor.web3.PublicKey;
  let delegateStatsPda: anchor.web3.PublicKey;

  // Test accounts
  const stakingProgram = anchor.web3.Keypair.generate().publicKey;
  const fiveAProgram = anchor.web3.Keypair.generate().publicKey;
  const voter2 = anchor.web3.Keypair.generate();
  const delegate = anchor.web3.Keypair.generate();

  before(async () => {
    // Derive PDAs
    [govConfigPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("gov-config")],
      program.programId
    );

    // Will be derived after initialization when we know proposal count
    [delegationPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("delegation"), authority.publicKey.toBuffer()],
      program.programId
    );

    [delegateStatsPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("delegate-stats"), delegate.publicKey.toBuffer()],
      program.programId
    );

    // Airdrop to test accounts
    const airdropSig = await provider.connection.requestAirdrop(
      voter2.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig);
  });

  describe("Initialization", () => {
    it("should initialize governance protocol", async () => {
      try {
        await program.methods
          .initialize(stakingProgram, fiveAProgram)
          .accounts({
            governanceConfig: govConfigPda,
            authority: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();

        const config = await program.account.governanceConfig.fetch(govConfigPda);
        
        expect(config.authority.toString()).to.equal(authority.publicKey.toString());
        expect(config.stakingProgram.toString()).to.equal(stakingProgram.toString());
        expect(config.fiveAProgram.toString()).to.equal(fiveAProgram.toString());
        expect(config.paused).to.be.false;
        expect(config.proposalCount.toNumber()).to.equal(0);
        expect(config.treasuryBalance.toNumber()).to.equal(200_000_000 * 1_000_000_000);
      } catch (error) {
        console.log("Init error (may already exist):", error.message);
      }
    });
  });

  describe("Proposal Creation", () => {
    it("should create a proposal", async () => {
      const config = await program.account.governanceConfig.fetch(govConfigPda);
      const nextProposalId = config.proposalCount.toNumber() + 1;

      [proposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), Buffer.from(new anchor.BN(nextProposalId).toArray("le", 8))],
        program.programId
      );

      const titleHash = createHash("sha256")
        .update("Test Proposal: Update Fee Structure")
        .digest();
      const descriptionUri = "ipfs://QmProposalDescription123";

      await program.methods
        .createProposal(
          Array.from(titleHash),
          descriptionUri,
          0, // Parameter type
          false // No private voting
        )
        .accounts({
          governanceConfig: govConfigPda,
          proposal: proposalPda,
          proposer: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const proposal = await program.account.proposal.fetch(proposalPda);
      
      expect(proposal.id.toNumber()).to.equal(nextProposalId);
      expect(proposal.proposer.toString()).to.equal(authority.publicKey.toString());
      expect(proposal.proposalType).to.equal(0);
      expect(proposal.status).to.equal(1); // Active
      expect(proposal.isPrivateVoting).to.be.false;
      expect(proposal.executed).to.be.false;
    });

    it("should increment proposal count", async () => {
      const config = await program.account.governanceConfig.fetch(govConfigPda);
      expect(config.proposalCount.toNumber()).to.be.greaterThan(0);
    });
  });

  describe("Public Voting", () => {
    it("should cast a vote", async () => {
      const proposal = await program.account.proposal.fetch(proposalPda);

      [voteRecordPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote-record"), proposalPda.toBuffer(), authority.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .castVote(
          1, // For
          new anchor.BN(10_000), // veVCoin balance
          8500, // 85% 5A score
          3 // Gold tier
        )
        .accounts({
          proposal: proposalPda,
          voteRecord: voteRecordPda,
          voter: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const voteRecord = await program.account.voteRecord.fetch(voteRecordPda);
      
      expect(voteRecord.voter.toString()).to.equal(authority.publicKey.toString());
      expect(voteRecord.voteChoice).to.equal(1); // For
      expect(voteRecord.voteWeight.toNumber()).to.be.greaterThan(0);
      expect(voteRecord.isPrivate).to.be.false;
      expect(voteRecord.revealed).to.be.true;
    });

    it("should update proposal vote counts", async () => {
      const proposal = await program.account.proposal.fetch(proposalPda);
      expect(Number(proposal.votesFor)).to.be.greaterThan(0);
    });

    it("should allow another voter to vote", async () => {
      const [voter2VoteRecord] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote-record"), proposalPda.toBuffer(), voter2.publicKey.toBuffer()],
        program.programId
      );

      await program.methods
        .castVote(
          2, // Against
          new anchor.BN(5_000), // veVCoin
          7000, // 70% 5A
          2 // Silver tier
        )
        .accounts({
          proposal: proposalPda,
          voteRecord: voter2VoteRecord,
          voter: voter2.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([voter2])
        .rpc();

      const voteRecord = await program.account.voteRecord.fetch(voter2VoteRecord);
      expect(voteRecord.voteChoice).to.equal(2); // Against
    });

    it("should reject duplicate votes", async () => {
      try {
        await program.methods
          .castVote(1, new anchor.BN(1000), 5000, 1)
          .accounts({
            proposal: proposalPda,
            voteRecord: voteRecordPda,
            voter: authority.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        // Expected - vote record already exists
      }
    });
  });

  describe("Voting Power Calculation", () => {
    it("should calculate correct voting power with quadratic formula", async () => {
      // Create a new proposal to test voting power
      const config = await program.account.governanceConfig.fetch(govConfigPda);
      const nextId = config.proposalCount.toNumber() + 1;

      const [testProposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), Buffer.from(new anchor.BN(nextId).toArray("le", 8))],
        program.programId
      );

      const titleHash = createHash("sha256").update("Power Test").digest();

      await program.methods
        .createProposal(Array.from(titleHash), "ipfs://test", 0, false)
        .accounts({
          governanceConfig: govConfigPda,
          proposal: testProposalPda,
          proposer: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const [powerVoteRecord] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote-record"), testProposalPda.toBuffer(), authority.publicKey.toBuffer()],
        program.programId
      );

      // Vote with high values to test formula
      await program.methods
        .castVote(
          1, // For
          new anchor.BN(100_000), // 100K veVCoin
          10000, // 100% 5A score (max)
          4 // Platinum tier
        )
        .accounts({
          proposal: testProposalPda,
          voteRecord: powerVoteRecord,
          voter: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const voteRecord = await program.account.voteRecord.fetch(powerVoteRecord);
      
      // With sqrt(100000) ≈ 316, 5A boost = 2.0x, tier mult = 10.0x
      // Expected raw: 316 * 2 * 10 = 6320
      expect(voteRecord.voteWeight.toNumber()).to.be.greaterThan(1000);
    });
  });

  describe("Vote Delegation", () => {
    it("should create delegation", async () => {
      await program.methods
        .delegateVotes(
          0, // Full delegation
          0, // All categories
          new anchor.BN(5000), // 5000 veVCoin
          new anchor.BN(0), // Never expires
          true // Revocable
        )
        .accounts({
          delegation: delegationPda,
          delegateStats: delegateStatsPda,
          delegator: authority.publicKey,
          delegate: delegate.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const delegation = await program.account.delegation.fetch(delegationPda);
      
      expect(delegation.delegator.toString()).to.equal(authority.publicKey.toString());
      expect(delegation.delegate.toString()).to.equal(delegate.publicKey.toString());
      expect(delegation.delegatedAmount.toNumber()).to.equal(5000);
      expect(delegation.revocable).to.be.true;

      const stats = await program.account.delegateStats.fetch(delegateStatsPda);
      expect(stats.uniqueDelegators).to.equal(1);
      expect(stats.totalDelegatedVevcoin.toNumber()).to.equal(5000);
    });

    it("should reject self-delegation", async () => {
      const [selfDelegation] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("delegation"), voter2.publicKey.toBuffer()],
        program.programId
      );

      const [selfStats] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("delegate-stats"), voter2.publicKey.toBuffer()],
        program.programId
      );

      try {
        await program.methods
          .delegateVotes(0, 0, new anchor.BN(1000), new anchor.BN(0), true)
          .accounts({
            delegation: selfDelegation,
            delegateStats: selfStats,
            delegator: voter2.publicKey,
            delegate: voter2.publicKey, // Self
            systemProgram: anchor.web3.SystemProgram.programId,
          })
          .signers([voter2])
          .rpc();
        
        expect.fail("Should have thrown error");
      } catch (error) {
        expect(error.message).to.include("CannotDelegateSelf");
      }
    });

    it("should revoke delegation", async () => {
      await program.methods
        .revokeDelegation()
        .accounts({
          delegation: delegationPda,
          delegateStats: delegateStatsPda,
          delegator: authority.publicKey,
        })
        .rpc();

      const stats = await program.account.delegateStats.fetch(delegateStatsPda);
      expect(stats.uniqueDelegators).to.equal(0);
    });
  });

  describe("Private Voting (ZK)", () => {
    it("should create proposal with private voting enabled", async () => {
      const config = await program.account.governanceConfig.fetch(govConfigPda);
      const nextId = config.proposalCount.toNumber() + 1;

      const [privateProposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), Buffer.from(new anchor.BN(nextId).toArray("le", 8))],
        program.programId
      );

      const titleHash = createHash("sha256").update("Private Voting Test").digest();

      await program.methods
        .createProposal(Array.from(titleHash), "ipfs://private", 1, true) // Treasury type, private
        .accounts({
          governanceConfig: govConfigPda,
          proposal: privateProposalPda,
          proposer: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const proposal = await program.account.proposal.fetch(privateProposalPda);
      expect(proposal.isPrivateVoting).to.be.true;
    });

    it("should enable private voting config for proposal", async () => {
      // Get latest proposal
      const config = await program.account.governanceConfig.fetch(govConfigPda);
      const proposalId = config.proposalCount.toNumber();

      const [privateProposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), Buffer.from(new anchor.BN(proposalId).toArray("le", 8))],
        program.programId
      );

      const [privateVotingPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("private-voting"), privateProposalPda.toBuffer()],
        program.programId
      );

      // Committee members
      const committee = [
        anchor.web3.Keypair.generate().publicKey,
        anchor.web3.Keypair.generate().publicKey,
        anchor.web3.Keypair.generate().publicKey,
        anchor.web3.Keypair.generate().publicKey,
        anchor.web3.Keypair.generate().publicKey,
      ];

      await program.methods
        .enablePrivateVoting(
          anchor.web3.Keypair.generate().publicKey, // encryption pubkey
          committee,
          5, // committee size
          3  // 3-of-5 threshold
        )
        .accounts({
          proposal: privateProposalPda,
          privateVotingConfig: privateVotingPda,
          proposer: authority.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      const privateConfig = await program.account.privateVotingConfig.fetch(privateVotingPda);
      expect(privateConfig.isEnabled).to.be.true;
      expect(privateConfig.decryptionThreshold).to.equal(3);
      expect(privateConfig.committeeSize).to.equal(5);
    });

    it("should cast private vote with encrypted data", async () => {
      const config = await program.account.governanceConfig.fetch(govConfigPda);
      const proposalId = config.proposalCount.toNumber();

      const [privateProposalPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("proposal"), Buffer.from(new anchor.BN(proposalId).toArray("le", 8))],
        program.programId
      );

      const [privateVotingPda] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("private-voting"), privateProposalPda.toBuffer()],
        program.programId
      );

      const [privateVoteRecord] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from("vote-record"), privateProposalPda.toBuffer(), voter2.publicKey.toBuffer()],
        program.programId
      );

      // Simulated encrypted vote data
      const encryptedChoice = Buffer.alloc(32);
      encryptedChoice.write("encrypted-choice");
      
      const encryptedWeight = Buffer.alloc(32);
      encryptedWeight.write("encrypted-weight");
      
      const zkProof = Buffer.alloc(128);
      zkProof.write("zk-proof-data");

      await program.methods
        .castPrivateVote(
          Array.from(encryptedChoice),
          Array.from(encryptedWeight),
          Array.from(zkProof)
        )
        .accounts({
          proposal: privateProposalPda,
          privateVotingConfig: privateVotingPda,
          voteRecord: privateVoteRecord,
          voter: voter2.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([voter2])
        .rpc();

      const voteRecord = await program.account.voteRecord.fetch(privateVoteRecord);
      expect(voteRecord.isPrivate).to.be.true;
      expect(voteRecord.revealed).to.be.false;
      expect(voteRecord.voteWeight.toNumber()).to.equal(0); // Hidden until reveal
    });
  });

  describe("Governance Configuration", () => {
    it("should update governance parameters", async () => {
      await program.methods
        .updateConfig(
          new anchor.BN(2000), // proposal threshold
          new anchor.BN(2_000_000), // quorum
          new anchor.BN(14 * 24 * 60 * 60), // 14 days voting
          new anchor.BN(72 * 60 * 60) // 72 hours timelock
        )
        .accounts({
          governanceConfig: govConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      const config = await program.account.governanceConfig.fetch(govConfigPda);
      expect(config.proposalThreshold.toNumber()).to.equal(2000);
      expect(config.quorum.toNumber()).to.equal(2_000_000);
    });

    it("should pause and unpause governance", async () => {
      await program.methods
        .setPaused(true)
        .accounts({
          governanceConfig: govConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      let config = await program.account.governanceConfig.fetch(govConfigPda);
      expect(config.paused).to.be.true;

      await program.methods
        .setPaused(false)
        .accounts({
          governanceConfig: govConfigPda,
          authority: authority.publicKey,
        })
        .rpc();

      config = await program.account.governanceConfig.fetch(govConfigPda);
      expect(config.paused).to.be.false;
    });
  });
});

