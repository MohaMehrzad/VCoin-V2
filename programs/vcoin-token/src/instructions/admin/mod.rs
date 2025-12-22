/// Admin instructions
pub mod initialize;
pub mod set_paused;
pub mod update_authority;
pub mod accept_authority;
pub mod cancel_authority_transfer;
pub mod update_delegate;

pub use initialize::*;
pub use set_paused::*;
pub use update_authority::*;
pub use accept_authority::*;
pub use cancel_authority_transfer::*;
pub use update_delegate::*;
