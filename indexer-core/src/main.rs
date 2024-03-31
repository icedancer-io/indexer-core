pub mod get_tx_data;

use wasmer::{imports, Cranelift, Instance, Module, Store, Value};

fn main() {
    // Read the Wasm file
    let wasm_bytes = include_bytes!("banana_swap.wasm");

    // Create a store
    let mut store = Store::default();

    // let compiler = Cranelift::default();
    // let mut store = Store::new(compiler);

    // Compile the module
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Create an import object with the host function
    let import_object = imports! {};

    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    let function = instance.exports.get_function("process_bytes").unwrap();

    // Example byte array
    let byte_array: Vec<u8> = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];

    // Call the exported function with the byte array
    let result = function.call(&mut store, &[
        Value::I32(byte_array.as_ptr() as i32),
        Value::I32(byte_array.len() as i32)
    ]).unwrap();

    // Check the result
    println!("Result: {:?}", result);

}
