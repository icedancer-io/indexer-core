use base64::prelude::*;
use borsh::BorshDeserialize;
use utils::{extract_base64, get_volume, set_volume, TxData};

mod utils;

const PROGRAM_ADDRESS: &str = "3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo";

#[derive(Debug, BorshDeserialize)]
struct SwapEvent {
    pub market: [u8; 32],
    pub amount_in: u64,
    pub amount_out: u64,
}

impl SwapEvent {
    pub fn decode(string_event: &String) -> Self {
        let decoded_bytes = BASE64_STANDARD.decode(string_event).unwrap();
        let slice = decoded_bytes.as_slice();

        // remove discriminator bits
        // TODO calculate length and remove
        let mut slice_without_discriminator = &slice[8..];

        let parsed_event = SwapEvent::deserialize(&mut slice_without_discriminator).unwrap();

        parsed_event
    }
}

#[no_mangle]
pub extern "C" fn process_bytes(ptr: *const u8, len: usize) -> u64 {
    let mut bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
    let tx_data = TxData::deserialize(&mut bytes).unwrap();

    let events = extract_base64(tx_data.logs, PROGRAM_ADDRESS);

    let mut total_volume = unsafe { get_volume() } as u64;

    for string_event in &events {
        let event = SwapEvent::decode(string_event);

        // Process the event and store the result in database
        total_volume += event.amount_out;
    }

    assert!(total_volume <= i64::MAX as u64);

    unsafe { set_volume(total_volume as i64) };

    total_volume
}

#[cfg(test)]
mod test {
    use crate::SwapEvent;

    #[test]
    fn decode() {
        let string_event = String::from(
            "QMbN6CYIceLFI/WD+W7U/b2K3hZfjf5VM8EPoCGiKj8x5UrcAgMWYegDAAAAAAAAZAAAAAAAAAA=",
        );

        let event = SwapEvent::decode(&string_event);
        println!("event {:?}", event);
    }
}
