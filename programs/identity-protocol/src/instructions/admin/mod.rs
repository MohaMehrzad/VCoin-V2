/// Admin instructions
pub mod initialize;
pub mod update_verification;
pub mod add_trusted_attester;
pub mod remove_trusted_attester;
pub mod set_paused;
pub mod update_authority;

pub use initialize::*;
pub use update_verification::*;
pub use add_trusted_attester::*;
pub use remove_trusted_attester::*;
pub use set_paused::*;
pub use update_authority::*;
