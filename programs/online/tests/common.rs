use solana_program::instruction::Instruction;
use solana_program_test::*;
use solana_sdk::signature::Signer;
use solana_sdk::transaction::Transaction;
use pyth_best_practice_online::id;
use pyth_best_practice_online::processor::process_instruction;

// Panics if running instruction fails
pub async fn test_instr_exec_ok(instr: Instruction) {
    let mut context = ProgramTest::new("pyth_best_practice_online", id(), processor!(process_instruction))
        .start_with_context()
        .await;

    context.warp_to_slot(1000).unwrap();

    let mut transaction = Transaction::new_with_payer(&[instr], Some(&context.payer.pubkey()));
    transaction.sign(&[&context.payer], context.last_blockhash);
    context
        .banks_client
        .process_transaction(transaction)
        .await
        .unwrap()
}
