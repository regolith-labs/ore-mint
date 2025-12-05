mod authority;

pub use authority::*;

use crate::consts::*;

use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum OreAccount {
    Authority = 100,
}

pub fn authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[AUTHORITY], &crate::ID)
}
