use anchor_lang::prelude::*;

use crate::errors::SSCREError;

/// User claim record (tracks all epochs claimed)
/// H-04 Security Fix: Extended to properly handle epochs > 255
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
    /// H-04 Fix: Array for high epochs 512+ (stores specific epoch numbers)
    pub high_epochs_claimed: [u64; 32],
    /// H-04 Fix: Number of high epochs stored
    pub high_epochs_count: u8,
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
        (8 * 4) + // claimed_epochs_bitmap_ext (epochs 256-511) - H-04
        (8 * 32) + // high_epochs_claimed (epochs 512+) - H-04
        1 +  // high_epochs_count - H-04
        1;   // bump
    
    /// Check if a specific epoch has been claimed (H-04 Fix)
    /// Properly handles all epoch ranges without false positives
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
        } else {
            // Epochs 512+: Check high epoch array
            for i in 0..self.high_epochs_count as usize {
                if self.high_epochs_claimed[i] == epoch {
                    return true;
                }
            }
            false
        }
    }
    
    /// Mark an epoch as claimed (H-04 Fix)
    /// Handles all epoch ranges with proper storage
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
        } else {
            // Epochs 512+: Add to high epoch array
            require!(
                (self.high_epochs_count as usize) < 32,
                SSCREError::TooManyEpochsClaimed
            );
            self.high_epochs_claimed[self.high_epochs_count as usize] = epoch;
            self.high_epochs_count += 1;
        }
        
        if epoch > self.last_claimed_epoch {
            self.last_claimed_epoch = epoch;
        }
        
        Ok(())
    }
}

