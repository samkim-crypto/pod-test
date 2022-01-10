//! Program state processor

use {
    crate::{pod::*, state::*, *},
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        pubkey::Pubkey,
    },
};

// takes `TestAccount` and sets counter to 1
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let test_account_info = next_account_info(account_info_iter)?;
    let test_account = TestAccount::from_account_info(test_account_info, &id())?;

    let mut test_account = test_account.into_mut();
    test_account.associated_counter = 1.into();

    Ok(())
}
