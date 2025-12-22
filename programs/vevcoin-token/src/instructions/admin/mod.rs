/// Admin instructions
pub mod initialize;
pub mod update_authority;
pub mod accept_authority;
pub mod cancel_authority_transfer;
pub mod update_staking_protocol;

pub use initialize::*;
pub use update_authority::*;
pub use accept_authority::*;
pub use cancel_authority_transfer::*;
pub use update_staking_protocol::*;
