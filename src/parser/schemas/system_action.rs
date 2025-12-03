// src/parser/schemas/system_action.rs

use sonic_rs::{Deserialize, Serialize, Value};

#[derive(Debug, Deserialize, Serialize)]
pub struct SystemAction {
    pub user: String,
    pub nonce: u64,
    pub evm_tx_hash: String,
    pub action: Value,
}