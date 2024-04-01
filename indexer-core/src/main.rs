pub mod get_tx_data;

use crate::get_tx_data::get_tx_data;
use anchor_lang::prelude::borsh::to_vec;
use wasmer::{imports, Instance, Module, Store, Value};

fn main() {
    // Read the Wasm file
    let wasm_bytes = include_bytes!("banana_swap.wasm");

    // Create a store
    let mut store = Store::default();

    // Compile the module
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Create an import object with the host function
    let import_object = imports! {};

    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    let function = instance.exports.get_function("process_bytes").unwrap();

    // Memory- pass slice via memory
    let memory = instance.exports.get_memory("memory").unwrap();

    let txs = get_tx_data();

    for tx in &txs {
        let tx_serialized = to_vec(tx).unwrap();

        let view = memory.view(&store);
        view.write(1, &tx_serialized).unwrap();

        let result = function
            .call(
                &mut store,
                &[
                    Value::I32(1),
                    wasmer::Value::I32(tx_serialized.len() as i32),
                ],
            )
            .unwrap();
        println!("result {:?}", result);
    }
}
