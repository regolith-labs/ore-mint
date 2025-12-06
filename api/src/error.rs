use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum OreMintError {
    #[error("Max amount exceeded")]
    MaxAmountExceeded = 0,

    #[error("Max supply exceeded")]
    MaxSupplyExceeded = 1,

    #[error("Mint frequency exceeded")]
    MintFrequencyExceeded = 2,
}

error!(OreMintError);
