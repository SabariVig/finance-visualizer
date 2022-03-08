mod handlers;
mod model;
use std::sync::Arc;

use crate::model::Model;
use axum::{routing::get, AddExtensionLayer, Router};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut model = Model::new("/home/hawk/temp_ledger/ledger.complete")?;
    model.convert_to_currency("INR", vec!["USD"])?;

    let shared_shared = Arc::new(Mutex::new(model));
    let app = Router::new()
        .route("/ping", get(handlers::ping))
        .route("/print", get(handlers::print))
        .route("/monthly/:path", get(handlers::monthly))
        .route("/cashflow/:path", get(handlers::cashflow))
        .route("/split/:path", get(handlers::split))
        .layer(AddExtensionLayer::new(shared_shared));

    tracing::debug!("server started on port 8080");
    axum::Server::bind(&"0.0.0.0:8080".parse()?)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
