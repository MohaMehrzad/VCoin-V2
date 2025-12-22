use anchor_lang::prelude::*;
use crate::constants::CONTENT_RECORD_SEED;
use crate::state::ContentRecord;

#[derive(Accounts)]
pub struct GetContent<'info> {
    #[account(
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump
    )]
    pub content_record: Account<'info, ContentRecord>,
}

