use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String, 
    pub result: Vec<Performance>,
    pub id: u64
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Performance {
    pub num_non_vote_transactions: u64, 
    pub num_slots: u64, 
    pub num_transactions: u64, 
    pub sample_period_secs: u64, 
    pub slot: u64,
}
