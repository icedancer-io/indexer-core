pub mod get_tx_data;

use std::sync::{Arc, Mutex};

use crate::get_tx_data::get_tx_data;
use anchor_lang::prelude::borsh::to_vec;
use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, Value};

fn main() {
    // Total volume for a market. A Postgres database will be used instead in production
    let volume_counter: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));

    #[derive(Clone)]
    struct Env {
        volume: Arc<Mutex<i64>>,
    }

    // Create the functions
    fn get_volume(env: FunctionEnvMut<Env>) -> i64 {
        *env.data().volume.lock().unwrap()
    }
    fn set_volume(env: FunctionEnvMut<Env>, value: i64) {
        let mut counter_ref = env.data().volume.lock().unwrap();

        *counter_ref = value;
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
            volume: volume_counter.clone(),
        },
    );

    // Create an import object with the host function
    let import_object = imports! {
        "env" => {
            "get_volume" => Function::new_typed_with_env(&mut store, &env, get_volume),
            "set_volume" => Function::new_typed_with_env(&mut store, &env, set_volume),
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

        function
            .call(
                &mut store,
                &[
                    Value::I32(1),
                    wasmer::Value::I32(tx_serialized.len() as i32),
                ],
            )
            .unwrap();
    }

    let volume = *volume_counter.lock().unwrap();

    println!("total volume {:?}", volume);
}
