use anchor_lang::prelude::*;
use crate::constants::PRIVATE_VOTING_SEED;
use crate::state::PrivateVotingConfig;

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

