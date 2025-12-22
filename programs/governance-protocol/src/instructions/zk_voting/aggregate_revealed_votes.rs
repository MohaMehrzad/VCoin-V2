use anchor_lang::prelude::*;
use crate::constants::ZK_VOTING_ENABLED;
use crate::contexts::AggregateRevealedVotes;
use crate::errors::GovernanceError;
use crate::events::ZKRevealComplete;

/// Aggregate revealed votes from decryption shares
/// 
/// CRITICAL SECURITY WARNING (C-03):
/// This function currently accepts aggregated vote counts as parameters,
/// which is insecure because a malicious caller could provide fabricated values.
/// 
/// Until proper on-chain threshold decryption is implemented, this function
/// is blocked by the ZK_VOTING_ENABLED flag.
pub fn handler(
    ctx: Context<AggregateRevealedVotes>,
    aggregated_for: u128,
    aggregated_against: u128,
    aggregated_abstain: u128,
) -> Result<()> {
    // === CRITICAL FIX C-03: Block until on-chain computation implemented ===
    // Accepting aggregated values as parameters allows vote manipulation.
    // This function should compute votes from stored DecryptionShare accounts.
    require!(ZK_VOTING_ENABLED, GovernanceError::ZKVotingNotEnabled);
    // === END CRITICAL FIX ===
    
    let proposal = &mut ctx.accounts.proposal;
    let private_config = &mut ctx.accounts.private_voting_config;
    
    require!(private_config.reveal_started, GovernanceError::RevealNotStarted);
    require!(
        private_config.shares_received >= private_config.decryption_threshold,
        GovernanceError::InvalidDecryptionShare
    );
    
    // TODO: When ZK_VOTING_ENABLED is true, this should:
    // 1. Load DecryptionShare accounts from remaining_accounts
    // 2. Verify each share is valid for this proposal
    // 3. Perform threshold decryption using Shamir's Secret Sharing
    // 4. Compute aggregated votes on-chain (not from parameters)
    
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

