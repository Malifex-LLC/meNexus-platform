// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod api;
pub mod errors;
pub mod middleware;
pub mod state;
pub mod utils;

use axum::extract::FromRef;

use crate::state::AppState;
use adapter_libp2p::initialize_p2p;

use adapter_postgres::auth_repository::PostgresAuthRepository;
use adapter_postgres::crypto_repository::PostgresCryptoRepository;
use adapter_postgres::events_repository::PostgresEventsRepository;
use adapter_postgres::profiles_repository::{PostgresProfilesDocStore, PostgresProfilesRepository};
use adapter_postgres::{create_pool, migrate};
use client_web::app::Shell;
use dashmap::DashMap;
use leptos::config::LeptosOptions;
use leptos::prelude::provide_context;
use leptos::view;
use leptos_axum::{LeptosRoutes, generate_route_list};
use module_auth::http::AuthModule;
use module_auth::http::routes as module_auth_routes;
use module_auth::types::AuthDeps;
use module_core::CoreModule;
use module_posts::PostsModule;
use module_posts::{PostsDeps, routes as module_posts_routes};
use module_profiles::types::ProfilesDeps;
use module_profiles::{ProfilesModule, routes as module_profiles_routes};
use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use synapse_application::events::CreateLocalEventUseCase;
use synapse_application::events::event_service::EventIngestService;
use synapse_application::events::event_service::{LocalEventService, RemoteEventService};
use synapse_application::modules::InMemoryModuleRegistry;
use synapse_application::profiles::profile_service::ProfileDiscoveryTransport;
use synapse_config::get_synapse_config;
use synapse_core::ports::modules::ModuleRegistry;
use synapse_core::ports::profiles::profile_repository::ProfileDiscovery;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;
use tracing::info;

impl axum::extract::FromRef<AppState> for AuthDeps {
    fn from_ref(app: &AppState) -> Self {
        AuthDeps {
            crypto_repo: app.crypto_repo.clone(),
            session_repo: app.session_repo.clone(),
        }
    }
}

impl axum::extract::FromRef<AppState> for ProfilesDeps {
    fn from_ref(app: &AppState) -> Self {
        ProfilesDeps {
            doc_store: app.profile_doc_store.clone(),
            profile_repo: app.profile_repo.clone(),
            profile_discovery: app.profile_discovery.clone(),
            create_local_event: app.create_local_event.clone(),
            create_remote_event: app.create_remote_event.clone(),
        }
    }
}

impl axum::extract::FromRef<AppState> for PostsDeps {
    fn from_ref(app: &AppState) -> Self {
        PostsDeps {
            repo: app.event_repo.clone(),
            create_local_event: app.create_local_event.clone(),
            create_remote_event: app.create_remote_event.clone(),
        }
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

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

    let module_registry = Arc::new(InMemoryModuleRegistry::new());
    let event_repo = Arc::new(PostgresEventsRepository::new(pool.clone()));
    let crypto_repo = Arc::new(PostgresCryptoRepository::new(pool.clone()));
    let session_repo = Arc::new(PostgresAuthRepository::new(pool.clone()));
    let ingest = Arc::new(EventIngestService::new(
        event_repo.clone(),
        module_registry.clone(),
    ));

    let profile_repo = Arc::new(PostgresProfilesRepository::new(pool.clone()));
    let profile_doc_store = Arc::new(PostgresProfilesDocStore::new(pool.clone()));

    module_registry.register(Arc::new(CoreModule::new(event_repo.clone())))?;
    module_registry.register(Arc::new(AuthModule::new(
        session_repo.clone(),
        crypto_repo.clone(),
    )))?;
    module_registry.register(Arc::new(ProfilesModule::new(
        profile_repo.clone(),
        profile_doc_store.clone(),
    )))?;
    module_registry.register(Arc::new(PostsModule::new(event_repo.clone())))?;

    let known_peers = Arc::new(DashMap::<String, String>::new());

    let config = get_synapse_config()?;
    let transport =
        Arc::new(initialize_p2p(config.clone(), ingest.clone(), known_peers.clone()).await?);

    let profile_discovery = Arc::new(ProfileDiscoveryTransport::new(transport.clone()));

    let create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync> =
        Arc::new(LocalEventService::new(ingest.clone()));

    let create_remote_event = Arc::new(RemoteEventService::new(transport.clone()));

    let leptos_options = client_web::leptos_options();

    let state = AppState {
        event_repo: event_repo.clone(),
        crypto_repo: crypto_repo.clone(),
        session_repo: session_repo.clone(),
        profile_doc_store: profile_doc_store.clone(),
        profile_repo: profile_repo.clone(),
        profile_discovery: profile_discovery.clone(),
        create_local_event,
        create_remote_event,
        known_peers: known_peers.clone(),
        leptos_options: leptos_options.clone(),
    };

    let posts_deps = PostsDeps::from_ref(&state);
    let auth_deps = AuthDeps::from_ref(&state);
    let profile_deps = ProfilesDeps::from_ref(&state);

    let routes = generate_route_list({
        let opts = leptos_options.clone();
        move || view! { <Shell options=opts.clone()/> }
    });
    let static_site_dir = PathBuf::from(leptos_options.site_root.as_ref());
    let app = api::routes()
        .merge(module_auth_routes::<AppState>())
        .merge(module_posts_routes::<AppState>())
        .merge(module_profiles_routes::<AppState>())
        .leptos_routes_with_context(
            &state,
            routes,
            {
                move || {
                    // Provide deps to server functions via context
                    provide_context(posts_deps.clone());
                    provide_context(auth_deps.clone());
                    provide_context(profile_deps.clone());
                }
            },
            {
                let opts = leptos_options.clone();
                move || view! { <Shell options=opts.clone()/> }
            },
        )
        .fallback_service(ServeDir::new(static_site_dir))
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
