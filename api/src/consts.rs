use solana_program::pubkey::{pubkey, Pubkey};

/// The decimal precision of the ORE token.
/// There are 100 billion indivisible units per ORE (called "grams").
pub const TOKEN_DECIMALS: u8 = 11;

/// One ORE token, denominated in indivisible units.
pub const ONE_ORE: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The maximum amount of ORE tokens that can be minted per request.
pub const MAX_MINT_AMOUNT: u64 = ONE_ORE * 2;

/// The number of slots required between mint requests.
pub const MIN_SLOTS_BETWEEN_MINT: u64 = 150;

/// The maximum token supply (5 million).
pub const MAX_SUPPLY: u64 = ONE_ORE * 5_000_000;

/// The seed of the automation account PDA.
pub const AUTHORITY: &[u8] = b"authority";

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey = pubkey!("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp");

/// The treasury address allowed to request a mint.
pub const TREASURY_ADDRESS: Pubkey = pubkey!("45db2FSR4mcXdSVVZbKbwojU6uYDpMyhpEi7cC8nHaWG");
