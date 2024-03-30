// Host functions
extern "C" {
    fn print_number_to_console(num: i32);
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    let result = a + b;

    unsafe {
        print_number_to_console(result)
    };

    result
}
