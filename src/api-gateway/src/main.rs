#![allow(unused)]

use axum::{
    routing::{get, post},
    Router, extract::State,
    response::Json,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tracing_subscriber;
use tracing;

#[derive(Clone)]
struct AppState {
    inference_semaphore: Arc<Semaphore>,
    request_count: Arc<std::sync::atomic::AtomicU64>,
}

#[tokio::main]
async fn main(){
    
}


