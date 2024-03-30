pub mod get_tx_data;

use wasmer::{Store, Module, Instance, Value, imports, Function};
use std::io::{self, Write};

fn main() {
    // Read the Wasm file
    let wasm_bytes = include_bytes!("banana_swap.wasm");

    // Create a store
    let mut store = Store::default();

    // Compile the module
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Create an import object with the host function
    let import_object = imports! {
        "env" => {
            // Pass the host function as an import
            "print_number_to_console" => Function::new_typed(&mut store, print_number_to_console),
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    let add_function = instance.exports.get_function("add").unwrap();

    let result = add_function.call(&mut store, &[3i32.into(), 4i32.into()]).unwrap();
    println!("result {:?}", result);

    assert_eq!(result[0], Value::I32(7));
}

fn print_number_to_console(num: i32) {
    println!("Number received from WebAssembly: {}", num);
}
