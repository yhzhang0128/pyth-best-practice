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

//use std::str::FromStr;
//use solana_program::pubkey::Pubkey;
//use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed};

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = PythClientInstruction::try_from_slice(input).unwrap();
    match instruction {
        PythClientInstruction::Loan2Value {} => {
            msg!("Calling Loan2Value()");

            // The ETH/USD price key on devnet
            // https://pyth.network/price-feeds/crypto-eth-usd/?cluster=devnet
            // let key = Pubkey::from_str(
            //     "EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw"
            // ).unwrap();
            // TODO: get the price of ETH/USD from Pyth

            let loan = 1;
            let value = 2;

            if value > loan {
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
