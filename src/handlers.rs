use axum::{extract::State, response::IntoResponse, http::StatusCode, Json};
use std::sync::Arc; 
use crate::blockchain_info; 

// holds client and wallet 
pub struct AppState {
    pub client: reqwest::Client, 
    pub wallet: String, 
}

// Handlers processes the request and sends a response back 
// 1. Parameter -- how it gets shared data, extracts from the request. 
// 2. Return type -- convert whatever your return into an HTTP response 
// 3. Body -- same match block, but using Json(reponse)

pub async fn account_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match blockchain_info::account_info(&state.client, &state.wallet).await {
        Ok(response) => Json(response).into_response(), 
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }

}

pub async fn epoch_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match blockchain_info::epoch_info(&state.client).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() 
    }
}

pub async fn recent_performance_samples(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match blockchain_info::recent_performance_samples(&state.client, Some(3)).await {
        Ok(response) => Json(response).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}

pub async fn signatures_for_address(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match blockchain_info::signatures_for_address(&state.client, &state.wallet).await {
        Ok(response) => {
            if let Some(first) = response.result.first() {
                match blockchain_info::transaction(&state.client, &first.signature).await {
                    Ok(tx) => Json(tx).into_response(), 
                    Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            } else {
                (StatusCode::NOT_FOUND, "No Signatures found".to_string()).into_response()
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
    }
}