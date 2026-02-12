use anchor_lang::prelude::*;

declare_id!("3R256kBN9iXozjypQFRAmegBhd6HJqXWqdNG7Th78HYe");

/// Governance Protocol
/// 
/// Full on-chain governance using veVCoin for voting power, boosted by 5A score.
/// Supports ZK Private Voting where votes are encrypted during voting period.
/// 
/// Voting Power Formula:
/// - Quadratic: base_votes = sqrt(vcoin_tokens)
/// - 5A Boost: five_a_boost = 1.0 + (five_a_score / 100)  // 1.0x to 2.0x
/// - Tier Multiplier: Bronze=1.0x, Silver=2.0x, Gold=5.0x, Platinum=10.0x
/// - effective_votes = base_votes * five_a_boost * tier_multiplier
/// 
/// Governance Tiers:
/// - Community (1+ veVCoin): Can vote
/// - Delegate (1,000+ veVCoin): Can create proposals
/// - Council (10,000+ veVCoin): Fast-track proposals

pub mod constants;
pub mod crypto;
pub mod errors;
pub mod events;
pub mod state;
pub mod contexts;
pub mod instructions;

#[cfg(test)]
mod tests;

use contexts::*;
use instructions::*;

#[program]
pub mod governance_protocol {
    use super::*;

    /// Initialize governance protocol
    pub fn initialize(
        ctx: Context<Initialize>,
        staking_program: Pubkey,
        five_a_program: Pubkey,
    ) -> Result<()> {
        admin::initialize::handler(ctx, staking_program, five_a_program)
    }
    
    /// Create a new proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title_hash: [u8; 32],
        description_uri: String,
        proposal_type: u8,
        enable_private_voting: bool,
    ) -> Result<()> {
        proposal::create::handler(ctx, title_hash, description_uri, proposal_type, enable_private_voting)
    }
    
    /// Cast a public vote
    /// 
    /// C-NEW-01 Security Fix: veVCoin balance, 5A score, and tier are now
    /// read directly from on-chain accounts instead of being passed as parameters.
    /// This prevents vote manipulation attacks.
    pub fn cast_vote(
        ctx: Context<CastVote>,
        choice: u8,
    ) -> Result<()> {
        vote::cast::handler(ctx, choice)
    }
    
    /// Cast a ZK private vote with cryptographic proof verification
    ///
    /// The voter encrypts their choice using Twisted ElGamal and provides
    /// OR proofs that each ciphertext encrypts 0 or weight, plus a sum proof
    /// that exactly one category equals weight. Voting power is verified on-chain.
    pub fn cast_private_vote(
        ctx: Context<CastPrivateVote>,
        ct_for: [u8; 64],
        ct_against: [u8; 64],
        ct_abstain: [u8; 64],
        proof_data: Vec<u8>,
    ) -> Result<()> {
        vote::cast_private::handler(ctx, ct_for, ct_against, ct_abstain, proof_data)
    }

    /// Enable ZK private voting for a proposal with ElGamal committee keys
    pub fn enable_private_voting(
        ctx: Context<EnablePrivateVoting>,
        encryption_pubkey: [u8; 32],
        decryption_committee: [Pubkey; 5],
        committee_size: u8,
        decryption_threshold: u8,
        committee_elgamal_pubkeys: [[u8; 32]; 5],
    ) -> Result<()> {
        zk_voting::enable_private_voting::handler(
            ctx, encryption_pubkey, decryption_committee,
            committee_size, decryption_threshold, committee_elgamal_pubkeys,
        )
    }

    /// Initiate ZK vote reveal after voting ends
    pub fn initiate_reveal(ctx: Context<InitiateReveal>) -> Result<()> {
        zk_voting::initiate_reveal::handler(ctx)
    }

    /// Submit decryption share with DLEQ proof (committee member)
    pub fn submit_decryption_share(
        ctx: Context<SubmitDecryptionShare>,
        committee_index: u8,
        partial_for: [u8; 32],
        partial_against: [u8; 32],
        partial_abstain: [u8; 32],
        dleq_challenge: [u8; 32],
        dleq_response: [u8; 32],
    ) -> Result<()> {
        zk_voting::submit_decryption_share::handler(
            ctx, committee_index, partial_for, partial_against,
            partial_abstain, dleq_challenge, dleq_response,
        )
    }

    /// Complete ZK reveal with trustless on-chain tally verification
    ///
    /// Anyone can submit the tally. On-chain verification ensures:
    ///   tally * H == C_sum - D
    /// for each category, making fabrication cryptographically impossible.
    pub fn aggregate_revealed_votes(
        ctx: Context<AggregateRevealedVotes>,
        tally_for: u64,
        tally_against: u64,
        tally_abstain: u64,
        lagrange_coefficients: Vec<[u8; 32]>,
    ) -> Result<()> {
        zk_voting::aggregate_revealed_votes::handler(
            ctx, tally_for, tally_against, tally_abstain, lagrange_coefficients,
        )
    }
    
    /// Finalize proposal (determine pass/fail)
    pub fn finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
        proposal::finalize::handler(ctx)
    }
    
    /// Execute a passed proposal
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        proposal::execute::handler(ctx)
    }
    
    /// Delegate voting power
    pub fn delegate_votes(
        ctx: Context<DelegateVotes>,
        delegation_type: u8,
        categories: u8,
        vevcoin_amount: u64,
        expires_at: i64,
        revocable: bool,
    ) -> Result<()> {
        delegation::delegate::handler(ctx, delegation_type, categories, vevcoin_amount, expires_at, revocable)
    }
    
    /// Revoke delegation
    pub fn revoke_delegation(ctx: Context<RevokeDelegation>) -> Result<()> {
        delegation::revoke::handler(ctx)
    }
    
    /// Update governance parameters
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        proposal_threshold: Option<u64>,
        quorum: Option<u64>,
        voting_period: Option<i64>,
        timelock_delay: Option<i64>,
    ) -> Result<()> {
        admin::update_config::handler(ctx, proposal_threshold, quorum, voting_period, timelock_delay)
    }
    
    /// Pause/unpause governance
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        admin::set_paused::handler(ctx, paused)
    }
    
    /// Propose a new authority (step 1 of two-step transfer - H-02 security fix)
    pub fn propose_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        admin::update_authority::handler(ctx, new_authority)
    }
    
    /// Accept authority transfer (step 2 of two-step transfer - H-02 security fix)
    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        admin::accept_authority::handler(ctx)
    }
    
    /// Cancel a pending authority transfer (H-02 security fix)
    pub fn cancel_authority_transfer(ctx: Context<UpdateAuthority>) -> Result<()> {
        admin::cancel_authority_transfer::handler(ctx)
    }
    
    /// Get proposal info
    pub fn get_proposal(ctx: Context<GetProposal>) -> Result<()> {
        query::get_proposal::handler(ctx)
    }
}
