/// Admin instructions
pub mod initialize;
pub mod register_oracle;
pub mod set_paused;
pub mod update_authority;
pub mod accept_authority;
pub mod cancel_authority_transfer;

pub use initialize::*;
pub use register_oracle::*;
pub use set_paused::*;
pub use update_authority::*;
pub use accept_authority::*;
pub use cancel_authority_transfer::*;
