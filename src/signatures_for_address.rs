use serde::{Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String, 
    pub result: Vec<Signature>, 
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature { 
    pub block_time: Option<i64>, 
    pub confirmation_status:Option<String>, 
    pub err: Option<String>, 
    pub memo: Option<String>, 
    pub signature: String, 
    pub slot: u64, 
}