use axum::extract::{Path, Query, Json};
use axum::http::StatusCode; 

pub struct AppState {
    pub client: reqwest::Client, 
    pub wallet: String, 
}