use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String,
    pub result: AccountInfo,
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub context: Context,
    // Option because Solana returns null if the account doesn't exist
    pub value: Option<AccountValue>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Context {
    pub api_version: String,
    pub slot: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountValue {
    pub data: Vec<String>,    // [base64_encoded_data, "base64"]
    pub executable: bool,     // true if this account is a program (smart contract)
    pub lamports: u64,        // balance in lamports (1 SOL = 1,000,000,000 lamports)
    pub owner: String,        // which program owns/controls this account
    pub rent_epoch: u64,
    pub space: u64,
}