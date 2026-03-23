mod account_info;
mod epoch_info;
mod recent_performance_samples;
mod blockchain_info;
mod transaction; 
mod signatures_for_address; 
mod handlers; 
use axum::{routing::get, Router}; 
use std::sync::Arc; 

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok(); 

    let wallet = dotenv::var("WALLET").expect("Cannot find WALLET"); // This reads you .env file 
    let client = reqwest::Client::new(); // creates the shared client 
    let state = Arc::new(handlers::AppState { client, wallet });

    let app = Router::new().route("/account_info", get(handlers::account_info))
    .route("/epoch_info", get(handlers::epoch_info))
    .route("/recent_performance_samples", get(handlers::recent_performance_samples))
    .route("/signatures_for_address", get(handlers::signatures_for_address))
    .with_state(state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap(); 

    

}

