use anchor_lang::prelude::*;

use crate::contexts::SlashTokens;
use crate::errors::VCoinError;

/// C-NEW-02 Security Fix: DEPRECATED Legacy Slash Function
/// 
/// This function is disabled and will always fail with DeprecatedSlashFunction error.
/// The legacy slash function allowed the permanent delegate to burn user tokens
/// immediately without governance approval, creating centralization risk.
/// 
/// For slashing bad actors, use the governance-approved flow:
/// 1. propose_slash() - Permanent delegate proposes a slash with evidence
/// 2. approve_slash() - Governance approves the slash request  
/// 3. execute_slash() - After 48h timelock, execute the approved slash
/// 
/// This ensures community oversight of all slashing actions.
#[allow(unused_variables)]
pub fn handler(ctx: Context<SlashTokens>, amount: u64) -> Result<()> {
    // C-NEW-02: Legacy slash function is deprecated
    // All slashing must go through the governance-approved flow
    require!(false, VCoinError::DeprecatedSlashFunction);
    
    Ok(())
}

