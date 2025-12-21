use anchor_lang::prelude::*;

declare_id!("3fgzSVwUho1rp4k87ZZ43K9fysxy1WqabDNWTemmD1vi");

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

pub mod constants {
    /// Seeds
    pub const GOV_CONFIG_SEED: &[u8] = b"gov-config";
    pub const PROPOSAL_SEED: &[u8] = b"proposal";
    pub const VOTE_RECORD_SEED: &[u8] = b"vote-record";
    pub const DELEGATION_SEED: &[u8] = b"delegation";
    pub const DELEGATE_STATS_SEED: &[u8] = b"delegate-stats";
    pub const PRIVATE_VOTING_SEED: &[u8] = b"private-voting";
    
    /// Governance thresholds (in veVCoin)
    pub const COMMUNITY_THRESHOLD: u64 = 1;
    pub const DELEGATE_THRESHOLD: u64 = 1_000;
    pub const COUNCIL_THRESHOLD: u64 = 10_000;
    
    /// Default governance parameters
    pub const DEFAULT_VOTING_PERIOD: i64 = 7 * 24 * 60 * 60;  // 7 days
    pub const DEFAULT_TIMELOCK_DELAY: i64 = 48 * 60 * 60;     // 48 hours
    pub const DEFAULT_QUORUM: u64 = 1_000_000;                 // 1M effective votes
    pub const DEFAULT_PROPOSAL_THRESHOLD: u64 = 1_000;         // 1000 veVCoin to propose
    
    /// Tier multipliers (x1000 for precision)
    pub const TIER_MULT_NONE: u64 = 1000;      // 1.0x
    pub const TIER_MULT_BRONZE: u64 = 1000;    // 1.0x
    pub const TIER_MULT_SILVER: u64 = 2000;    // 2.0x
    pub const TIER_MULT_GOLD: u64 = 5000;      // 5.0x
    pub const TIER_MULT_PLATINUM: u64 = 10000; // 10.0x
    
    /// Anti-plutocracy threshold
    pub const DIMINISHING_THRESHOLD: u64 = 100_000;
    
    /// ZK voting constants
    pub const MIN_DECRYPTION_THRESHOLD: u8 = 3;
    pub const MAX_COMMITTEE_SIZE: usize = 5;
}

pub mod errors {
    use super::*;
    
    #[error_code]
    pub enum GovernanceError {
        #[msg("Unauthorized: Only the authority can perform this action")]
        Unauthorized,
        #[msg("Governance is paused")]
        GovernancePaused,
        #[msg("Insufficient veVCoin to create proposal")]
        InsufficientVeVCoin,
        #[msg("Proposal not found")]
        ProposalNotFound,
        #[msg("Voting period has not started")]
        VotingNotStarted,
        #[msg("Voting period has ended")]
        VotingEnded,
        #[msg("Voting period has not ended")]
        VotingNotEnded,
        #[msg("Already voted on this proposal")]
        AlreadyVoted,
        #[msg("Invalid vote choice")]
        InvalidVoteChoice,
        #[msg("Quorum not reached")]
        QuorumNotReached,
        #[msg("Proposal already executed")]
        ProposalAlreadyExecuted,
        #[msg("Timelock not expired")]
        TimelockNotExpired,
        #[msg("Cannot delegate to self")]
        CannotDelegateSelf,
        #[msg("Delegation already exists")]
        DelegationExists,
        #[msg("Delegation not found")]
        DelegationNotFound,
        #[msg("ZK voting not enabled for this proposal")]
        ZKVotingNotEnabled,
        #[msg("ZK reveal not started")]
        RevealNotStarted,
        #[msg("ZK reveal already complete")]
        RevealAlreadyComplete,
        #[msg("Invalid decryption share")]
        InvalidDecryptionShare,
        #[msg("Invalid ZK proof")]
        InvalidZKProof,
        #[msg("Arithmetic overflow")]
        Overflow,
    }
}

pub mod state {
    use super::*;
    use crate::constants::*;
    
