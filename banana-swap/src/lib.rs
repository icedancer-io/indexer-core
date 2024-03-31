use borsh::{BorshSerialize, BorshDeserialize};

// this is causing problem, even if nothing is used
// use anchor_lang::prelude::{borsh::{BorshDeserialize, BorshSerialize, try_from_slice_with_schema}, *};

// Host functions
extern "C" {
    fn print_number_to_console(num: i32);
}

// use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TxData {
    pub slot: u64,
    pub account_keys: Vec<[u8; 32]>,
    pub logs: Vec<String>,
}

// #[no_mangle]
// pub extern "C" fn add(a: i32, b: i32) -> i32 {
//     let result = a + b;

//     unsafe {
//         print_number_to_console(result)
//     };

//     result
// }

#[no_mangle]
pub extern "C" fn process(ptr: *const u8, len: usize) -> i32 {
    // println!("in process()");
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };

    let first = bytes[0];

    first as i32
    // len as i32
    // // println!("Received data: {:?}", data_slice);

    // let decoded_data = TxData::try_from_slice(data_slice).unwrap();

    // println!("Decoded {:?}", decoded_data);
}
