use serde::{Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct RPCResponse {
    pub json_rpc: String, 
    pub result: EpochInfo, 
    pub id: u64, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct EpochInfo {
    pub absolute_slot: u64, 
    pub block_height: u64, 
    pub epoch: u64, 
    pub slot_index: u64, 
    pub lot_in_epoch: u64, 
    pub transaction_count: u64,
}