use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct TxData {
    pub slot: u64,
    pub account_keys: Vec<[u8; 32]>,
    pub logs: Vec<String>,
}
