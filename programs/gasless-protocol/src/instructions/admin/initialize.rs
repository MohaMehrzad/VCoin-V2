use anchor_lang::prelude::*;
use crate::constants::*;
use crate::contexts::Initialize;
use crate::events::GaslessConfigInitialized;
use crate::state::GaslessConfig;

pub fn handler(
    ctx: Context<Initialize>,
    fee_payer: Pubkey,
    daily_budget: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    config.authority = ctx.accounts.authority.key();
    config.fee_payer = fee_payer;
    config.vcoin_mint = ctx.accounts.vcoin_mint.key();
    config.fee_vault = ctx.accounts.fee_vault.key();
    config.sscre_program = Pubkey::default();
    config.daily_subsidy_budget = daily_budget;
    config.sol_fee_per_tx = DEFAULT_SOL_FEE;
    config.vcoin_fee_multiplier = VCOIN_FEE_MULTIPLIER;
    config.sscre_deduction_bps = SSCRE_DEDUCTION_BPS;
    config.max_subsidized_per_user = MAX_SUBSIDIZED_TX_PER_USER;
    config.total_subsidized_tx = 0;
    config.total_sol_spent = 0;
    config.total_vcoin_collected = 0;
    config.paused = false;
    config.current_day = GaslessConfig::get_day_number(Clock::get()?.unix_timestamp);
    config.day_spent = 0;
    config.bump = ctx.bumps.config;
    
    emit!(GaslessConfigInitialized {
        authority: config.authority,
        fee_payer,
        daily_budget,
    });
    
    msg!("Gasless Protocol initialized with {} lamports daily budget", daily_budget);
    Ok(())
}

