pub mod get_tx_data;
pub mod entity;

use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::get_tx_data::get_tx_data;
use anchor_lang::{prelude::borsh::to_vec, solana_program::pubkey::Pubkey};
use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, Value};

fn main() {
    // Hashmap storing total volume. A Postgres database will be used instead in production
    let market_volumes = HashMap::<Pubkey, u64>::new();

    let shared_counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    #[derive(Clone)]
    struct Env {
        counter: Arc<Mutex<i32>>,
    }

    // Create the functions
    fn get_counter(env: FunctionEnvMut<Env>) -> i32 {
        *env.data().counter.lock().unwrap()
    }
    fn add_to_counter(env: FunctionEnvMut<Env>, add: i32) -> i32 {
        let mut counter_ref = env.data().counter.lock().unwrap();

        *counter_ref += add;
        *counter_ref
    }

    // Read the Wasm file
    let wasm_bytes = include_bytes!("banana_swap.wasm");

    // Create a store
    let mut store = Store::default();

    // Compile the module
    let module = Module::new(&store, wasm_bytes).unwrap();

    let env = FunctionEnv::new(
        &mut store,
        Env {
            counter: shared_counter.clone(),
        },
    );


    // Create an import object with the host function
    let import_object = imports! {
        "env" => {
            "get_counter" => Function::new_typed_with_env(&mut store, &env, get_counter),
            "add_to_counter" => Function::new_typed_with_env(&mut store, &env, add_to_counter),
        }
    };

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
