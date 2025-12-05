use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum OreMintError {
    #[error("Amount too large")]
    AmountTooLarge = 0,

    #[error("Mint too frequently")]
    MintTooFrequently = 1,
}

error!(OreMintError);
