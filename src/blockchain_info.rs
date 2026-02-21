use reqwest;
use tokio; 
use dotenv; 
use serde_json::Result; 
use crate::account_info::RPCResponse as AccountInfo; 
use crate::epoch_info::RPCResponse as EpochInfo;
use crate::recent_performance_samples::RPCResponse as PerformanceSamples;

// Entrypoint URL for Solana blockchain; Sending requests here
const HOST_ROOT: &str = "https://api.mainnet-beta.solana.com"; 


pub async fn send_request(url: &str, body: serde_json::Value) -> String {
    let client = reqwest::Client::new(); 
    client
        .post(url)
        .header("Content-Type", "application/json")
        .header("api-key", dotenv::var("API_KEY").expect("Could not find API KEY"))
        .json(&body)
        .send()
        .await
        .expect("Failed to get reponse")
        .text()
        .await
        .expect("Failed to conver payload")
}

pub async fn account_info_request(pubkey:&str) {
    let body = serde_json::json!({
        "jsonrpc": "2.0", 
        "id": 1, 
        "method": "getAccountInfo",
        "params": [pubkey],  
    });
    
    let response = send_request(&HOST_ROOT, body).await; 
    println!("{}", response); 
}