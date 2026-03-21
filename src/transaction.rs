use serde:: {Deserialize, Serialize}; 

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RPCResponse {
    pub jsonrpc: String, 
    pub result: Transaction,
    pub id: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_time: Option<i64>, 
    pub meta: Meta,
    pub slot: i64, 
    pub transaction: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta { 
    pub err: Option<serde_json::Value>, 
    pub fee: i64, 
    pub inner_instructions: Vec<Vec<String>>,
    pub loaded_addresses: Loaded,
    pub log_messages: Vec<String>, 
    pub post_balances: Vec<i64>, 
    pub post_token_balances: Vec<PostToken>,
    pub pre_balances: Vec<i64>, 
    pub pre_token_balances: Vec<PreToken>,
    pub rewards: Vec<String>, 
    pub status: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Loaded {
    pub readonly: Vec<String>, 
    pub writable: Vec<String>, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostToken { 
    pub account_index: i64, 
    pub mint: String, 
    pub owner: String, 
    pub program_id: String, 
    pub ui_token_amount: PostAmount, 
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostAmount {
    pub amount: String, 
    pub decimals: i64, 
    pub ui_amount: Option<f64>, 
    pub ui_amount_string: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreToken { 
    pub account_index: i64, 
    pub mint: String, 
    pub owner: String,
    pub program_id: String, 
    pub ui_token_amount: PreAmount,  
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreAmount {
    pub amount: String, 
    pub decimals: i64, 
    pub ui_amount: Option<f64>, 
    pub ui_amount_string: String,
}
