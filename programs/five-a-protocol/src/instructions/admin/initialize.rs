use anchor_lang::prelude::*;

use crate::contexts::Initialize;

/// Initialize the 5A protocol
pub fn handler(ctx: Context<Initialize>, identity_program: Pubkey, vcoin_mint: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.five_a_config;
    
    config.authority = ctx.accounts.authority.key();
    config.identity_program = identity_program;
    config.vcoin_mint = vcoin_mint;
    config.vouch_vault = ctx.accounts.vouch_vault.key();
    config.oracles = [Pubkey::default(); 10];
    config.oracle_count = 0;
    config.required_consensus = 1;
    config.total_users = 0;
    config.current_epoch = 0;
    config.last_snapshot_time = 0;
    config.paused = false;
    config.bump = ctx.bumps.five_a_config;
    
    msg!("5A Protocol initialized");
    Ok(())
}

