//! Program instructions for end-to-end testing and instruction counts

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

/// Instructions supported by the pyth-client program, used for testing and
/// instruction counts
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum PythClientInstruction {
    PriceStatusCheck {
        // A Price serialized as a vector of bytes. This field is stored as a vector of bytes
        // (instead of a Price) so that we do not have to add Borsh serialization to all
        // structs, which is expensive.
        price_account_data:    Vec<u8>,
        expected_price_status: PriceStatus,
    },
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
