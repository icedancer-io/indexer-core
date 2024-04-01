use borsh::BorshDeserialize;
use utils::TxData;

mod utils;

#[no_mangle]
pub extern "C" fn process_bytes(ptr: *const u8, len: usize) -> u64 {
    let mut bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let tx_data = TxData::deserialize(&mut bytes).unwrap();

    tx_data.slot
}
