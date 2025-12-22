use anchor_lang::prelude::*;

use crate::constants::PAIR_TRACKING_SEED;
use crate::state::PairTracking;

#[derive(Accounts)]
pub struct GetPairStats<'info> {
    #[account(
        seeds = [PAIR_TRACKING_SEED, sender.key().as_ref(), receiver.key().as_ref()],
        bump = pair_tracking.bump
    )]
    pub pair_tracking: Account<'info, PairTracking>,
    
    /// CHECK: Just used for PDA derivation
    pub sender: UncheckedAccount<'info>,
    
    /// CHECK: Just used for PDA derivation
    pub receiver: UncheckedAccount<'info>,
}

