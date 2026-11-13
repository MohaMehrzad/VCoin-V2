use anchor_lang::prelude::*;

use crate::constants::*;
use crate::errors::FiveAError;
use crate::state::*;

/// Upgrade authority only (audit 5A-02).
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init, payer = authority, space = FiveAConfig::LEN, seeds = [FIVE_A_CONFIG_SEED], bump)]
    pub config: Box<Account<'info, FiveAConfig>>,
    #[account(constraint = program.programdata_address()? == Some(program_data.key()) @ FiveAError::InvalidProgramData)]
    pub program: Program<'info, crate::program::FiveAProtocol>,
    pub program_data: Account<'info, ProgramData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(mut, seeds = [FIVE_A_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, FiveAConfig>>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct AcceptAuthority<'info> {
    #[account(mut, seeds = [FIVE_A_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, FiveAConfig>>,
    pub new_authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct SubmitScore<'info> {
    #[account(seeds = [FIVE_A_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, FiveAConfig>>,
    #[account(mut)]
    pub oracle: Signer<'info>,
    /// A wallet (system-owned, possibly unfunded).
    pub user: SystemAccount<'info>,
    #[account(
        init_if_needed,
        payer = oracle,
        space = PendingScoreUpdate::LEN,
        seeds = [PENDING_SCORE_SEED, user.key().as_ref()],
        bump,
    )]
    pub pending: Box<Account<'info, PendingScoreUpdate>>,
    /// CHECK: the round's opener (the signing oracle for a new round); refunded when the round
    /// closes. Checked against `pending.rent_payer` in the handler.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = oracle,
        space = UserScore::LEN,
        seeds = [USER_SCORE_SEED, user.key().as_ref()],
        bump,
    )]
    pub user_score: Box<Account<'info, UserScore>>,
    pub system_program: Program<'info, System>,
}

/// Anyone may close an expired round; rent returns to its opener.
#[derive(Accounts)]
pub struct CloseExpiredRound<'info> {
    #[account(
        mut,
        seeds = [PENDING_SCORE_SEED, pending.user.as_ref()],
        bump = pending.bump,
        has_one = rent_payer,
        close = rent_payer,
    )]
    pub pending: Box<Account<'info, PendingScoreUpdate>>,
    /// CHECK: bound by `has_one`.
    #[account(mut)]
    pub rent_payer: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct VouchForUser<'info> {
    #[account(seeds = [FIVE_A_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, FiveAConfig>>,
    #[account(mut)]
    pub voucher: Signer<'info>,
    #[account(seeds = [USER_SCORE_SEED, voucher.key().as_ref()], bump = voucher_score.bump)]
    pub voucher_score: Box<Account<'info, UserScore>>,
    pub vouchee: SystemAccount<'info>,
    /// CHECK: the vouchee's UserScore PDA (may not exist); checked by `read_score`.
    pub vouchee_score: UncheckedAccount<'info>,
    #[account(
        init,
        payer = voucher,
        space = VouchRecord::LEN,
        seeds = [VOUCH_RECORD_SEED, voucher.key().as_ref(), vouchee.key().as_ref()],
        bump,
    )]
    pub vouch_record: Box<Account<'info, VouchRecord>>,
    /// CHECK: the reverse vouch PDA; must not exist (no direct mutual vouching).
    #[account(seeds = [VOUCH_RECORD_SEED, vouchee.key().as_ref(), voucher.key().as_ref()], bump)]
    pub reverse_vouch: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = voucher,
        space = VoucherStats::LEN,
        seeds = [VOUCHER_STATS_SEED, voucher.key().as_ref()],
        bump,
    )]
    pub voucher_stats: Box<Account<'info, VoucherStats>>,
    #[account(
        init_if_needed,
        payer = voucher,
        space = UserVouchStatus::LEN,
        seeds = [VOUCH_STATUS_SEED, vouchee.key().as_ref()],
        bump,
    )]
    pub vouchee_status: Box<Account<'info, UserVouchStatus>>,
    pub system_program: Program<'info, System>,
}

/// Closes a vouch record (rent to the voucher) and updates the voucher's stats.
#[derive(Accounts)]
pub struct CloseVouch<'info> {
    #[account(seeds = [FIVE_A_CONFIG_SEED], bump = config.bump)]
    pub config: Box<Account<'info, FiveAConfig>>,
    #[account(
        mut,
        seeds = [VOUCH_RECORD_SEED, vouch_record.voucher.as_ref(), vouch_record.vouchee.as_ref()],
        bump = vouch_record.bump,
        has_one = voucher,
        close = voucher,
    )]
    pub vouch_record: Box<Account<'info, VouchRecord>>,
    /// CHECK: bound by `has_one`; receives the record's rent.
    #[account(mut)]
    pub voucher: UncheckedAccount<'info>,
    #[account(mut, seeds = [VOUCHER_STATS_SEED, vouch_record.voucher.as_ref()], bump = voucher_stats.bump)]
    pub voucher_stats: Box<Account<'info, VoucherStats>>,
    #[account(mut, seeds = [VOUCH_STATUS_SEED, vouch_record.vouchee.as_ref()], bump = vouchee_status.bump)]
    pub vouchee_status: Box<Account<'info, UserVouchStatus>>,
}

/// The vouchee declines a vouch made for them.
#[derive(Accounts)]
pub struct DeclineVouch<'info> {
    pub close: CloseVouch<'info>,
    #[account(address = close.vouch_record.vouchee @ FiveAError::NotVouchee)]
    pub vouchee: Signer<'info>,
}

/// Oracle-only evaluation at or after the 90-day point (audit 5A-05).
#[derive(Accounts)]
pub struct EvaluateVouch<'info> {
    pub close: CloseVouch<'info>,
    pub oracle: Signer<'info>,
    /// CHECK: the vouchee's UserScore PDA (missing = failed); checked by `read_score`.
    pub vouchee_score: UncheckedAccount<'info>,
}
