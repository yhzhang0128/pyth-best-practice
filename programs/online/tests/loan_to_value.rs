use solana_program_test::*;
use pyth_best_practice_online::instruction;

mod common;
use common::test_instr_exec_ok;

#[tokio::test]
async fn test_loan_to_value() {
    test_instr_exec_ok(instruction::loan_to_value())
    .await;
}
