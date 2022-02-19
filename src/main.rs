mod handlers;
mod model;
use crate::model::Model;
// use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let model = Model::new("/home/hawk/temp_ledger/ledger.complete")?;
    let ledger = model.open("/home/hawk/temp_ledger/ledger.complete")?;
    println!("{}",ledger);

    // let app = Router::new().route("/ping", get(handlers::ping));
    // axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();
    Ok(())
}
