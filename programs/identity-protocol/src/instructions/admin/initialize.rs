use anchor_lang::prelude::*;

use crate::contexts::Initialize;

/// Initialize the identity protocol
pub fn handler(ctx: Context<Initialize>, sas_program: Pubkey, usdc_mint: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.identity_config;
    
    config.authority = ctx.accounts.authority.key();
    config.sas_program = sas_program;
    config.usdc_mint = usdc_mint;
    config.treasury = ctx.accounts.treasury.key();
    config.trusted_attesters = [Pubkey::default(); 10];
    config.attester_count = 0;
    config.total_identities = 0;
    config.verified_identities = 0;
    config.paused = false;
    config.bump = ctx.bumps.identity_config;
    
    msg!("Identity protocol initialized");
    Ok(())
}

