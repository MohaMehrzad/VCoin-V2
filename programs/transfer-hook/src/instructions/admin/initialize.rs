use anchor_lang::prelude::*;

use crate::contexts::Initialize;

/// Initialize the transfer hook configuration
pub fn handler(
    ctx: Context<Initialize>,
    five_a_program: Pubkey,
    min_activity_amount: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.hook_config;
    
    config.authority = ctx.accounts.authority.key();
    config.vcoin_mint = ctx.accounts.vcoin_mint.key();
    config.five_a_program = five_a_program;
    config.block_wash_trading = false; // Start with flagging only
    config.min_activity_amount = min_activity_amount;
    config.total_transfers = 0;
    config.wash_trading_flags = 0;
    config.paused = false;
    config.bump = ctx.bumps.hook_config;
    
    msg!("Transfer hook initialized for VCoin mint: {}", config.vcoin_mint);
    Ok(())
}

