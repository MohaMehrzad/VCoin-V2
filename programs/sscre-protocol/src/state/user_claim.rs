use anchor_lang::prelude::*;

use crate::errors::SSCREError;

/// User claim record (tracks all epochs claimed)
/// H-04 Security Fix: Extended to properly handle epochs > 255
/// H-NEW-04 Security Fix: Changed high_epochs from array to bitmap for epochs 512-1023
#[account]
#[derive(Default)]
pub struct UserClaim {
    /// User wallet
    pub user: Pubkey,
    /// Last claimed epoch
    pub last_claimed_epoch: u64,
    /// Total VCoin claimed all-time
    pub total_claimed: u64,
    /// Total claims made
    pub claims_count: u32,
    /// First claim timestamp
    pub first_claim_at: i64,
    /// Last claim timestamp
    pub last_claim_at: i64,
    /// Bitmap of claimed epochs 0-255
    pub claimed_epochs_bitmap: [u64; 4],
    /// H-04 Fix: Extended bitmap for epochs 256-511
    pub claimed_epochs_bitmap_ext: [u64; 4],
    /// H-NEW-04 Fix: Bitmap for epochs 512-1023 (replaces array approach)
    /// This supports unlimited claims for epochs 512-1023 (512 more epochs = 42+ years)
    pub high_epochs_bitmap: [u64; 8],
    /// Reserved for future epochs 1024+ if ever needed
    pub _reserved: [u8; 8],
    /// PDA bump
    pub bump: u8,
}

impl UserClaim {
    pub const LEN: usize = 8 + // discriminator
        32 + // user
        8 +  // last_claimed_epoch
        8 +  // total_claimed
        4 +  // claims_count
        8 +  // first_claim_at
        8 +  // last_claim_at
        (8 * 4) + // claimed_epochs_bitmap (epochs 0-255)
        (8 * 4) + // claimed_epochs_bitmap_ext (epochs 256-511)
        (8 * 8) + // high_epochs_bitmap (epochs 512-1023) - H-NEW-04
        8 +  // _reserved
        1;   // bump
    
    /// Check if a specific epoch has been claimed
    /// H-04 + H-NEW-04: Properly handles all epoch ranges 0-1023 using bitmaps
    pub fn is_epoch_claimed(&self, epoch: u64) -> bool {
        if epoch < 256 {
            // Epochs 0-255: Use first bitmap
            let bitmap_index = (epoch / 64) as usize;
            let bit_position = epoch % 64;
            (self.claimed_epochs_bitmap[bitmap_index] & (1 << bit_position)) != 0
        } else if epoch < 512 {
            // Epochs 256-511: Use extended bitmap
            let adjusted_epoch = epoch - 256;
            let bitmap_index = (adjusted_epoch / 64) as usize;
            let bit_position = adjusted_epoch % 64;
            (self.claimed_epochs_bitmap_ext[bitmap_index] & (1 << bit_position)) != 0
        } else if epoch < 1024 {
            // H-NEW-04: Epochs 512-1023: Use high epochs bitmap
            let adjusted_epoch = epoch - 512;
            let bitmap_index = (adjusted_epoch / 64) as usize;
            let bit_position = adjusted_epoch % 64;
            (self.high_epochs_bitmap[bitmap_index] & (1 << bit_position)) != 0
        } else {
            // Epochs 1024+: Not supported (would be year 85+ at monthly epochs)
            false
        }
    }
    
    /// Mark an epoch as claimed
    /// H-04 + H-NEW-04: Handles epochs 0-1023 with bitmap storage
    pub fn mark_epoch_claimed(&mut self, epoch: u64) -> Result<()> {
        if epoch < 256 {
            // Epochs 0-255: Use first bitmap
            let bitmap_index = (epoch / 64) as usize;
            let bit_position = epoch % 64;
            self.claimed_epochs_bitmap[bitmap_index] |= 1 << bit_position;
        } else if epoch < 512 {
            // Epochs 256-511: Use extended bitmap
            let adjusted_epoch = epoch - 256;
            let bitmap_index = (adjusted_epoch / 64) as usize;
            let bit_position = adjusted_epoch % 64;
            self.claimed_epochs_bitmap_ext[bitmap_index] |= 1 << bit_position;
        } else if epoch < 1024 {
            // H-NEW-04: Epochs 512-1023: Use high epochs bitmap
            let adjusted_epoch = epoch - 512;
            let bitmap_index = (adjusted_epoch / 64) as usize;
            let bit_position = adjusted_epoch % 64;
            self.high_epochs_bitmap[bitmap_index] |= 1 << bit_position;
        } else {
            // Epochs 1024+: Not supported
            require!(false, SSCREError::TooManyEpochsClaimed);
        }
        
        if epoch > self.last_claimed_epoch {
            self.last_claimed_epoch = epoch;
        }
        
        Ok(())
    }
}

