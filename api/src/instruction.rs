use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum OreMintInstruction {
    MintORE = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintORE {
    pub amount: [u8; 8],
}

instruction!(OreMintInstruction, MintORE);
