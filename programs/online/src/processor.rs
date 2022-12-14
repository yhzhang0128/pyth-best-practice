//! Program instruction processor
/// PriceStatusCheck comes from the official pyth-sdk-rs

use solana_program::msg;
use solana_program::pubkey::Pubkey;
use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::program_error::ProgramError;

use borsh::BorshDeserialize;
use crate::instruction::PythClientInstruction;
use pyth_sdk_solana::state::load_price_account;
use pyth_sdk_solana::load_price_feed_from_account_info;

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = PythClientInstruction::try_from_slice(input).unwrap();
    match instruction {
        PythClientInstruction::Loan2Value {} => {
            // Suppose we have 1 loan token and 3000 collateral token
            let loan_cnt = 1;
            let collateral_cnt = 3000;
            
            let loan = &_accounts[0];
            msg!("The loan key: {}", loan.key);
            let feed1 = load_price_feed_from_account_info(&loan).unwrap();
            let result1 = feed1.get_current_price().unwrap();
            let loan_value = result1.price * loan_cnt;
            msg!("loan unit price: {}", result1.price);

            let collateral = &_accounts[1];
            msg!("The collateral key: {}", collateral.key);
            let feed2 = load_price_feed_from_account_info(&collateral).unwrap();
            let result2 = feed2.get_current_price().unwrap();
            let collateral_value = result2.price * collateral_cnt;
            msg!("collateral unit price: {}", result2.price);

            if collateral_value > loan_value {
                Ok(())
            } else {
                Err(ProgramError::Custom(0))
            }
        },
        PythClientInstruction::PriceStatusCheck {
            price_account_data,
            expected_price_status,
        } => {
            let price_account = load_price_account(price_account_data.as_ref())?;
            let price = price_account.to_price_feed(&Pubkey::default());

            if price.status == expected_price_status {
                Ok(())
            } else {
                Err(ProgramError::Custom(0))
            }
        }
    }
}
