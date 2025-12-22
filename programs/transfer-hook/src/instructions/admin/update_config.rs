use anchor_lang::prelude::*;

use crate::contexts::UpdateConfig;

/// Update hook configuration
pub fn handler(
    ctx: Context<UpdateConfig>,
    new_five_a_program: Option<Pubkey>,
    new_min_activity_amount: Option<u64>,
    block_wash_trading: Option<bool>,
) -> Result<()> {
    let config = &mut ctx.accounts.hook_config;
    
    if let Some(program) = new_five_a_program {
        config.five_a_program = program;
    }
    if let Some(amount) = new_min_activity_amount {
        config.min_activity_amount = amount;
    }
    if let Some(block) = block_wash_trading {
        config.block_wash_trading = block;
    }
    
    msg!("Hook config updated");
    Ok(())
}

