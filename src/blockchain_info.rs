use serde::de::DeserializeOwned;
use crate::account_info::RPCResponse as AccountInfo;
use crate::epoch_info::RPCResponse as EpochInfoResponse;
use crate::recent_performance_samples::RPCResponse as RecentPerfomance;
use crate::transaction::RPCResponse as Transaction; 
use crate::signatures_for_address::RPCResponse as SignaturesForAddress; 

const HOST_ROOT: &str = "https://api.mainnet-beta.solana.com";

// Generic request sender — deserializes directly into whatever type T you ask for
pub async fn send_request<T: DeserializeOwned>(
    client: &reqwest::Client, // creates a connection pool that is able to do multiple requests
    body: serde_json::Value,  
) -> Result<T, reqwest::Error> { // result if fails
client
    .post(HOST_ROOT)
    .header("Content-Type", "application/json")
    .header("api-key", dotenv::var("API_KEY").expect("Could not find API_KEY")) // dotenv parses for the .env file within your project for your password
    .json(&body)
    .send()
    .await?
    .json::<T>() // deserializes the reponse body into the generic type 
    .await // resets automatically after 
}

pub async fn account_info(
    client: &reqwest::Client,
    pubkey:&str,
) -> Result<AccountInfo, reqwest::Error> {
    let body = serde_json::json!({ // creating body of the request
        "jsonrpc": "2.0", 
        "id": 1,
        "method": "getAccountInfo", 
        "params": [pubkey], // pubkey is the wallet address you want to query 
    }); 

    send_request(client, body).await // sends it and returns the result 
}

// does not need a pub key because it is a global and doesnt need a wallet address
pub async fn epoch_info(
    client:&reqwest::Client  
) -> Result<EpochInfoResponse, reqwest::Error> {
    let body = serde_json::json!({
        "jsonrpc": "2.0", 
        "id": 1,
        "method": "getEpochInfo", 
        "params": [],
    }); 

    send_request(client, body).await // sends it and returns the result 
}

pub async fn recent_performance_samples(
    client:&reqwest::Client, 
    limit: Option<u64> 
) -> Result<RecentPerfomance, reqwest::Error> {
    let body = serde_json::json!({
        "jsonrpc": "2.0", 
        "id": 1,
        "method": "getRecentPerformanceSamples", 
        "params": [limit],
    }); 

    send_request(client, body).await 
}


pub async fn transaction(
    client: &reqwest::Client, 
    signature:&str 
) -> Result<Transaction, reqwest::Error> {
    let body = serde_json::json!({
        "jsonrpc": "2.0", 
        "id": 1,
        "method": "getTransaction",
        "params": [signature],
    });

    send_request(client, body).await 
}


pub async fn signatures_for_address(
    client: &reqwest::Client, 
    pubkey: &str
) -> Result<SignaturesForAddress, reqwest::Error> {
    let body = serde_json::json!({
        "jsonrpc": "2.0", 
        "id": 1, 
        "method": "getSignaturesForAddress", 
        "params":[pubkey],
    });

    send_request(client, body).await
}

