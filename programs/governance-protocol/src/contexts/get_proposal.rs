use anchor_lang::prelude::*;
use crate::constants::PROPOSAL_SEED;
use crate::state::Proposal;

#[derive(Accounts)]
pub struct GetProposal<'info> {
    #[account(
        seeds = [PROPOSAL_SEED, proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
    )]
    pub proposal: Account<'info, Proposal>,
}

