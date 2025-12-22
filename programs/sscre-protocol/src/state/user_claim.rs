use anchor_lang::prelude::*;

/// User claim record (tracks all epochs claimed)
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
    /// Bitmap of claimed epochs (last 256 epochs)
    pub claimed_epochs_bitmap: [u64; 4],
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
        (8 * 4) + // claimed_epochs_bitmap
        1;   // bump
    
    /// Check if a specific epoch has been claimed
    pub fn is_epoch_claimed(&self, epoch: u64) -> bool {
        if epoch > 255 {
            return epoch <= self.last_claimed_epoch;
        }
        let bitmap_index = (epoch / 64) as usize;
        let bit_position = epoch % 64;
        if bitmap_index >= 4 {
            return false;
        }
        (self.claimed_epochs_bitmap[bitmap_index] & (1 << bit_position)) != 0
    }
    
    /// Mark an epoch as claimed
    pub fn mark_epoch_claimed(&mut self, epoch: u64) {
        if epoch <= 255 {
            let bitmap_index = (epoch / 64) as usize;
            let bit_position = epoch % 64;
            if bitmap_index < 4 {
                self.claimed_epochs_bitmap[bitmap_index] |= 1 << bit_position;
            }
        }
        if epoch > self.last_claimed_epoch {
            self.last_claimed_epoch = epoch;
        }
    }
}

