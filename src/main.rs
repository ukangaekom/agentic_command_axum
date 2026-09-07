mod fuzz;


use crate::fuzz::content::fuzzy_ipi_content;
use axum::{Router, routing::get};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // dotenv_flow::dotenv_flow().ok();

    // Define CORS layer
    /*
    This endpoint is free for now as it's in demo and testing phase
    */
    let cors = CorsLayer::new()
        .allow_origin(Any) // 👈 Allow requests from any domain
        .allow_methods(Any)
        .allow_headers(Any);

    // Axum router
    let ipi_app: Router = Router::new()
        .route("/agent", get(fuzzy_ipi_content))
        .layer(cors);

    // Define Ip and Port
    let address: &'static str = "0.0.0.0:3000";
    let listener: TcpListener = tokio::net::TcpListener::bind(address).await.unwrap();

    println!("Listener on {address}\n");

    axum::serve(listener, ipi_app).await.unwrap();
}
