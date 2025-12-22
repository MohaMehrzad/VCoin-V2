use anchor_lang::prelude::*;
use crate::contexts::ExecuteProposal;
use crate::errors::GovernanceError;
use crate::events::ProposalExecuted;
use crate::state::ProposalStatus;

pub fn handler(ctx: Context<ExecuteProposal>) -> Result<()> {
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

