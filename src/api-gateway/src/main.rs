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

#[derive(Deserialize)]
struct InferenceRequest {
    prompt: String,
    max_tokens: Option<u32>,
    temperature: Option<f32>
}

#[derive(Serialize)]
struct InferenceResponse {
    generated_text: String,
    latency_ms: u64,
    tokens_generated: u32,
    model_version: String
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    total_requests: u64,
    active_connections: usize,
}


// health check point
async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    let total_requests: u64 = state.request_count.load(std::sync::atomic::Ordering::Relaxed);

    let active_connections = state.inference_semaphore.available_permits();

    Json(HealthResponse { status: ("healthy".to_string()), version: ("0.1.0".to_string()), total_requests, active_connections:(100-active_connections) })

}

#[tokio::main]
async fn main(){
    
}


