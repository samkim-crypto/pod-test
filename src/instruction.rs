use crate::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

// Oddly, if we remove the commented lines, then it does not produce errors anymore.
pub fn build_instruction(
    test_account_1: Pubkey,
    test_account_2: Pubkey // remove
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(test_account_1, false),
        AccountMeta::new(test_account_2, false), // remove
    ];

    Instruction {
        program_id: id(),
        accounts,
        data: vec![],
    }
}
