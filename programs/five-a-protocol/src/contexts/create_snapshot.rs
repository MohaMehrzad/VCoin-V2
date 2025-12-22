use anchor_lang::prelude::*;

use crate::constants::{FIVE_A_CONFIG_SEED, SCORE_SNAPSHOT_SEED};
use crate::state::{FiveAConfig, ScoreSnapshot};

#[derive(Accounts)]
pub struct CreateSnapshot<'info> {
    #[account(
        mut,
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        init,
        payer = oracle,
        space = ScoreSnapshot::LEN,
        seeds = [SCORE_SNAPSHOT_SEED, (five_a_config.current_epoch + 1).to_le_bytes().as_ref()],
        bump
    )]
    pub snapshot: Account<'info, ScoreSnapshot>,
    
    #[account(mut)]
    pub oracle: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

