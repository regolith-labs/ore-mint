use ore_mint_api::prelude::*;
use steel::*;

/// Initializes the authority account.
pub fn process_init(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let [signer_info, authority_info, system_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    authority_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[AUTHORITY], &ore_mint_api::ID)?;
    system_program.is_program(&system_program::ID)?;

    // Create authority account.
    create_program_account::<Authority>(
        authority_info,
        system_program,
        signer_info,
        &ore_mint_api::ID,
        &[AUTHORITY],
    )?;
    let authority = authority_info.as_account_mut::<Authority>(&ore_mint_api::ID)?;
    authority.last_mint_at = 0;

    Ok(())
}
