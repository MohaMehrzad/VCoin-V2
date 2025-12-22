use anchor_lang::prelude::*;

use crate::contexts::RegisterOracle;
use crate::errors::FiveAError;

/// Register an oracle
pub fn handler(ctx: Context<RegisterOracle>, name: String) -> Result<()> {
    let config = &mut ctx.accounts.five_a_config;
    
    require!(config.oracle_count < 10, FiveAError::MaxOraclesReached);
    
    // Check if already registered
    let oracle_key = ctx.accounts.oracle_wallet.key();
    for i in 0..config.oracle_count as usize {
        require!(
            config.oracles[i] != oracle_key,
            FiveAError::OracleAlreadyRegistered
        );
    }
    
    // Add to config
    let idx = config.oracle_count as usize;
    config.oracles[idx] = oracle_key;
    config.oracle_count += 1;
    
    // Initialize oracle account
    let oracle = &mut ctx.accounts.oracle;
    oracle.wallet = oracle_key;
    
    let name_bytes = name.as_bytes();
    let len = name_bytes.len().min(32);
    oracle.name[..len].copy_from_slice(&name_bytes[..len]);
    
    oracle.is_active = true;
    oracle.total_submissions = 0;
    oracle.accuracy_rate = 10000;
    oracle.last_submission = 0;
    oracle.bump = ctx.bumps.oracle;
    
    msg!("Oracle registered: {}", oracle_key);
    Ok(())
}

