use anchor_lang::prelude::*;
use crate::contexts::UpdateUserTier;
use crate::state::UserEnergy;

pub fn handler(ctx: Context<UpdateUserTier>, new_tier: u8) -> Result<()> {
    let user_energy = &mut ctx.accounts.user_energy;
    
    user_energy.tier = new_tier;
    user_energy.max_energy = UserEnergy::max_energy_for_tier(new_tier);
    user_energy.regen_rate = UserEnergy::regen_rate_for_tier(new_tier);
    
    // Cap current energy at new max
    if user_energy.current_energy > user_energy.max_energy {
        user_energy.current_energy = user_energy.max_energy;
    }
    
    msg!("User tier updated: {}", new_tier);
    Ok(())
}

