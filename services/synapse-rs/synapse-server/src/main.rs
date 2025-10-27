pub mod api;
pub mod errors;
pub mod middleware;
pub mod state;
pub mod utils;

use crate::state::AppState;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let state = AppState {};

    let app = api::routes()
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let port = 3002;
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    info!("Server running on http://{socket}");
    axum::serve(listener, app).await.unwrap();
}
