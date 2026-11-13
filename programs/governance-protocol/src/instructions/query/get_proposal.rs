use crate::contexts::GetProposal;
use anchor_lang::prelude::*;

pub fn handler(ctx: Context<GetProposal>) -> Result<()> {
    let proposal = &ctx.accounts.proposal;
    msg!("ID: {}", proposal.id);
    msg!("Status: {}", proposal.status);
    msg!("For: {}", proposal.votes_for);
    msg!("Against: {}", proposal.votes_against);
    msg!("Abstain: {}", proposal.votes_abstain);
    Ok(())
}
