use ethereum::{BlockAny, Log, ReceiptAny};
use ethereum_types::Bloom;
use primitive_types::{H160, H256, U256};
use serde::{Deserialize, Serialize};

pub const PREFIX: &str = "evm";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBasic {
    pub balance: U256,
    pub code: Vec<u8>,
    pub nonce: U256,
}

pub type Block = BlockAny;

pub type Receipt = ReceiptAny;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub transaction_hash: H256,
    pub transaction_index: u32,
    pub from: H160,
    pub to: Option<H160>,
    pub contract_address: Option<H160>,
    pub logs: Vec<Log>,
    pub logs_bloom: Bloom,
}
