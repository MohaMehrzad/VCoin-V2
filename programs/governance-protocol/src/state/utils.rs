use crate::constants::*;

/// Helper to calculate voting power
pub fn calculate_voting_power(
    vevcoin_balance: u64,
    five_a_score: u16,  // 0-10000
    tier: u8,
) -> u64 {
    // Step 1: Quadratic base votes
    let base_votes = integer_sqrt(vevcoin_balance);
    
    // Step 2: 5A boost (1.0x to 2.0x)
    let five_a_boost = 1000 + (five_a_score as u64 / 10); // 1000-2000
    
    // Step 3: Tier multiplier
    let tier_mult = match tier {
        0 => TIER_MULT_NONE,
        1 => TIER_MULT_BRONZE,
        2 => TIER_MULT_SILVER,
        3 => TIER_MULT_GOLD,
        4 => TIER_MULT_PLATINUM,
        _ => TIER_MULT_NONE,
    };
    
    // Step 4: Combined (divide by 1_000_000 to normalize)
    let raw_votes = (base_votes * five_a_boost * tier_mult) / 1_000_000;
    
    // Step 5: Diminishing returns for extreme concentration
    if raw_votes > DIMINISHING_THRESHOLD {
        DIMINISHING_THRESHOLD + integer_sqrt(raw_votes - DIMINISHING_THRESHOLD)
    } else {
        raw_votes
    }
}

/// Integer square root using Newton's method
pub fn integer_sqrt(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    let mut x = n;
    let mut y = (x + 1) / 2;
    while y < x {
        x = y;
        y = (x + n / x) / 2;
    }
    x
}

