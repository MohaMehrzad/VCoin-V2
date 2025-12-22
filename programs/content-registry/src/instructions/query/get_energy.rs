use anchor_lang::prelude::*;
use crate::contexts::GetEnergy;

pub fn handler(ctx: Context<GetEnergy>) -> Result<()> {
    let energy = &ctx.accounts.user_energy;
    msg!("User: {}", energy.user);
    msg!("Current: {}/{}", energy.current_energy, energy.max_energy);
    msg!("Regen rate: {}/hr", energy.regen_rate);
    msg!("Tier: {}", energy.tier);
    Ok(())
}

