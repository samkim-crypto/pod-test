use crate::*;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub fn build_instruction(
    test_account_1: Pubkey,
    test_account_2: Pubkey
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(test_account_1, false),
        AccountMeta::new(test_account_2, false),
    ];

    Instruction {
        program_id: id(),
        accounts,
        data: vec![],
    }
}
