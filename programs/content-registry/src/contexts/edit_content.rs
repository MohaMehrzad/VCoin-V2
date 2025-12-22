use anchor_lang::prelude::*;
use crate::constants::*;
use crate::state::{ContentRecord, UserEnergy};

#[derive(Accounts)]
pub struct EditContent<'info> {
    #[account(
        mut,
        seeds = [CONTENT_RECORD_SEED, content_record.tracking_id.as_ref()],
        bump = content_record.bump,
        has_one = author
    )]
    pub content_record: Account<'info, ContentRecord>,
    
    #[account(
        mut,
        seeds = [USER_ENERGY_SEED, author.key().as_ref()],
        bump = user_energy.bump
    )]
    pub user_energy: Account<'info, UserEnergy>,
    
    pub author: Signer<'info>,
}

