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

use std::mem;
use std::str::FromStr;
use pyth_sdk_solana::load_price_feed_from_account_info;

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
            let mut lamports = 2;
            let mut data = vec![0; mem::size_of::<u32>()];
            let key = Pubkey::from_str(
                "EdVCmQ9FSPcVe5YySXDPCRmc8aDQLKJ9xvYBMZPie1Vw"
            ).unwrap();
            let owner = Pubkey::from_str(
                "gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s"
            ).unwrap();
            // https://docs.rs/solana-program/1.5.1/solana_program/account_info/struct.AccountInfo.html
            msg!("Constructing account info");
            let account = AccountInfo::new(
                &key,
                false,          // is_signer
                false,          // is_writable
                &mut lamports,
                &mut data,
                &owner,
                false,          // executable
                366,            // offline
            );
            msg!("Calling Pyth");
            // let feed = load_price_feed_from_account_info(&account).unwrap();
            // let result = feed.get_current_price().unwrap();
            // msg!("exponent: \t{}", result.expo);
            // msg!("conf: \t\t{}", result.conf);
            // msg!("price: \t\t{}", result.price);

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
