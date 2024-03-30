use anchor_lang::prelude::*;
// use base64::decode;
use base64::prelude::*;


#[derive(Debug)]
#[event]
struct SwapEvent {
    market: Pubkey,
    amount_in: u64,
    amount_out: u64
}

fn main() {
    let log = "QMbN6CYIceImFhONSTJ9v+fhBJjmI31WGVv/Sh2Sk7A+XIdf2DosgugDAAAAAAAA5wMAAAAAAAA=";
    let decoded_bytes = BASE64_STANDARD.decode(log).unwrap();
    let mut slice = decoded_bytes.as_slice();

    let event = SwapEvent::deserialize(&mut slice).unwrap();
    println!("event {:?}", event);
}
