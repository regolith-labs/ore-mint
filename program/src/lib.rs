mod mint_ore;

use mint_ore::*;

use ore_mint_api::instruction::*;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&ore_mint_api::ID, program_id, data)?;

    match ix {
        OreMintInstruction::MintORE => process_mint_ore(accounts, data)?,
    }

    Ok(())
}

entrypoint!(process_instruction);
