
use solana_program_test::*;
use solana_sdk::{
    account::{Account, WritableAccount},
    clock::Epoch,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_program::pubkey::Pubkey;
use pod_test::{self, pod::*, *};

fn program_test() -> ProgramTest {
    ProgramTest::new(
        "pod_test",
        id(),
        processor!(processor::process_instruction),
    )
}

const ACCOUNT_RENT_EXEMPTION: u64 = 1_000_000_000; // go with something big to be safe

async fn get_test_account_state(
    banks_client: &mut BanksClient,
    test_account_address: Pubkey,
) -> state::TestAccount {
    let account = banks_client
        .get_account(test_account_address)
        .await
        .expect("get_account")
        .expect("zk_token_account not found");
    *state::TestAccount::from_bytes(&account.data).unwrap()
}

fn add_test_account(
    program_test: &mut ProgramTest,
) -> Pubkey {
    let test_address = Keypair::new().pubkey();

    let test_account_state = state::TestAccount {
        associated_counter: 0.into(),
    };

    let test_account = Account::create(
        ACCOUNT_RENT_EXEMPTION,
        bytemuck::bytes_of(&test_account_state).to_vec(),
        id(),
        false,
        Epoch::default(),
    );

    program_test.add_account(test_address, test_account);
    test_address
}


#[tokio::test]
async fn test_pod_update() {
    let mut program_test = program_test();

    let test_address = add_test_account(&mut program_test);

    let instructions = instruction::build_instruction(
        test_address,
        test_address, // remove
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;


    let mut transaction = Transaction::new_with_payer(
        &[instructions],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    assert_eq!(
        u64::from(
            get_test_account_state(&mut banks_client, test_address)
                .await
                .associated_counter
        ),
        1,
    );
}
