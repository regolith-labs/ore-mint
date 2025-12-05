use ore_mint_api::prelude::*;
use steel::*;

/// Mints ORE to the treasury.
pub fn process_mint_ore(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data.
    let args = MintORE::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, authority_info, mint_info, to_info, token_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&TREASURY_ADDRESS)?;
    let authority = authority_info.as_account_mut::<Authority>(&ore_mint_api::ID)?;
    let mint = mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;
    to_info.as_associated_token_account(&TREASURY_ADDRESS, &MINT_ADDRESS)?;
    token_program.is_program(&spl_token::ID)?;

    // Check amount
    let remaining_supply = MAX_SUPPLY.saturating_sub(mint.supply());
    assert!(amount <= remaining_supply, "Amount too large");
    assert!(amount <= MAX_MINT_AMOUNT, "Amount too large");

    // Check rate of minting.
    let slots_since = clock.slot.saturating_sub(authority.last_mint_at);
    assert!(slots_since >= MIN_SLOTS_BETWEEN_MINT, "Mint too frequent");

    // Update timestamps
    authority.last_mint_at = clock.slot;

    // Mint tokens to the treasury.
    mint_to_signed(
        mint_info,
        to_info,
        authority_info,
        token_program,
        amount,
        &[AUTHORITY],
    )?;

    Ok(())
}
