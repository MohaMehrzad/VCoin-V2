use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Proposal, PrivateVotingConfig, VoteRecord, GovernanceConfig};

#[derive(Accounts)]
pub struct CastPrivateVote<'info> {
    /// GovernanceConfig for staking_program and five_a_program addresses
    #[account(
        seeds = [GOV_CONFIG_SEED],
        bump = config.bump
    )]
    pub config: Account<'info, GovernanceConfig>,

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

    /// UserStake account from staking-protocol for on-chain tier and veVCoin verification
    /// CHECK: Verified in handler via PDA derivation from staking_program
    #[account()]
    pub user_stake: AccountInfo<'info>,

    /// UserScore account from five-a-protocol for on-chain 5A score verification
    /// CHECK: Verified in handler via PDA derivation from five_a_program
    #[account()]
    pub user_score: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}
