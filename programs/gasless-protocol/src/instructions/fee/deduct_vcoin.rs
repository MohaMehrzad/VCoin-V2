use anchor_lang::prelude::*;
use anchor_spl::token_2022;
use crate::contexts::DeductVCoinFee;
use crate::errors::GaslessError;
use crate::events::FeeCollected;

pub fn handler(ctx: Context<DeductVCoinFee>, amount: u64) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let user_stats = &mut ctx.accounts.user_stats;
    
    require!(!config.paused, GaslessError::ProtocolPaused);
    
    let clock = Clock::get()?;
    
    // Calculate VCoin fee equivalent
    let vcoin_fee = config.sol_fee_per_tx
        .saturating_mul(config.vcoin_fee_multiplier);
    
    let fee_to_deduct = if amount > 0 { amount } else { vcoin_fee };
    
    // Transfer VCoin from user to fee vault
    token_2022::transfer_checked(
        CpiContext::new(ctx.accounts.token_program.to_account_info(),
            token_2022::TransferChecked {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.fee_vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
                mint: ctx.accounts.vcoin_mint.to_account_info(),
            },
        ),
        fee_to_deduct,
        ctx.accounts.vcoin_mint.decimals,
    )?;
    
    // Update stats
    config.total_vcoin_collected = config.total_vcoin_collected.saturating_add(fee_to_deduct);
    user_stats.total_vcoin_fees = user_stats.total_vcoin_fees.saturating_add(fee_to_deduct);
    user_stats.total_gasless_tx = user_stats.total_gasless_tx.saturating_add(1);
    user_stats.last_gasless_at = clock.unix_timestamp;
    user_stats.bump = ctx.bumps.user_stats;
    
    emit!(FeeCollected {
        user: ctx.accounts.user.key(),
        fee_method: 1,
        amount: fee_to_deduct,
        is_vcoin: true,
    });
    
    msg!("VCoin fee deducted: {} VCoin", fee_to_deduct);
    Ok(())
}

