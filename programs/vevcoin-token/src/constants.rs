//! veVCoin constants.

use anchor_lang::prelude::*;

#[constant]
pub const VEVCOIN_CONFIG_SEED: &[u8] = b"vevcoin-config";

/// Mint PDA seed. The veVCoin mint is created by this program at this address.
#[constant]
pub const VEVCOIN_MINT_SEED: &[u8] = b"vevcoin-mint";

#[constant]
pub const VEVCOIN_DECIMALS: u8 = 9;

/// The staking program; its registrar PDA is the only account allowed to mint/burn.
/// Pinned (not configurable) so no admin can redirect minting/burning (audit VEV-02).
pub const STAKING_PROGRAM_ID: Pubkey = pubkey!("6EFcistyr2E81adLUcuBJRr8W2xzpt3D3dFYEcMewpWu");

/// Seed of the staking program's registrar PDA (must match `staking_protocol::REGISTRAR_SEED`).
#[constant]
pub const STAKING_REGISTRAR_SEED: &[u8] = b"registrar";
