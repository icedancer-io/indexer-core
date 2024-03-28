use wasmer::{Store, Module, Instance, Value, imports};

fn main() {
    // Read the Wasm file
    let wasm_bytes = include_bytes!("banana_swap.wasm");

    // Create a store
    let mut store = Store::default();

    // Compile the module
    let module = Module::new(&store, wasm_bytes).unwrap();

    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).unwrap();

    let add_function = instance.exports.get_function("add").unwrap();
    let result = add_function.call(&mut store, &[3i32.into(), 4i32.into()]).unwrap();
    println!("result {:?}", result);

    assert_eq!(result[0], Value::I32(7));
}
