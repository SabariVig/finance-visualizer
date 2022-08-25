mod handlers;
mod model;
mod utils;
use std::{env, sync::Arc};

use crate::{model::Model, utils::shutdown_signal};
use axum::{routing::get, AddExtensionLayer, Router};
use tokio::sync::Mutex;
use tower_http::trace::TraceLayer;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let path = env::var("LEDGER_FILE").unwrap();
    let native_currency = env::var("NATIVE_CURRENCY").unwrap_or("INR".to_string());
    let foreign_currency = env::var("FOREIGN_CURRENCY").unwrap_or("USD".to_string());

    let mut model = Model::new(path)?;
    model.convert_to_currency(&native_currency, vec![&foreign_currency])?;

    let shared_shared = Arc::new(Mutex::new(model));
    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .route("/print", get(handlers::print))
        .route("/monthly/:path", get(handlers::monthly))
        .route("/cashflow/:path", get(handlers::cashflow))
        .route("/split/:path", get(handlers::split))
        .route("/balance/:path", get(handlers::balance))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(shared_shared));

    tracing::info!("server started on port 8080");
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
