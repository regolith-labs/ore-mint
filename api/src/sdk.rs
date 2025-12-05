use solana_program::pubkey::Pubkey;
use steel::*;

use crate::{consts::MINT_ADDRESS, instruction::*, state::*};

pub fn mint_ore(signer: Pubkey, to: Pubkey, amount: u64) -> Instruction {
    let authority_address = authority_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(authority_address, false),
            AccountMeta::new(MINT_ADDRESS, false),
            AccountMeta::new(to, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: MintORE {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
