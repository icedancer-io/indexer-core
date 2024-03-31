#[no_mangle]
pub extern "C" fn process_bytes(ptr: *const u8, len: usize) -> u8 {
    if len > 0 {
        let bytes = unsafe { std::slice::from_raw_parts(ptr, len) };
        let first = bytes[0];

        first
    } else {
        0
    }
}
