use borsh::BorshDeserialize;
use helloworld::{process_instruction, Fibonacci};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Signer,
    transaction::Transaction,
};
use std::mem;

#[tokio::test]
async fn test_helloworld() {
    let program_id = Pubkey::new_unique();
    let fibo_pubkey = Pubkey::new_unique();

    let mut program_test = ProgramTest::new(
        "helloworld", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );
    program_test.add_account(
        fibo_pubkey,
        Account {
            lamports: 5,
            data: vec![0_u8; mem::size_of::<u32>()],
            owner: program_id,
            ..Account::default()
        },
    );
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // Verify account has zero greetings
    let fibo_account = banks_client
        .get_account(fibo_pubkey)
        .await
        .expect("get_account")
        .expect("fibo_account not found");
    assert_eq!(
        Fibonacci::try_from_slice(&fibo_account.data)
            .unwrap()
            .val,
        0
    );

    // Greet once
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(fibo_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has one greeting
    let fibo_account = banks_client
        .get_account(fibo_pubkey)
        .await
        .expect("get_account")
        .expect("fibo_account not found");
    assert_eq!(
        Fibonacci::try_from_slice(&fibo_account.data)
            .unwrap()
            .val,
        1
    );

    // Greet again
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[1], // ignored but makes the instruction unique in the slot
            vec![AccountMeta::new(fibo_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // Verify account has two greetings
    let fibo_account = banks_client
        .get_account(fibo_pubkey)
        .await
        .expect("get_account")
        .expect("fibo_account not found");
    assert_eq!(
        Fibonacci::try_from_slice(&fibo_account.data)
            .unwrap()
            .val,
        2
    );
}
