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
    pub date: String,
    pub amount: Decimal,
    pub account: Option<String>,
}

pub async fn ping() -> &'static str {
    "Pong"
}

pub async fn print() -> &'static str {
    "Pong"
}

pub async fn monthly(
    Path(account): Path<String>,
    Extension(state): Extension<Arc<Mutex<Model>>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    let response = &model.monthly(account).await;
    println!("{}", json!(response));
    Json(json!(response))
}
