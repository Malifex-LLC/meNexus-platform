// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod domain;
pub mod errors;
pub mod handlers;

use std::sync::Arc;

use async_trait::async_trait;
use axum::{Json, extract::Path, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use synapse_application::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use synapse_config::get_synapse_config;
use synapse_core::domain::events::ObjectRef;
use synapse_core::{
    CoreError, domain::events::Event, ports::events::event_repository::EventRepository,
    ports::modules::Module,
};
use time::OffsetDateTime;
use tracing::debug;
use uuid::Uuid;

use crate::errors::ModulePostsError;

// implement module port
pub struct PostsModule {
    kind: String,
    version: String,
    repo: Arc<dyn EventRepository>,
}

impl PostsModule {
    pub fn new(repo: Arc<dyn EventRepository>) -> Self {
        Self {
            kind: "posts".to_string(),
            version: "1.0.0".to_string(),
            repo,
        }
    }
}

#[async_trait]
impl Module for PostsModule {
    fn kind(&self) -> Result<String, CoreError> {
        Ok(self.kind.clone())
    }
    fn version(&self) -> Result<String, CoreError> {
        Ok(self.version.clone())
    }
    async fn handle_event(&self, event: &Event) -> Result<Option<Vec<Event>>, CoreError> {
        match event.event_type.as_str() {
            "posts:create_post" => create_post(event).await,
            "posts:list_posts" => {
                let posts = self
                    .repo
                    .retrieve("posts:create_post".to_string())
                    .await
                    .unwrap();
                Ok(posts)
            }
            _ => Ok(None),
        }
    }
}

#[derive(Clone)]
pub struct PostsDeps {
    pub repo: std::sync::Arc<dyn synapse_core::ports::events::event_repository::EventRepository>,
    pub create_local_event: Arc<dyn CreateLocalEventUseCase + Send + Sync>,
    pub create_remote_event: Arc<dyn CreateRemoteEventUseCase + Send + Sync>,
}

pub fn routes<S>() -> axum::Router<S>
where
    S: Clone + Send + Sync + 'static,
    PostsDeps: axum::extract::FromRef<S>,
{
    use axum::routing::{get, post};
    axum::Router::new()
        .route("/posts", get(list_posts_http))
        .route("/posts", post(create_post_http))
        .route(
            "/synapses/{synapse_public_key}/posts",
            get(list_remote_events),
        )
        .route(
            "/synapses/{synapse_public_key}/posts",
            post(create_post_remote),
        )
}

#[derive(Deserialize)]
struct ListPostsRequest {
    event_type: String,
}

#[derive(Serialize)]
struct ListPostsResult {
    posts: Option<Vec<Event>>,
}

async fn list_posts_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    Json(body): Json<ListPostsRequest>,
) -> Result<(StatusCode, Json<ListPostsResult>), ModulePostsError> {
    let results = deps
        .repo
        .retrieve("posts:create_post".to_string())
        .await
        .unwrap();
    Ok((
        StatusCode::CREATED,
        Json(ListPostsResult { posts: results }),
    ))
}

#[derive(Deserialize)]
struct ListRemotePostsRequest {
    event_type: String,
}

#[derive(Serialize)]
struct ListRemotePostsResult {
    events: Option<Vec<Event>>,
}

async fn list_remote_events(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    Path(synapse_public_key): Path<String>,
    Json(body): Json<ListRemotePostsRequest>,
) -> Result<(StatusCode, Json<ListRemotePostsResult>), ModulePostsError> {
    let inner = CreateEventCommand {
        event_type: "posts:list_posts".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: None,
        agent: "100".to_string(),
        target: None,
        previous: None,
        content: None,
        artifacts: None,
        metadata: None,
        links: None,
        data: None,
        expiration: None,
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
        event: inner,
    };

    let results = deps.create_remote_event.execute(cmd).await?;

    Ok((
        StatusCode::CREATED,
        Json(ListRemotePostsResult { events: results }),
    ))
}

#[derive(Deserialize)]
struct CreatePostRequest {
    event_type: String,
    agent: String,
    module_kind: Option<String>,
    module_slug: Option<String>,
    target: Option<ObjectRef>,
    previous: Option<Uuid>,
    content: Option<String>,
    artifacts: Option<Vec<String>>,
    metadata: Option<HashMap<String, String>>,
    links: Option<Vec<String>>,
    data: Option<Vec<u8>>,
    expiration: Option<OffsetDateTime>,
}

async fn create_post_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    axum::Json(body): axum::Json<CreatePostRequest>,
) -> Result<
    (
        axum::http::StatusCode,
        axum::Json<synapse_core::domain::events::Event>,
    ),
    ModulePostsError,
> {
    use synapse_application::events::CreateEventCommand;

    let cmd = CreateEventCommand {
        event_type: "posts:create_post".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: body.module_slug,
        agent: "local-agent".to_string(), // TODO set real agent public key
        target: body.target,
        previous: body.previous,
        content: body.content,
        artifacts: body.artifacts,
        metadata: body.metadata,
        links: body.links,
        data: body.data,
        expiration: body.expiration,
    };

    let created = deps.create_local_event.execute(cmd).await?;
    Ok((axum::http::StatusCode::CREATED, axum::Json(created)))
}

async fn create_post_remote(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    axum::extract::Path(synapse_public_key): axum::extract::Path<String>,
    axum::Json(body): axum::Json<CreatePostRequest>,
) -> Result<
    (
        axum::http::StatusCode,
        axum::Json<Option<Vec<synapse_core::domain::events::Event>>>,
    ),
    axum::http::StatusCode,
> {
    use synapse_application::events::{CreateEventCommand, CreateRemoteEventCommand};

    let inner = CreateEventCommand {
        event_type: "posts:create_post".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: body.module_slug,
        agent: "local-agent".to_string(), // TODO set real agent public key
        target: body.target,
        previous: body.previous,
        content: body.content,
        artifacts: body.artifacts,
        metadata: body.metadata,
        links: body.links,
        data: body.data,
        expiration: body.expiration,
    };
    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
        event: inner,
    };

    let res = deps
        .create_remote_event
        .execute(cmd)
        .await
        .map_err(|_| axum::http::StatusCode::BAD_GATEWAY)?;
    Ok((axum::http::StatusCode::CREATED, axum::Json(res)))
}

async fn create_post(event: &Event) -> Result<Option<Vec<Event>>, CoreError> {
    // TODO write Event validation and other side effect logic
    debug!("posts:create_post called!");
    let config = get_synapse_config().unwrap();
    let public_key = config.identity.public_key;
    let res_event = Event::new()
        .with_event_type("posts:post_created")
        .with_module_kind("core")
        .with_agent(public_key.clone())
        .with_content(public_key.clone())
        .build();
    let events = vec![res_event];
    Ok(Some(events))
}
