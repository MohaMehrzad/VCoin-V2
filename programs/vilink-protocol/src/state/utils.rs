use anchor_lang::prelude::*;

/// Get action type name
pub fn action_type_name(action_type: u8) -> &'static str {
    match action_type {
        0 => "Tip",
        1 => "Vouch",
        2 => "Follow",
        3 => "Challenge",
        4 => "Stake",
        5 => "ContentReact",
        6 => "Delegate",
        7 => "Vote",
        _ => "Unknown",
    }
}

/// Generate action ID from inputs
pub fn generate_action_id(
    creator: &Pubkey,
    target: &Pubkey,
    action_type: u8,
    amount: u64,
    timestamp: i64,
) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(81);
    data.extend_from_slice(creator.as_ref());
    data.extend_from_slice(target.as_ref());
    data.push(action_type);
    data.extend_from_slice(&amount.to_le_bytes());
    data.extend_from_slice(&timestamp.to_le_bytes());
    
    keccak::hash(&data).to_bytes()
}

/// Generate dApp ID from authority
pub fn generate_dapp_id(authority: &Pubkey) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(40);
    data.extend_from_slice(b"vilink-dapp");
    data.extend_from_slice(authority.as_ref());
    
    keccak::hash(&data).to_bytes()
}

/// Generate batch ID
pub fn generate_batch_id(creator: &Pubkey, timestamp: i64) -> [u8; 32] {
    use solana_program::keccak;
    
    let mut data = Vec::with_capacity(48);
    data.extend_from_slice(b"vilink-batch");
    data.extend_from_slice(creator.as_ref());
    data.extend_from_slice(&timestamp.to_le_bytes());
    
    keccak::hash(&data).to_bytes()
}

