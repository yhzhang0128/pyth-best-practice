//! Program instructions
/// PriceStatusCheck comes from the official pyth-sdk-rs

use bytemuck::bytes_of;

use pyth_sdk_solana::state::PriceAccount;
use pyth_sdk_solana::{
    PriceStatus,
};

use crate::id;
use borsh::{
    BorshDeserialize,
    BorshSerialize,
};
use solana_program::instruction::Instruction;

#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PythClientInstruction {
    Loan2Value {
        loan:  u64,
        value: u64,
    },
    PriceStatusCheck {
        // A Price serialized as a vector of bytes.
        // This field is stored as a vector of bytes
        // (instead of a Price) so that we do not have to add
        // Borsh serialization to all structs, which is expensive.
        price_account_data:    Vec<u8>,
        expected_price_status: PriceStatus,
    },
}

pub fn loan_to_value() -> Instruction {
    Instruction {
        program_id: id(),
        accounts:   vec![],
        data:       PythClientInstruction::Loan2Value {
            loan:  1,
            value: 2,
        }
        .try_to_vec()
        .unwrap(),
    }
}

// Returns ok if price account status matches given expected price status.
pub fn price_status_check(price: &PriceAccount, expected_price_status: PriceStatus) -> Instruction {
    Instruction {
        program_id: id(),
        accounts:   vec![],
        data:       PythClientInstruction::PriceStatusCheck {
            price_account_data: bytes_of(price).to_vec(),
            expected_price_status,
        }
        .try_to_vec()
        .unwrap(),
    }
}
