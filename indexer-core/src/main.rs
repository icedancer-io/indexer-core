pub mod get_tx_data;

use anchor_lang::{prelude::borsh::to_vec, AnchorDeserialize, AnchorSerialize};
use wasmer::{imports, Cranelift, Engine, Function, Instance, Memory, MemoryType, Module, NativeEngineExt, Store, Value};
use std::io::{self, Write};
use solana_client;

use crate::get_tx_data::{get_tx_data, TxData};

fn main() {
    // Read the Wasm file
    let wasm_bytes = include_bytes!("banana_swap.wasm");

    // Create a store
    let mut store = Store::default();

    // Memory
    let memory = Memory::new(&mut store, MemoryType::new(1, None, false)).unwrap();

    // Compile the module
    let module = Module::new(&store, wasm_bytes).unwrap();

    // Create an import object with the host function
    let import_object = imports! {
        "env" => {
            // Pass the host function as an import
            "print_number_to_console" => Function::new_typed(&mut store, print_number_to_console),
            "memory" => memory,
        },
    };

    let instance = Instance::new(&mut store, &module, &import_object).unwrap();
    let function = instance.exports.get_function("process").unwrap();

    let byte_array: Vec<u8> = vec![65, 66, 67, 68, 69];
    let byte_array_len = byte_array.len() as i32;

    let result = function.call(&mut store, &[
        wasmer::Value::I32(byte_array.as_ptr() as i32),
        byte_array_len.into()
    ]).unwrap();

    println!("result {:?}", result);

    // // Get transactions
    // let txs =  get_tx_data();

    // for tx in &txs {
    //     let tx_serialized = to_vec(tx).unwrap();
    //     println!("length {:?}", tx_serialized.len());

    //     // let decoded_data = TxData::try_from_slice(tx_serialized.as_slice()).unwrap();
    //     // println!("decoded data {:?}", decoded_data);

    //     let result = function.call(&mut store, &[
    //         wasmer::Value::I32(tx_serialized.as_ptr() as i32),
    //         wasmer::Value::I32(tx_serialized.len() as i32)
    //     ]).unwrap();
    //     println!("result {:?}", result);

    //     // assert_eq!(result[0], Value::I32(7));
    // }


}

fn print_number_to_console(num: i32) {
    println!("Number received from WebAssembly: {}", num);
}
