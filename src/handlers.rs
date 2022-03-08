use crate::Model;
use axum::extract::Extension;
use axum::extract::Path;
use axum::Json;
use rust_decimal::Decimal;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct LedgerResponse {
    pub date: Option<String>,
    pub amount: Decimal,
    pub account: Option<String>,
}

pub async fn ping() -> &'static str {
    "Pong"
}

pub async fn print(Extension(state): Extension<Arc<Mutex<Model>>>) -> String {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    format!("{}", model.ledger)
}

pub async fn cashflow(
    Path(account): Path<String>,
    Extension(state): Extension<Arc<Mutex<Model>>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    let response = &model.cashflow(account).await;
    Json(json!(response))
}

pub async fn monthly(
    Path(account): Path<String>,
    Extension(state): Extension<Arc<Mutex<Model>>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    let response = &model.monthly(account).await;
    Json(json!(response))
}


pub async fn split(
    Path(account): Path<String>,
    Extension(state): Extension<Arc<Mutex<Model>>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    let response = &model.split(account).await;
    Json(json!(response))
}