    /// Vote choice enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum VoteChoice {
        #[default]
        Abstain = 0,
        For = 1,
        Against = 2,
    }
    
    impl VoteChoice {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(VoteChoice::Abstain),
                1 => Some(VoteChoice::For),
                2 => Some(VoteChoice::Against),
                _ => None,
            }
        }
    }
    
    /// Proposal status enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum ProposalStatus {
        #[default]
        Pending = 0,
        Active = 1,
        Passed = 2,
        Rejected = 3,
        Executed = 4,
        Cancelled = 5,
    }
    
    impl ProposalStatus {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(ProposalStatus::Pending),
                1 => Some(ProposalStatus::Active),
                2 => Some(ProposalStatus::Passed),
                3 => Some(ProposalStatus::Rejected),
                4 => Some(ProposalStatus::Executed),
                5 => Some(ProposalStatus::Cancelled),
                _ => None,
            }
        }
    }
    
    /// Proposal type enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum ProposalType {
        #[default]
        Parameter = 0,
        Treasury = 1,
        Protocol = 2,
        Emissions = 3,
    }
    
    impl ProposalType {
        pub fn from_u8(value: u8) -> Option<Self> {
            match value {
                0 => Some(ProposalType::Parameter),
                1 => Some(ProposalType::Treasury),
                2 => Some(ProposalType::Protocol),
                3 => Some(ProposalType::Emissions),
                _ => None,
            }
        }
    }
    
    /// Delegation type enum
    #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Default)]
    pub enum DelegationType {
        #[default]
        Full = 0,
        PerCategory = 1,
        PerProposal = 2,
    }
    
    /// Global governance configuration
    #[account]
    #[derive(Default)]
    pub struct GovernanceConfig {
        /// Admin authority
        pub authority: Pubkey,
        /// Staking program
        pub staking_program: Pubkey,
        /// 5A Protocol program
        pub five_a_program: Pubkey,
        /// veVCoin required to propose
        pub proposal_threshold: u64,
        /// Minimum votes for valid proposal
        pub quorum: u64,
        /// Voting period in seconds
        pub voting_period: i64,
        /// Timelock delay before execution
        pub timelock_delay: i64,
        /// Total proposals created
        pub proposal_count: u64,
        /// Treasury balance (200M VCoin)
        pub treasury_balance: u64,
        /// Whether governance is paused
        pub paused: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl GovernanceConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // authority
            32 + // staking_program
            32 + // five_a_program
            8 +  // proposal_threshold
            8 +  // quorum
            8 +  // voting_period
            8 +  // timelock_delay
            8 +  // proposal_count
            8 +  // treasury_balance
            1 +  // paused
            1;   // bump
    }
    
    /// Proposal account
    #[account]
    pub struct Proposal {
        /// Proposal ID
        pub id: u64,
        /// Proposer
        pub proposer: Pubkey,
        /// Hash of title
        pub title_hash: [u8; 32],
        /// IPFS link to description
        pub description_uri: [u8; 128],
        /// URI length
        pub uri_len: u8,
        /// Proposal type
        pub proposal_type: u8,
        /// Voting start time
        pub start_time: i64,
        /// Voting end time
        pub end_time: i64,
        /// Votes for (u128 for large-scale)
        pub votes_for: u128,
        /// Votes against
        pub votes_against: u128,
        /// Abstain votes
        pub votes_abstain: u128,
        /// Current status
        pub status: u8,
        /// Execution time (after timelock)
        pub execution_time: i64,
        /// Whether executed
        pub executed: bool,
        /// Whether ZK voting is enabled
        pub is_private_voting: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Default for Proposal {
        fn default() -> Self {
            Self {
                id: 0,
                proposer: Pubkey::default(),
                title_hash: [0u8; 32],
                description_uri: [0u8; 128],
                uri_len: 0,
                proposal_type: 0,
                start_time: 0,
                end_time: 0,
                votes_for: 0,
                votes_against: 0,
                votes_abstain: 0,
                status: 0,
                execution_time: 0,
                executed: false,
                is_private_voting: false,
                bump: 0,
            }
        }
    }
    
    impl Proposal {
        pub const LEN: usize = 8 + // discriminator
            8 +   // id
            32 +  // proposer
            32 +  // title_hash
            128 + // description_uri
            1 +   // uri_len
            1 +   // proposal_type
            8 +   // start_time
            8 +   // end_time
            16 +  // votes_for (u128)
            16 +  // votes_against
            16 +  // votes_abstain
            1 +   // status
            8 +   // execution_time
            1 +   // executed
            1 +   // is_private_voting
            1;    // bump
    }
    
    /// Vote record (PDA per user per proposal)
    #[account]
    pub struct VoteRecord {
        /// Voter
        pub voter: Pubkey,
        /// Proposal
        pub proposal: Pubkey,
        /// Vote weight (veVCoin * 5A boost)
        pub vote_weight: u64,
        /// Vote choice
        pub vote_choice: u8,
        /// Timestamp
        pub voted_at: i64,
        /// Whether this is a ZK encrypted vote
        pub is_private: bool,
        /// Encrypted choice (for ZK voting)
        pub encrypted_choice: [u8; 32],
        /// Encrypted weight (for ZK voting)
        pub encrypted_weight: [u8; 32],
        /// ZK proof
        pub zk_proof: [u8; 128],
        /// Whether vote has been revealed
        pub revealed: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Default for VoteRecord {
        fn default() -> Self {
            Self {
                voter: Pubkey::default(),
                proposal: Pubkey::default(),
                vote_weight: 0,
                vote_choice: 0,
                voted_at: 0,
                is_private: false,
                encrypted_choice: [0u8; 32],
                encrypted_weight: [0u8; 32],
                zk_proof: [0u8; 128],
                revealed: false,
                bump: 0,
            }
        }
    }
    
    impl VoteRecord {
        pub const LEN: usize = 8 + // discriminator
            32 + // voter
            32 + // proposal
            8 +  // vote_weight
            1 +  // vote_choice
            8 +  // voted_at
            1 +  // is_private
            32 + // encrypted_choice
            32 + // encrypted_weight
            128 + // zk_proof
            1 +  // revealed
            1;   // bump
    }
    
    /// ZK Private voting configuration (per proposal)
    #[account]
    #[derive(Default)]
    pub struct PrivateVotingConfig {
        /// Proposal
        pub proposal: Pubkey,
        /// Whether private voting is enabled
        pub is_enabled: bool,
        /// Threshold encryption public key
        pub encryption_pubkey: Pubkey,
        /// Decryption threshold (e.g., 3-of-5)
        pub decryption_threshold: u8,
        /// Decryption committee (max 5)
        pub decryption_committee: [Pubkey; 5],
        /// Committee size
        pub committee_size: u8,
        /// Decryption shares received
        pub shares_received: u8,
        /// Whether reveal has started
        pub reveal_started: bool,
        /// Whether reveal is complete
        pub reveal_completed: bool,
        /// Aggregated votes for (revealed)
        pub aggregated_for: u128,
        /// Aggregated votes against
        pub aggregated_against: u128,
        /// Aggregated abstain
        pub aggregated_abstain: u128,
        /// PDA bump
        pub bump: u8,
    }
    
    impl PrivateVotingConfig {
        pub const LEN: usize = 8 + // discriminator
            32 + // proposal
            1 +  // is_enabled
            32 + // encryption_pubkey
            1 +  // decryption_threshold
            (32 * 5) + // decryption_committee
            1 +  // committee_size
            1 +  // shares_received
            1 +  // reveal_started
            1 +  // reveal_completed
            16 + // aggregated_for
            16 + // aggregated_against
            16 + // aggregated_abstain
            1;   // bump
    }
    
    /// Delegation account (PDA per delegator)
    #[account]
    #[derive(Default)]
    pub struct Delegation {
        /// Who is delegating
        pub delegator: Pubkey,
        /// Who receives voting power
        pub delegate: Pubkey,
        /// Delegation type
        pub delegation_type: u8,
        /// Category bitmap (for PerCategory)
        pub categories: u8,
        /// Amount of veVCoin delegated
        pub delegated_amount: u64,
        /// When delegated
        pub delegated_at: i64,
        /// Expiration (0 = never)
        pub expires_at: i64,
        /// Whether revocable mid-vote
        pub revocable: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl Delegation {
        pub const LEN: usize = 8 + // discriminator
            32 + // delegator
            32 + // delegate
            1 +  // delegation_type
            1 +  // categories
            8 +  // delegated_amount
            8 +  // delegated_at
            8 +  // expires_at
            1 +  // revocable
            1;   // bump
    }
    
    /// Delegate statistics
    #[account]
    #[derive(Default)]
    pub struct DelegateStats {
        /// Delegate wallet
        pub delegate: Pubkey,
        /// League tier (0=Bronze, 1=Silver, 2=Gold, 3=Diamond)
        pub league_tier: u8,
        /// Total proposals voted on
        pub total_proposals_voted: u32,
        /// Proposals voted with winning outcome
        pub proposals_with_outcome: u32,
        /// Voting accuracy (0-10000)
        pub voting_accuracy: u16,
        /// Participation rate (0-10000)
        pub participation_rate: u16,
        /// Number of unique delegators
        pub unique_delegators: u32,
        /// Total veVCoin delegated to this delegate
        pub total_delegated_vevcoin: u64,
        /// Delegator satisfaction score (0-10000)
        pub delegator_satisfaction: u16,
        /// Last vote timestamp
        pub last_vote_at: i64,
        /// Tier last updated
        pub tier_updated_at: i64,
        /// Whether eligible for promotion
        pub promotion_eligible: bool,
        /// Whether warned about demotion
        pub demotion_warning: bool,
        /// PDA bump
        pub bump: u8,
    }
    
    impl DelegateStats {
        pub const LEN: usize = 8 + // discriminator
            32 + // delegate
            1 +  // league_tier
            4 +  // total_proposals_voted
            4 +  // proposals_with_outcome
            2 +  // voting_accuracy
            2 +  // participation_rate
            4 +  // unique_delegators
            8 +  // total_delegated_vevcoin
            2 +  // delegator_satisfaction
            8 +  // last_vote_at
            8 +  // tier_updated_at
            1 +  // promotion_eligible
            1 +  // demotion_warning
            1;   // bump
        
        /// Get max delegation percent based on tier
        pub fn max_delegation_pct(&self) -> u16 {
            match self.league_tier {
                0 => 100,  // 1% Bronze
                1 => 300,  // 3% Silver
                2 => 500,  // 5% Gold
                3 => 1000, // 10% Diamond
                _ => 100,
            }
        }
    }
    
    /// Helper to calculate voting power
    pub fn calculate_voting_power(
        vevcoin_balance: u64,
        five_a_score: u16,  // 0-10000
        tier: u8,
    ) -> u64 {
        // Step 1: Quadratic base votes
        let base_votes = integer_sqrt(vevcoin_balance);
        
        // Step 2: 5A boost (1.0x to 2.0x)
        let five_a_boost = 1000 + (five_a_score as u64 / 10); // 1000-2000
        
        // Step 3: Tier multiplier
        let tier_mult = match tier {
            0 => TIER_MULT_NONE,
            1 => TIER_MULT_BRONZE,
            2 => TIER_MULT_SILVER,
            3 => TIER_MULT_GOLD,
            4 => TIER_MULT_PLATINUM,
            _ => TIER_MULT_NONE,
        };
        
        // Step 4: Combined (divide by 1_000_000 to normalize)
        let raw_votes = (base_votes * five_a_boost * tier_mult) / 1_000_000;
        
        // Step 5: Diminishing returns for extreme concentration
        if raw_votes > DIMINISHING_THRESHOLD {
            DIMINISHING_THRESHOLD + integer_sqrt(raw_votes - DIMINISHING_THRESHOLD)
        } else {
            raw_votes
        }
    }
    
    /// Integer square root using Newton's method
    pub fn integer_sqrt(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut x = n;
        let mut y = (x + 1) / 2;
        while y < x {
            x = y;
            y = (x + n / x) / 2;
        }
        x
    }
}

pub mod events {
    use super::*;
    
    #[event]
    pub struct ProposalCreated {
        pub id: u64,
        pub proposer: Pubkey,
        pub proposal_type: u8,
        pub start_time: i64,
        pub end_time: i64,
    }
    
    #[event]
    pub struct VoteCast {
        pub proposal_id: u64,
        pub voter: Pubkey,
        pub choice: u8,
        pub weight: u64,
        pub is_private: bool,
    }
    
    #[event]
    pub struct ProposalExecuted {
        pub id: u64,
        pub executor: Pubkey,
        pub timestamp: i64,
    }
    
    #[event]
    pub struct DelegationCreated {
        pub delegator: Pubkey,
        pub delegate: Pubkey,
        pub amount: u64,
        pub delegation_type: u8,
    }
    
    #[event]
    pub struct ZKRevealComplete {
        pub proposal_id: u64,
        pub votes_for: u128,
        pub votes_against: u128,
        pub votes_abstain: u128,
    }
}

use constants::*;
use errors::*;
use state::*;
use events::*;

#[program]
pub mod governance_protocol {
    use super::*;

    /// Initialize governance protocol
    pub fn initialize(
        ctx: Context<Initialize>,
        staking_program: Pubkey,
        five_a_program: Pubkey,
    ) -> Result<()> {
        let config = &mut ctx.accounts.governance_config;
        
        config.authority = ctx.accounts.authority.key();
        config.staking_program = staking_program;
        config.five_a_program = five_a_program;
        config.proposal_threshold = DEFAULT_PROPOSAL_THRESHOLD;
        config.quorum = DEFAULT_QUORUM;
        config.voting_period = DEFAULT_VOTING_PERIOD;
        config.timelock_delay = DEFAULT_TIMELOCK_DELAY;
        config.proposal_count = 0;
        config.treasury_balance = 200_000_000 * 1_000_000_000; // 200M VCoin
        config.paused = false;
        config.bump = ctx.bumps.governance_config;
        
        msg!("Governance protocol initialized");
        Ok(())
    }
    
    /// Create a new proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>,
        title_hash: [u8; 32],
        description_uri: String,
        proposal_type: u8,
        enable_private_voting: bool,
    ) -> Result<()> {
        let config = &mut ctx.accounts.governance_config;
        require!(!config.paused, GovernanceError::GovernancePaused);
        require!(description_uri.len() <= 128, GovernanceError::Overflow);
        
        // Verify proposer has enough veVCoin
        // In production, this would CPI to veVCoin program
        // For now, we accept the vevcoin_balance parameter
        
        let clock = Clock::get()?;
        
        // Increment proposal count
        config.proposal_count = config.proposal_count.saturating_add(1);
        
        let proposal = &mut ctx.accounts.proposal;
        proposal.id = config.proposal_count;
        proposal.proposer = ctx.accounts.proposer.key();
        proposal.title_hash = title_hash;
        
        let uri_bytes = description_uri.as_bytes();
        proposal.description_uri[..uri_bytes.len()].copy_from_slice(uri_bytes);
        proposal.uri_len = uri_bytes.len() as u8;
        
        proposal.proposal_type = proposal_type;
        proposal.start_time = clock.unix_timestamp;
        proposal.end_time = clock.unix_timestamp + config.voting_period;
        proposal.votes_for = 0;
        proposal.votes_against = 0;
        proposal.votes_abstain = 0;
        proposal.status = ProposalStatus::Active as u8;
        proposal.execution_time = 0;
        proposal.executed = false;
        proposal.is_private_voting = enable_private_voting;
        proposal.bump = ctx.bumps.proposal;
        
        emit!(ProposalCreated {
            id: proposal.id,
            proposer: proposal.proposer,
            proposal_type,
            start_time: proposal.start_time,
            end_time: proposal.end_time,
        });
        
        msg!("Proposal created: {}", proposal.id);
        Ok(())
    }
    
    /// Cast a public vote
    pub fn cast_vote(
        ctx: Context<CastVote>,
        choice: u8,
        vevcoin_balance: u64,
        five_a_score: u16,
        tier: u8,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        // Verify voting period
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= proposal.start_time,
            GovernanceError::VotingNotStarted
        );
        require!(
            clock.unix_timestamp <= proposal.end_time,
            GovernanceError::VotingEnded
        );
        require!(
            !proposal.is_private_voting,
            GovernanceError::ZKVotingNotEnabled
        );
        
        let vote_choice = VoteChoice::from_u8(choice)
            .ok_or(GovernanceError::InvalidVoteChoice)?;
        
        // Calculate voting power
        let vote_weight = calculate_voting_power(vevcoin_balance, five_a_score, tier);
        
        // Record vote
        let vote_record = &mut ctx.accounts.vote_record;
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.proposal = proposal.key();
        vote_record.vote_weight = vote_weight;
        vote_record.vote_choice = choice;
        vote_record.voted_at = clock.unix_timestamp;
        vote_record.is_private = false;
        vote_record.revealed = true;
        vote_record.bump = ctx.bumps.vote_record;
        
        // Update proposal vote counts
        match vote_choice {
            VoteChoice::For => {
                proposal.votes_for = proposal.votes_for.saturating_add(vote_weight as u128);
            }
            VoteChoice::Against => {
                proposal.votes_against = proposal.votes_against.saturating_add(vote_weight as u128);
            }
            VoteChoice::Abstain => {
                proposal.votes_abstain = proposal.votes_abstain.saturating_add(vote_weight as u128);
            }
        }
        
        emit!(VoteCast {
            proposal_id: proposal.id,
            voter: vote_record.voter,
            choice,
            weight: vote_weight,
            is_private: false,
        });
        
        msg!("Vote cast: {} with weight {}", choice, vote_weight);
        Ok(())
    }
    
    /// Cast a ZK private vote
    pub fn cast_private_vote(
        ctx: Context<CastPrivateVote>,
        encrypted_choice: [u8; 32],
        encrypted_weight: [u8; 32],
        zk_proof: [u8; 128],
    ) -> Result<()> {
        let proposal = &ctx.accounts.proposal;
        let private_config = &ctx.accounts.private_voting_config;
        
        require!(proposal.is_private_voting, GovernanceError::ZKVotingNotEnabled);
        require!(private_config.is_enabled, GovernanceError::ZKVotingNotEnabled);
        
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= proposal.start_time,
            GovernanceError::VotingNotStarted
        );
        require!(
            clock.unix_timestamp <= proposal.end_time,
            GovernanceError::VotingEnded
        );
        
        // In production, verify ZK proof here
        // For now, we accept the encrypted values
        
        // Record private vote
        let vote_record = &mut ctx.accounts.vote_record;
        vote_record.voter = ctx.accounts.voter.key();
        vote_record.proposal = proposal.key();
        vote_record.vote_weight = 0; // Hidden until reveal
        vote_record.vote_choice = 0; // Hidden until reveal
        vote_record.voted_at = clock.unix_timestamp;
        vote_record.is_private = true;
        vote_record.encrypted_choice = encrypted_choice;
        vote_record.encrypted_weight = encrypted_weight;
        vote_record.zk_proof = zk_proof;
        vote_record.revealed = false;
        vote_record.bump = ctx.bumps.vote_record;
        
        emit!(VoteCast {
            proposal_id: proposal.id,
            voter: vote_record.voter,
            choice: 0, // Hidden
            weight: 0, // Hidden
            is_private: true,
        });
        
        msg!("Private vote cast");
        Ok(())
    }
    
    /// Enable ZK private voting for a proposal
    pub fn enable_private_voting(
        ctx: Context<EnablePrivateVoting>,
        encryption_pubkey: Pubkey,
        decryption_committee: [Pubkey; 5],
        committee_size: u8,
        decryption_threshold: u8,
    ) -> Result<()> {
        let private_config = &mut ctx.accounts.private_voting_config;
        
        private_config.proposal = ctx.accounts.proposal.key();
        private_config.is_enabled = true;
        private_config.encryption_pubkey = encryption_pubkey;
        private_config.decryption_committee = decryption_committee;
        private_config.committee_size = committee_size;
        private_config.decryption_threshold = decryption_threshold;
        private_config.shares_received = 0;
        private_config.reveal_started = false;
        private_config.reveal_completed = false;
        private_config.aggregated_for = 0;
        private_config.aggregated_against = 0;
        private_config.aggregated_abstain = 0;
        private_config.bump = ctx.bumps.private_voting_config;
        
        msg!("Private voting enabled with {}-of-{} threshold", 
            decryption_threshold, committee_size);
        Ok(())
    }
    
    /// Initiate ZK vote reveal after voting ends
    pub fn initiate_reveal(ctx: Context<InitiateReveal>) -> Result<()> {
        let proposal = &ctx.accounts.proposal;
        let private_config = &mut ctx.accounts.private_voting_config;
        
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp > proposal.end_time,
            GovernanceError::VotingNotEnded
        );
        require!(!private_config.reveal_started, GovernanceError::RevealAlreadyComplete);
        
        private_config.reveal_started = true;
        
        msg!("ZK reveal initiated for proposal {}", proposal.id);
        Ok(())
    }
    
    /// Submit decryption share (committee member)
    pub fn submit_decryption_share(
        ctx: Context<SubmitDecryptionShare>,
        decryption_share: [u8; 32],
        committee_index: u8,
    ) -> Result<()> {
        let private_config = &mut ctx.accounts.private_voting_config;
        
        require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
        require!(!private_config.reveal_completed, GovernanceError::RevealAlreadyComplete);
        
        // Verify committee member
        let committee_member = ctx.accounts.committee_member.key();
        require!(
            private_config.decryption_committee[committee_index as usize] == committee_member,
            GovernanceError::Unauthorized
        );
        
        // In production, process the decryption share
        // For now, just count it
        private_config.shares_received = private_config.shares_received.saturating_add(1);
        
        msg!("Decryption share {} of {} received", 
            private_config.shares_received, 
            private_config.decryption_threshold);
        Ok(())
    }
    
    /// Complete ZK reveal and aggregate votes
    pub fn aggregate_revealed_votes(
        ctx: Context<AggregateRevealedVotes>,
        aggregated_for: u128,
        aggregated_against: u128,
        aggregated_abstain: u128,
    ) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        let private_config = &mut ctx.accounts.private_voting_config;
        
        require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
        require!(
            private_config.shares_received >= private_config.decryption_threshold,
            GovernanceError::InvalidDecryptionShare
        );
        
        // Update aggregated totals
        private_config.aggregated_for = aggregated_for;
        private_config.aggregated_against = aggregated_against;
        private_config.aggregated_abstain = aggregated_abstain;
        private_config.reveal_completed = true;
        
        // Update proposal with revealed totals
        proposal.votes_for = aggregated_for;
        proposal.votes_against = aggregated_against;
        proposal.votes_abstain = aggregated_abstain;
        
        emit!(ZKRevealComplete {
            proposal_id: proposal.id,
            votes_for: aggregated_for,
            votes_against: aggregated_against,
            votes_abstain: aggregated_abstain,
        });
        
        msg!("ZK reveal complete: For={}, Against={}, Abstain={}", 
            aggregated_for, aggregated_against, aggregated_abstain);
        Ok(())
    }
    
    /// Finalize proposal (determine pass/fail)
    pub fn finalize_proposal(ctx: Context<FinalizeProposal>) -> Result<()> {
        let config = &ctx.accounts.governance_config;
        let proposal = &mut ctx.accounts.proposal;
        
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp > proposal.end_time,
            GovernanceError::VotingNotEnded
        );
        
        let total_votes = proposal.votes_for + proposal.votes_against + proposal.votes_abstain;
        
        // Check quorum
        if total_votes < config.quorum as u128 {
            proposal.status = ProposalStatus::Rejected as u8;
            msg!("Proposal rejected: quorum not reached");
            return Ok(());
        }
        
        // Determine outcome
        if proposal.votes_for > proposal.votes_against {
            proposal.status = ProposalStatus::Passed as u8;
            proposal.execution_time = clock.unix_timestamp + config.timelock_delay;
            msg!("Proposal passed, execution time: {}", proposal.execution_time);
        } else {
            proposal.status = ProposalStatus::Rejected as u8;
            msg!("Proposal rejected");
        }
        
        Ok(())
    }
    
    /// Execute a passed proposal
    pub fn execute_proposal(ctx: Context<ExecuteProposal>) -> Result<()> {
        let proposal = &mut ctx.accounts.proposal;
        
        require!(
            proposal.status == ProposalStatus::Passed as u8,
            GovernanceError::ProposalNotFound
        );
        require!(!proposal.executed, GovernanceError::ProposalAlreadyExecuted);
        
        let clock = Clock::get()?;
        require!(
            clock.unix_timestamp >= proposal.execution_time,
            GovernanceError::TimelockNotExpired
        );
        
        proposal.executed = true;
        proposal.status = ProposalStatus::Executed as u8;
        
        emit!(ProposalExecuted {
            id: proposal.id,
            executor: ctx.accounts.executor.key(),
            timestamp: clock.unix_timestamp,
        });
        
        msg!("Proposal {} executed", proposal.id);
        Ok(())
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
        let delegator_key = ctx.accounts.delegator.key();
        let delegate_key = ctx.accounts.delegate.key();
        
        require!(delegator_key != delegate_key, GovernanceError::CannotDelegateSelf);
        
        let clock = Clock::get()?;
        
        let delegation = &mut ctx.accounts.delegation;
        delegation.delegator = delegator_key;
        delegation.delegate = delegate_key;
        delegation.delegation_type = delegation_type;
        delegation.categories = categories;
        delegation.delegated_amount = vevcoin_amount;
        delegation.delegated_at = clock.unix_timestamp;
        delegation.expires_at = expires_at;
        delegation.revocable = revocable;
        delegation.bump = ctx.bumps.delegation;
        
        // Update delegate stats
        let delegate_stats = &mut ctx.accounts.delegate_stats;
        delegate_stats.delegate = delegate_key;
        delegate_stats.unique_delegators = delegate_stats.unique_delegators.saturating_add(1);
        delegate_stats.total_delegated_vevcoin = delegate_stats
            .total_delegated_vevcoin
            .saturating_add(vevcoin_amount);
        delegate_stats.bump = ctx.bumps.delegate_stats;
        
        emit!(DelegationCreated {
            delegator: delegator_key,
            delegate: delegate_key,
            amount: vevcoin_amount,
            delegation_type,
        });
        
        msg!("Delegation created: {} veVCoin", vevcoin_amount);
        Ok(())
    }
    
    /// Revoke delegation
    pub fn revoke_delegation(ctx: Context<RevokeDelegation>) -> Result<()> {
        let delegation = &ctx.accounts.delegation;
        let delegate_stats = &mut ctx.accounts.delegate_stats;
        
        // Update delegate stats
        delegate_stats.unique_delegators = delegate_stats.unique_delegators.saturating_sub(1);
        delegate_stats.total_delegated_vevcoin = delegate_stats
            .total_delegated_vevcoin
            .saturating_sub(delegation.delegated_amount);
        
        // Delegation account will be closed by Anchor's close attribute
        
        msg!("Delegation revoked");
        Ok(())
    }
    
    /// Update governance parameters
    pub fn update_config(
        ctx: Context<UpdateConfig>,
        proposal_threshold: Option<u64>,
        quorum: Option<u64>,
        voting_period: Option<i64>,
        timelock_delay: Option<i64>,
    ) -> Result<()> {
        let config = &mut ctx.accounts.governance_config;
        
        if let Some(threshold) = proposal_threshold {
            config.proposal_threshold = threshold;
        }
        if let Some(q) = quorum {
            config.quorum = q;
        }
        if let Some(period) = voting_period {
            config.voting_period = period;
        }
        if let Some(delay) = timelock_delay {
            config.timelock_delay = delay;
        }
        
        msg!("Governance config updated");
        Ok(())
    }
    
    /// Pause/unpause governance
    pub fn set_paused(ctx: Context<UpdateConfig>, paused: bool) -> Result<()> {
        ctx.accounts.governance_config.paused = paused;
        msg!("Governance paused: {}", paused);
        Ok(())
    }
    
    /// Update authority
    pub fn update_authority(ctx: Context<UpdateAuthority>, new_authority: Pubkey) -> Result<()> {
        ctx.accounts.governance_config.authority = new_authority;
        msg!("Authority updated to: {}", new_authority);
        Ok(())
    }
    
    /// Get proposal info
    pub fn get_proposal(ctx: Context<GetProposal>) -> Result<()> {
        let proposal = &ctx.accounts.proposal;
        msg!("ID: {}", proposal.id);
        msg!("Status: {}", proposal.status);
        msg!("For: {}", proposal.votes_for);
        msg!("Against: {}", proposal.votes_against);
        msg!("Abstain: {}", proposal.votes_abstain);
        Ok(())
    }
}

