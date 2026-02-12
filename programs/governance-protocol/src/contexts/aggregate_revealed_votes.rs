use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{Proposal, PrivateVotingConfig};

/// Permissionless tally aggregation — anyone can submit the tally since on-chain
/// cryptographic verification (tally * H == C_sum - D) prevents fabrication.
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
}
