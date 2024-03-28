use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process_data(data: &[u8]) -> Vec<u8> {
    // Process the data here, for example, simply double each byte
    vec![7]
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data: [u8; 1] = [0];

        let result = process_data(&data);
        assert_eq!(result, vec![7]);
    }
}