// Account contexts

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = GovernanceConfig::LEN,
        seeds = [GOV_CONFIG_SEED],
        bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateProposal<'info> {
    #[account(
        mut,
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(
        init,
        payer = proposer,
        space = Proposal::LEN,
        seeds = [PROPOSAL_SEED, (governance_config.proposal_count + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastVote<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = voter,
        space = VoteRecord::LEN,
        seeds = [VOTE_RECORD_SEED, proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CastPrivateVote<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    #[account(
        init,
        payer = voter,
        space = VoteRecord::LEN,
        seeds = [VOTE_RECORD_SEED, proposal.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_record: Account<'info, VoteRecord>,
    
    #[account(mut)]
    pub voter: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnablePrivateVoting<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump,
        has_one = proposer
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        init,
        payer = proposer,
        space = PrivateVotingConfig::LEN,
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    #[account(mut)]
    pub proposer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitiateReveal<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        mut,
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    pub initiator: Signer<'info>,
}

#[derive(Accounts)]
pub struct SubmitDecryptionShare<'info> {
    #[account(
        mut,
        seeds = [PRIVATE_VOTING_SEED, private_voting_config.proposal.as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    pub committee_member: Signer<'info>,
}

#[derive(Accounts)]
pub struct AggregateRevealedVotes<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    #[account(
        mut,
        seeds = [PRIVATE_VOTING_SEED, proposal.key().as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump,
        has_one = authority @ GovernanceError::Unauthorized
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct FinalizeProposal<'info> {
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub finalizer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ExecuteProposal<'info> {
    #[account(
        mut,
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
    
    pub executor: Signer<'info>,
}

#[derive(Accounts)]
pub struct DelegateVotes<'info> {
    #[account(
        init,
        payer = delegator,
        space = Delegation::LEN,
        seeds = [DELEGATION_SEED, delegator.key().as_ref()],
        bump
    )]
    pub delegation: Account<'info, Delegation>,
    
    #[account(
        init_if_needed,
        payer = delegator,
        space = DelegateStats::LEN,
        seeds = [DELEGATE_STATS_SEED, delegate.key().as_ref()],
        bump
    )]
    pub delegate_stats: Account<'info, DelegateStats>,
    
    #[account(mut)]
    pub delegator: Signer<'info>,
    
    /// CHECK: Delegate receiving voting power
    pub delegate: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RevokeDelegation<'info> {
    #[account(
        mut,
        close = delegator,
        seeds = [DELEGATION_SEED, delegator.key().as_ref()],
        bump = delegation.bump,
        has_one = delegator
    )]
    pub delegation: Account<'info, Delegation>,
    
    #[account(
        mut,
        seeds = [DELEGATE_STATS_SEED, delegation.delegate.as_ref()],
        bump = delegate_stats.bump
    )]
    pub delegate_stats: Account<'info, DelegateStats>,
    
    #[account(mut)]
    pub delegator: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump,
        has_one = authority @ GovernanceError::Unauthorized
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(
        mut,
        seeds = [GOV_CONFIG_SEED],
        bump = governance_config.bump,
        has_one = authority @ GovernanceError::Unauthorized
    )]
    pub governance_config: Account<'info, GovernanceConfig>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetProposal<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}


