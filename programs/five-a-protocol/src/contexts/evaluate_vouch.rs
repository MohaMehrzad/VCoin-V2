use anchor_lang::prelude::*;

use crate::constants::{VOUCH_RECORD_SEED, USER_SCORE_SEED, VOUCHER_STATS_SEED};
use crate::state::{VouchRecord, UserScore, VoucherStats};

#[derive(Accounts)]
pub struct EvaluateVouch<'info> {
    #[account(
        mut,
        seeds = [VOUCH_RECORD_SEED, vouch_record.voucher.as_ref(), vouch_record.vouchee.as_ref()],
        bump = vouch_record.bump
    )]
    pub vouch_record: Account<'info, VouchRecord>,
    
    #[account(
        seeds = [USER_SCORE_SEED, vouch_record.vouchee.as_ref()],
        bump = vouchee_score.bump
    )]
    pub vouchee_score: Account<'info, UserScore>,
    
    #[account(
        mut,
        seeds = [VOUCHER_STATS_SEED, vouch_record.voucher.as_ref()],
        bump = voucher_stats.bump
    )]
    pub voucher_stats: Account<'info, VoucherStats>,
    
    pub evaluator: Signer<'info>,
}

