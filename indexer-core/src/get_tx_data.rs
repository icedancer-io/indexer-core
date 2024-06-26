use std::str::FromStr;

use anchor_lang::prelude::{
    borsh::{BorshDeserialize, BorshSerialize},
    *,
};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TxData {
    pub slot: u64,
    pub account_keys: Vec<Pubkey>,
    pub logs: Vec<String>,
}

pub fn get_tx_data() -> Vec<TxData> {
    let transactions = vec![
        TxData {
            slot: 245,
            account_keys: vec![
                Pubkey::from_str("3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo").unwrap()
            ],
            logs: vec![
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo invoke [1]"),
                String::from("Program log: Instruction: Swap"),
                String::from("Program data: QMbN6CYIceLFI/WD+W7U/b2K3hZfjf5VM8EPoCGiKj8x5UrcAgMWYegDAAAAAAAAZAAAAAAAAAA="),
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo consumed 1245 of 200000 compute units"),
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo success")
              ]
        },
        TxData {
            slot: 246,
            account_keys: vec![
                Pubkey::from_str("3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo").unwrap()
            ],
            logs: vec![
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo invoke [1]"),
                String::from("Program log: Instruction: Swap"),
                String::from("Program data: QMbN6CYIceLFI/WD+W7U/b2K3hZfjf5VM8EPoCGiKj8x5UrcAgMWYegDAAAAAAAAyAAAAAAAAAA="),
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo consumed 1245 of 200000 compute units"),
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo success")
              ]
        },
        TxData {
            slot: 247,
            account_keys: vec![
                Pubkey::from_str("3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo").unwrap()
            ],
            logs: vec![
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo invoke [1]"),
                String::from("Program log: Instruction: Swap"),
                String::from("Program data: QMbN6CYIceLFI/WD+W7U/b2K3hZfjf5VM8EPoCGiKj8x5UrcAgMWYegDAAAAAAAALAEAAAAAAAA="),
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo consumed 1245 of 200000 compute units"),
                String::from("Program 3Zg1z6Yfuv4vAkVcm1j3jxhb1QqF3DPzn7uaf3jeNqwo success")
              ]
        }
    ];

    transactions
}
