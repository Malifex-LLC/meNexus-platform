// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use std::sync::Arc;

use async_trait::async_trait;
use axum::{Json, extract::Path, http::StatusCode};
use synapse_application::events::{
    CreateEventCommand, CreateLocalEventUseCase, CreateRemoteEventCommand, CreateRemoteEventUseCase,
};
use synapse_config::get_synapse_config;
use synapse_core::{
    CoreError,
    domain::events::Event,
    ports::events::event_repository::{EventFilter, EventRepository},
    ports::modules::Module,
};
use tracing::debug;

use crate::{
    errors::ModulePostsError,
    service::{create_post, get_posts_config, list_posts},
};
use crate::{
    service::list_posts_for_channel,
    types::{
        CreatePostRequest, GetPostsConfigResult, ListPostsForChannelRequest,
        ListPostsForChannelResult, ListPostsRequest, ListPostsResult, ListRemotePostsRequest,
        ListRemotePostsResult, Post, PostsDeps, PostsModuleConfig,
    },
};

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
    async fn handle_event(&self, event: &Event) -> Result<Vec<Event>, CoreError> {
        match event.event_type.as_str() {
            "posts:create_post" => create_post_handler(event).await,
            "posts:list_posts" => {
                let posts = self
                    .repo
                    .retrieve(EventFilter {
                        event_type: Some("posts:create_post".to_string()),
                        module_kind: None,
                        module_slug: None,
                    })
                    .await
                    .unwrap();
                Ok(posts)
            }
            "posts:list_posts_for_channel" => {
                // Extract channel from metadata
                let channel = event
                    .metadata
                    .as_ref()
                    .and_then(|m| m.get("channel"))
                    .cloned()
                    .unwrap_or_default();

                let posts = self
                    .repo
                    .retrieve(EventFilter {
                        event_type: Some("posts:create_post".to_string()),
                        module_kind: None,
                        module_slug: Some(channel),
                    })
                    .await
                    .unwrap();
                Ok(posts)
            }
            "posts:get_config" => {
                let config = get_posts_config().map_err(|e| CoreError::Other(e.to_string()))?;
                let config_bytes =
                    serde_json::to_vec(&config).map_err(|e| CoreError::Other(e.to_string()))?;
                let synapse_config = get_synapse_config().unwrap();
                let res_event = Event::new()
                    .with_event_type("posts:config")
                    .with_module_kind("posts")
                    .with_agent(synapse_config.identity.public_key.clone())
                    .with_data(config_bytes)
                    .build();
                Ok(vec![res_event])
            }
            _ => Ok(vec![]),
        }
    }
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
        .route("/posts/config", get(get_posts_config_http))
        .route("/posts/{channel}", get(list_posts_for_channel_http))
        .route(
            "/synapses/{synapse_public_key}/posts",
            get(list_remote_events),
        )
        .route(
            "/synapses/{synapse_public_key}/posts",
            post(create_post_remote),
        )
        .route(
            "/synapses/{synapse_public_key}/posts/config",
            get(get_posts_config_remote_http),
        )
        .route(
            "/synapses/{synapse_public_key}/posts/{channel}",
            get(list_posts_for_channel_remote_http),
        )
}

async fn list_posts_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    Json(body): Json<ListPostsRequest>,
) -> Result<(StatusCode, Json<ListPostsResult>), ModulePostsError> {
    let posts = list_posts(deps).await.unwrap();
    Ok((StatusCode::CREATED, Json(ListPostsResult { posts })))
}

async fn list_posts_for_channel_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    Json(body): Json<ListPostsForChannelRequest>,
) -> Result<(StatusCode, Json<ListPostsForChannelResult>), ModulePostsError> {
    let posts = list_posts_for_channel(deps, body).await.unwrap();
    Ok((
        StatusCode::CREATED,
        Json(ListPostsForChannelResult { posts }),
    ))
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

async fn create_post_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    axum::Json(body): axum::Json<CreatePostRequest>,
) -> Result<(axum::http::StatusCode, axum::Json<Post>), ModulePostsError> {
    let post = create_post(deps, body).await.unwrap();
    Ok((axum::http::StatusCode::CREATED, axum::Json(post)))
}

async fn create_post_remote(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    axum::extract::Path(synapse_public_key): axum::extract::Path<String>,
    axum::Json(body): axum::Json<CreatePostRequest>,
) -> Result<(axum::http::StatusCode, axum::Json<Vec<Event>>), axum::http::StatusCode> {
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

/// Get posts module configuration (local)
async fn get_posts_config_http() -> Result<(StatusCode, Json<GetPostsConfigResult>), ModulePostsError> {
    let config = get_posts_config()?;
    Ok((StatusCode::OK, Json(GetPostsConfigResult { config })))
}

/// Get posts module configuration from a remote synapse
async fn get_posts_config_remote_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    Path(synapse_public_key): Path<String>,
) -> Result<(StatusCode, Json<GetPostsConfigResult>), ModulePostsError> {
    let inner = CreateEventCommand {
        event_type: "posts:get_config".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: None,
        agent: "local-agent".to_string(),
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

    // Parse the config from the first event's data field
    if let Some(event) = results.first() {
        if let Some(data) = &event.data {
            if let Ok(config) = serde_json::from_slice::<PostsModuleConfig>(data) {
                return Ok((StatusCode::OK, Json(GetPostsConfigResult { config })));
            }
        }
    }

    // Fallback to default config if remote didn't return valid config
    Ok((
        StatusCode::OK,
        Json(GetPostsConfigResult {
            config: PostsModuleConfig::default(),
        }),
    ))
}

/// List posts for a specific channel from a remote synapse
async fn list_posts_for_channel_remote_http(
    axum::extract::State(deps): axum::extract::State<PostsDeps>,
    Path((synapse_public_key, channel)): Path<(String, String)>,
) -> Result<(StatusCode, Json<Vec<Post>>), ModulePostsError> {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("channel".to_string(), channel);

    let inner = CreateEventCommand {
        event_type: "posts:list_posts_for_channel".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: None,
        agent: "local-agent".to_string(),
        target: None,
        previous: None,
        content: None,
        artifacts: None,
        metadata: Some(metadata),
        links: None,
        data: None,
        expiration: None,
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key: synapse_public_key.clone(),
        event: inner,
    };

    let events = deps.create_remote_event.execute(cmd).await?;

    // Convert events to Posts
    let mut posts = Vec::new();
    for event in events {
        // Try to get profile info for each post author
        let (author_name, author_handle) =
            if let Ok(Some(profile)) = deps.profile_repo.get_profile(&event.agent).await {
                (
                    profile.display_name.unwrap_or_else(|| "Unknown".to_string()),
                    profile.handle.unwrap_or_else(|| "unknown".to_string()),
                )
            } else {
                // Fallback: use truncated public key
                let short_pk = if event.agent.len() > 8 {
                    format!("{}...", &event.agent[..8])
                } else {
                    event.agent.clone()
                };
                (short_pk.clone(), short_pk)
            };

        posts.push(Post {
            id: event.id.to_string(),
            author_public_key: event.agent.clone(),
            author_name,
            author_handle,
            author_avatar: "AvatarPath".to_string(),
            timestamp: "TimeStamp".to_string(),
            content: event.content.unwrap_or_default(),
            posted_in: event.module_slug.unwrap_or_default(),
            likes: 0,
            comments: 0,
            liked: false,
        });
    }

    Ok((StatusCode::OK, Json(posts)))
}

async fn create_post_handler(event: &Event) -> Result<Vec<Event>, CoreError> {
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
    Ok(events)
}
