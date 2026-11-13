//! veVCoin: non-transferable receipt of VCoin locked in `staking-protocol`.
//!
//! * One veVCoin is minted per VCoin locked and burned when it is unlocked or slashed:
//!   a 1:1 receipt of principal, so it can never drift from staking records (audit
//!   STK-03/VEV-01). Voting weight is NOT read from this balance; the staking program's
//!   voter-weight plugin computes it from lock time.
//! * Mint: NonTransferable + PermanentDelegate = config PDA, no freeze authority.
//! * Only the staking registrar PDA may mint/burn; it is derived at init from a pinned
//!   program id and cannot be changed by any admin (audit VEV-02).
//! * Burns use `min(requested, balance)`, so a holder's own burn never blocks unstaking.

use anchor_lang::prelude::*;

pub mod constants;
pub mod contexts;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

#[cfg(test)]
mod tests;

use contexts::*;

declare_id!("FB39ae9x53FxVL3pER9LqCPEx2TRnEnQP55i838Upnjx");

#[program]
pub mod vevcoin_token {
    use super::*;

    /// Creates the config and the veVCoin mint. Upgrade authority only.
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// Mints receipts. Signer must be the staking registrar PDA.
    pub fn mint_vevcoin(ctx: Context<MintVeVCoin>, amount: u64) -> Result<()> {
        instructions::mint_vevcoin(ctx, amount)
    }

    /// Burns `min(amount, balance)` receipts. Signer must be the staking registrar PDA.
    pub fn burn_vevcoin(ctx: Context<BurnVeVCoin>, amount: u64) -> Result<()> {
        instructions::burn_vevcoin(ctx, amount)
    }

    pub fn propose_authority(ctx: Context<UpdateConfig>, new_authority: Pubkey) -> Result<()> {
        instructions::propose_authority(ctx, new_authority)
    }

    pub fn cancel_authority_transfer(ctx: Context<UpdateConfig>) -> Result<()> {
        instructions::cancel_authority_transfer(ctx)
    }

    pub fn accept_authority(ctx: Context<AcceptAuthority>) -> Result<()> {
        instructions::accept_authority(ctx)
    }
}
