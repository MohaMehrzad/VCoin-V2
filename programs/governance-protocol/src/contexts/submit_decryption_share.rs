use anchor_lang::prelude::*;
use crate::constants::{PRIVATE_VOTING_SEED, DECRYPTION_SHARE_SEED};
use crate::state::{PrivateVotingConfig, DecryptionShare};

#[derive(Accounts)]
#[instruction(decryption_share: [u8; 32], committee_index: u8)]
pub struct SubmitDecryptionShare<'info> {
    #[account(
        mut,
        seeds = [PRIVATE_VOTING_SEED, private_voting_config.proposal.as_ref()],
        bump = private_voting_config.bump
    )]
    pub private_voting_config: Account<'info, PrivateVotingConfig>,
    
    /// The decryption share account to store the share on-chain (C-02 fix)
    #[account(
        init,
        payer = committee_member,
        space = DecryptionShare::LEN,
        seeds = [DECRYPTION_SHARE_SEED, private_voting_config.proposal.as_ref(), &[committee_index]],
        bump
    )]
    pub decryption_share: Account<'info, DecryptionShare>,
    
    #[account(mut)]
    pub committee_member: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

