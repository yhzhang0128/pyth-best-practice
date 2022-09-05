use solana_program_test::*;
use pyth_best_practice_online::instruction;

mod common;
use common::test_instr_exec_ok;

use std::str::FromStr;
use solana_program::pubkey::Pubkey;
use solana_program::instruction::AccountMeta;

#[tokio::test]
async fn test_loan_to_value() {
    // Suppose the loan is ETH, see
    // https://pyth.network/zh/price-feeds/crypto-eth-usd/?cluster=devnet
    let loan_key = Pubkey::from_str(
        "EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw"
    ).unwrap();
    let loan = AccountMeta::new(loan_key, false);

    // Suppose the collateral is USDT, see
    // https://pyth.network/zh/price-feeds/crypto-usdc-usd?cluster=devnet
    let collateral_key = Pubkey::from_str(
        "5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7"
    ).unwrap();
    let collateral = AccountMeta::new(collateral_key, false);

    test_instr_exec_ok(instruction::loan_to_value(loan, collateral))
    .await;
}
