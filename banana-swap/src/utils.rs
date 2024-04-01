use borsh::{BorshDeserialize, BorshSerialize};
use regex::Regex;

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TxData {
    pub slot: u64,
    pub account_keys: Vec<[u8; 32]>,
    pub logs: Vec<String>,
}

pub fn extract_base64(logs: Vec<String>, program_address: &str) -> Vec<String> {
    let mut program_stack = Vec::<String>::new();
    let mut base64_strings = Vec::<String>::new();

    let re_program_start = Regex::new(r"Program\s+(\w+)\s+invoke").unwrap();
    let re_program_end = Regex::new(r"Program\s+(\w+)\s+success").unwrap();
    let re_program_log = Regex::new(r"Program\s+data:\s+([\w+/=]+)").unwrap();

    for log in logs.iter() {
        if let Some(captures) = re_program_start.captures(log) {
            let program_address = &captures[1];

            program_stack.push(program_address.to_string());
        } else if re_program_end.captures(log).is_some() {
            if program_stack.is_empty() {
                panic!("Empty program stack");
            }

            program_stack.pop();
        } else if let Some(captures) = re_program_log.captures(log) {
            let current_program = program_stack.last();

            if current_program.is_some() && current_program.unwrap() == program_address {
                let base64_value = &captures[1];
                base64_strings.push(base64_value.to_string());
            }
        }
    }
    base64_strings
}

// Host functions
extern "C" {
    pub fn get_volume() -> i64;

    pub fn set_volume(add: i64);
}
