use anchor_lang::prelude::*;
use crate::contexts::InitializeUserEnergy;
use crate::state::UserEnergy;

pub fn handler(ctx: Context<InitializeUserEnergy>, tier: u8) -> Result<()> {
    let clock = Clock::get()?;
    let user_energy = &mut ctx.accounts.user_energy;
    
    user_energy.user = ctx.accounts.user.key();
    user_energy.tier = tier;
    user_energy.max_energy = UserEnergy::max_energy_for_tier(tier);
    user_energy.regen_rate = UserEnergy::regen_rate_for_tier(tier);
    user_energy.current_energy = user_energy.max_energy; // Start full
    user_energy.last_regen_time = clock.unix_timestamp;
    user_energy.energy_spent_today = 0;
    user_energy.energy_refunded_today = 0;
    user_energy.last_reset = clock.unix_timestamp;
    user_energy.bump = ctx.bumps.user_energy;
    
    msg!("User energy initialized: tier {}, max {}", tier, user_energy.max_energy);
    Ok(())
}

