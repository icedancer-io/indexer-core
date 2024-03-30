use anchor_lang::prelude::*;
use regex::Regex;

#[derive(Debug)]
#[event]
struct SwapEvent {
    market: Pubkey,
    amount_in: u64,
    amount_out: u64
}

fn extract_base64(logs: &[&str], program_address: &str) -> Vec<String> {
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

fn main() {
    let logs = [
      "Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo invoke [1]",
      "Program log: Instruction: Swap",
      "Program data: QMbN6CYIceImFhONSTJ9v+fhBJjmI31WGVv/Sh2Sk7A+XIdf2DosgugDAAAAAAAA5wMAAAAAAAA=",
      "Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo consumed 1244 of 200000 compute units",
      "Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo success"
    ];

    let program_address = "3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo";

    let base64_strings = extract_base64(&logs, program_address);

    if base64_strings.is_empty() {
        println!("Base64 strings not found for program address {}", program_address);
    } else {
        println!("Base64 Strings:");
        for base64_string in base64_strings {
            println!("{}", base64_string);
        }
    }
}
