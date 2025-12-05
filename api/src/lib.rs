pub mod consts;
pub mod error;
pub mod instruction;
pub mod sdk;
pub mod state;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
}

use steel::*;

declare_id!("mintzxW6Kckmeyh1h6Zfdj9QcYgCzhPSGiC8ChZ6fCx");
