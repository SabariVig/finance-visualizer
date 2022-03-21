use crate::Model;
use axum::{
    extract::{Extension, Path, Query},
    Json,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct LedgerResponse {
    pub date: Option<String>,
    pub amount: Decimal,
    pub account: Option<String>,
}

#[derive(Deserialize)]
pub struct NativeCurrency {
    convert_commodity: bool,
}

// pub async fn ping() -> &'static str {
//     "Pong"
// }

pub async fn ping(Extension(state): Extension<Arc<Mutex<Model>>>) -> &'static str {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    model.sort_by_date();
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
    native_currency: Option<Query<NativeCurrency>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    let Query(native_currency) = native_currency.unwrap_or(Query(NativeCurrency {
        convert_commodity: false,
    }));
    let response = &model
        .cashflow(account, native_currency.convert_commodity)
        .await;
    Json(json!(response))
}

pub async fn monthly(
    Path(account): Path<String>,
    native_currency: Option<Query<NativeCurrency>>,
    Extension(state): Extension<Arc<Mutex<Model>>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();
    let Query(native_currency) = native_currency.unwrap_or(Query(NativeCurrency {
        convert_commodity: false,
    }));
    let response = &model
        .monthly(account, native_currency.convert_commodity)
        .await;
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

pub async fn balance(
    Path(account): Path<String>,
    native_currency: Option<Query<NativeCurrency>>,
    Extension(state): Extension<Arc<Mutex<Model>>>,
) -> Json<Value> {
    let state = Arc::clone(&state);
    let mut model = state.lock().await;
    model.reload_file().unwrap();

    model.reload_file().unwrap();
    let Query(native_currency) = native_currency.unwrap_or(Query(NativeCurrency {
        convert_commodity: false,
    }));
    let response = &model
        .balance(account, native_currency.convert_commodity)
        .await;

    Json(json!(response))
}
