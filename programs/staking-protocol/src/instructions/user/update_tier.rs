use anchor_lang::prelude::*;

use crate::constants::STAKING_POOL_SEED;
use crate::contexts::UpdateTier;
use crate::errors::StakingError;
use crate::events::TierUpdated;
use crate::state::StakingTier;
use crate::utils::calculate_vevcoin;

/// Update user's tier based on current stake
/// C-04 Security Fix: Added veVCoin CPI minting
pub fn handler(ctx: Context<UpdateTier>) -> Result<()> {
    let user_stake = &ctx.accounts.user_stake;
    
    require!(user_stake.staked_amount > 0, StakingError::NoActiveStake);
    
    let new_tier = StakingTier::from_amount(user_stake.staked_amount);
    let old_tier = user_stake.tier;
    let old_vevcoin = user_stake.ve_vcoin_amount;
    
    // Recalculate veVCoin with new tier
    let new_vevcoin = calculate_vevcoin(
        user_stake.staked_amount,
        user_stake.lock_duration,
        new_tier,
    )?;
    let vevcoin_to_mint = new_vevcoin.checked_sub(old_vevcoin).unwrap_or(0);
    
    // Capture pool bump before CPI
    let pool_bump = ctx.bumps.pool;
    
    // === CRITICAL FIX C-04: Mint veVCoin via CPI ===
    if vevcoin_to_mint > 0 {
        let seeds = &[STAKING_POOL_SEED, &[pool_bump]];
        let signer_seeds = &[&seeds[..]];
        
        vevcoin_token::cpi::mint_vevcoin(
            CpiContext::new_with_signer(
                ctx.accounts.vevcoin_program.to_account_info(),
                vevcoin_token::cpi::accounts::MintVeVCoin {
                    staking_protocol: ctx.accounts.pool.to_account_info(),
                    user: ctx.accounts.user.to_account_info(),
                    config: ctx.accounts.vevcoin_config.to_account_info(),
                    user_account: ctx.accounts.user_vevcoin.to_account_info(),
                    mint: ctx.accounts.vevcoin_mint.to_account_info(),
                    user_token_account: ctx.accounts.user_vevcoin_account.to_account_info(),
                    payer: ctx.accounts.user.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                signer_seeds,
            ),
            vevcoin_to_mint,
        )?;
    }
    // === END CRITICAL FIX ===
    
    // Update stake
    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.tier = new_tier.as_u8();
    user_stake.ve_vcoin_amount = new_vevcoin;
    
    let clock = Clock::get()?;
    
    // L-01: Emit tier update event
    emit!(TierUpdated {
        user: ctx.accounts.user.key(),
        old_tier,
        new_tier: new_tier.as_u8(),
        timestamp: clock.unix_timestamp,
    });
    
    msg!("Tier updated from {} to {}", old_tier, new_tier.as_u8());
    msg!("veVCoin updated to: {}", new_vevcoin);
    msg!("Additional veVCoin minted: {}", vevcoin_to_mint);
    
    Ok(())
}
