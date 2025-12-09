use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum OreMintInstruction {
    Init = 0,
    MintORE = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Init {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct MintORE {
    pub amount: [u8; 8],
}

instruction!(OreMintInstruction, Init);
instruction!(OreMintInstruction, MintORE);
