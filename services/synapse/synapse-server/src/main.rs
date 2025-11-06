// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod api;
pub mod errors;
pub mod middleware;
pub mod state;
pub mod utils;

use crate::state::AppState;
use adapter_postgres::events_repository::PostgresEventsRepository;
use adapter_postgres::{create_pool, migrate};
use synapse_application::events::event_service::EventService;
use synapse_application::federation::federation_service::initialize_p2p;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;
    migrate(&pool).await?;

    initialize_p2p().await;

    let repo = Arc::new(PostgresEventsRepository::new(pool.clone()));
    let create_event = Arc::new(EventService::new(repo));
    let state = AppState { create_event };

    let app = api::routes()
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let port = 3000;
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let listener = tokio::net::TcpListener::bind(socket).await.unwrap();
    info!("Server running on http://{socket}");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
