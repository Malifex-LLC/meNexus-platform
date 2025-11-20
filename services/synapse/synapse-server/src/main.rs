// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod api;
pub mod errors;
pub mod middleware;
pub mod state;
pub mod utils;

use crate::state::AppState;
use adapter_libp2p::initialize_p2p;

use adapter_postgres::events_repository::PostgresEventsRepository;
use adapter_postgres::{create_pool, migrate};
use synapse_application::events::CreateLocalEventUseCase;
use synapse_application::events::event_service::EventIngestService;
use synapse_application::events::event_service::{LocalEventService, RemoteEventService};
use synapse_config::get_synapse_config;

use dashmap::DashMap;
use std::collections::HashMap;
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use time::format_description;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt::time::LocalTime;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        //.compact()
        .with_thread_names(true)
        .with_target(true)
        .with_level(true)
        .with_line_number(true)
        .with_file(true)
        .with_ansi(true)
        .pretty()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let database_url = std::env::var("DATABASE_URL")?;
    let pool = create_pool(&database_url).await?;
    migrate(&pool).await?;

    let repo = Arc::new(PostgresEventsRepository::new(pool.clone()));
    let ingest = Arc::new(EventIngestService::new(repo));

    let known_peers = Arc::new(DashMap::<String, String>::new());

    let config = get_synapse_config()?;
    let transport = initialize_p2p(config, ingest.clone(), known_peers.clone()).await?;

    let create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync> =
        Arc::new(LocalEventService::new(ingest.clone()));

    let create_remote_event = Arc::new(RemoteEventService::new(Arc::new(transport)));
    let state = AppState {
        create_local_event,
        create_remote_event,
        known_peers: known_peers.clone(),
    };

    let app = api::routes()
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let port: u16 = env::var("AXUM_PORT")?
        .parse()
        .expect("Failed to parse AXUM_PORT");
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let listener = tokio::net::TcpListener::bind(socket).await?;
    info!("Server running on http://{socket}");
    axum::serve(listener, app).await?;

    Ok(())
}
