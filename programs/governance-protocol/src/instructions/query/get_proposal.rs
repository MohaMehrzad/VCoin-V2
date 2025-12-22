use anchor_lang::prelude::*;
use crate::contexts::GetProposal;

pub fn handler(ctx: Context<GetProposal>) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    msg!("ID: {}", proposal.id);
    msg!("Status: {}", proposal.status);
    msg!("For: {}", proposal.votes_for);
    msg!("Against: {}", proposal.votes_against);
    msg!("Abstain: {}", proposal.votes_abstain);
    Ok(())
}

