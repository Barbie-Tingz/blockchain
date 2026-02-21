use serde::{Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String, 
    pub result: AccountInfo, 
    pub id: u64, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct AccountInfo {
    pub context: Context, 
    pub value: AccountValue, 
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Context {
    pub api_version: String, 
    pub slot: u64, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct AccountValue {
     pub data: Vec<String>,
     pub executable: bool, 
     pub lamports: u64, 
     pub owner: String,
     pub rent_epoch: u64, 
     pub space: u64,
}


