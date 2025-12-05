use serde::{Deserialize, Serialize};
use steel::*;

use super::OreAccount;

/// Account which has the mint authority for the ORE token.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Authority {
    /// The slot of the last mint.
    pub last_mint_at: u64,
}

account!(OreAccount, Authority);
