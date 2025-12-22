use anchor_lang::prelude::*;

use crate::constants::{FIVE_A_CONFIG_SEED, USER_SCORE_SEED, VOUCH_RECORD_SEED, VOUCH_STATUS_SEED, VOUCHER_STATS_SEED};
use crate::state::{FiveAConfig, UserScore, VouchRecord, UserVouchStatus, VoucherStats};

#[derive(Accounts)]
pub struct VouchForUser<'info> {
    #[account(
        seeds = [FIVE_A_CONFIG_SEED],
        bump = five_a_config.bump
    )]
    pub five_a_config: Account<'info, FiveAConfig>,
    
    #[account(
        seeds = [USER_SCORE_SEED, voucher.key().as_ref()],
        bump = voucher_score.bump
    )]
    pub voucher_score: Account<'info, UserScore>,
    
    #[account(
        init,
        payer = voucher,
        space = VouchRecord::LEN,
        seeds = [VOUCH_RECORD_SEED, voucher.key().as_ref(), vouchee.key().as_ref()],
        bump
    )]
    pub vouch_record: Account<'info, VouchRecord>,
    
    #[account(
        init_if_needed,
        payer = voucher,
        space = UserVouchStatus::LEN,
        seeds = [VOUCH_STATUS_SEED, vouchee.key().as_ref()],
        bump
    )]
    pub vouchee_status: Account<'info, UserVouchStatus>,
    
    #[account(
        init_if_needed,
        payer = voucher,
        space = VoucherStats::LEN,
        seeds = [VOUCHER_STATS_SEED, voucher.key().as_ref()],
        bump
    )]
    pub voucher_stats: Account<'info, VoucherStats>,
    
    #[account(mut)]
    pub voucher: Signer<'info>,
    
    /// CHECK: User receiving vouch
    pub vouchee: UncheckedAccount<'info>,
    
    pub system_program: Program<'info, System>,
}

