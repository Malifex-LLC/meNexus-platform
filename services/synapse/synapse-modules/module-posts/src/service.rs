// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::config::load_config;
use crate::errors::ModulePostsError;
use crate::types::CreatePostRequest;
use crate::types::ListPostsForChannelRequest;
use crate::types::Post;
use crate::types::PostsDeps;
use crate::types::PostsModuleConfig;
use synapse_application::events::{CreateEventCommand, CreateRemoteEventCommand};
use synapse_core::ports::events::event_repository::EventFilter;

pub async fn list_posts(deps: PostsDeps) -> Result<Vec<Post>, ModulePostsError> {
    let events = deps
        .repo
        .retrieve(EventFilter {
            event_type: Some("posts:create_post".to_string()),
            module_kind: None,
            module_slug: None,
        })
        .await
        .unwrap();

    let mut posts: Vec<Post> = Vec::new();

    for event in events {
        let profile = deps
            .profile_repo
            .get_profile(&event.agent)
            .await
            .unwrap()
            .unwrap();
        posts.push(Post {
            id: event.id.to_string(),
            author_public_key: event.agent.clone(),
            author_name: profile.display_name.unwrap().clone(),
            author_handle: profile.handle.unwrap().clone(),
            author_avatar: "AvatarPath".to_string(),
            timestamp: "TimeStamp".to_string(),
            content: event.content.unwrap_or_default(),
            posted_in: event.module_slug.unwrap(),
            likes: 0,
            comments: 0,
            liked: false,
        });
    }
    Ok(posts)
}

pub async fn list_posts_for_channel(
    deps: PostsDeps,
    request: ListPostsForChannelRequest,
) -> Result<Vec<Post>, ModulePostsError> {
    let events = deps
        .repo
        .retrieve(EventFilter {
            event_type: Some("posts:create_post".to_string()),
            module_kind: None,
            module_slug: Some(request.channel),
        })
        .await
        .unwrap();

    let mut posts: Vec<Post> = Vec::new();

    for event in events {
        let profile = deps
            .profile_repo
            .get_profile(&event.agent)
            .await
            .unwrap()
            .unwrap();
        posts.push(Post {
            id: event.id.to_string(),
            author_public_key: event.agent.clone(),
            author_name: profile.display_name.unwrap().clone(),
            author_handle: profile.handle.unwrap().clone(),
            author_avatar: "AvatarPath".to_string(),
            timestamp: "TimeStamp".to_string(),
            content: event.content.unwrap_or_default(),
            posted_in: event.module_slug.unwrap(),
            likes: 0,
            comments: 0,
            liked: false,
        });
    }

    Ok(posts)
}

pub async fn create_post(
    deps: PostsDeps,
    request: CreatePostRequest,
) -> Result<Post, ModulePostsError> {
    let cmd = CreateEventCommand {
        event_type: "posts:create_post".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: request.module_slug,
        agent: request.agent,
        target: request.target,
        previous: request.previous,
        content: request.content,
        artifacts: request.artifacts,
        metadata: request.metadata,
        links: request.links,
        data: request.data,
        expiration: request.expiration,
        agent_signature: request.agent_signature,
    };
    let event = deps.create_local_event.execute(cmd).await?;

    let profile = deps
        .profile_repo
        .get_profile(&event.agent)
        .await
        .unwrap()
        .unwrap();
    let post = Post {
        id: event.id.to_string(),
        author_public_key: event.agent.clone(),
        author_name: profile.display_name.unwrap().clone(),
        author_handle: profile.handle.unwrap().clone(),
        author_avatar: "AvatarPath".to_string(),
        timestamp: "TimeStamp".to_string(),
        content: event.content.unwrap_or_default(),
        posted_in: event.module_slug.unwrap(),
        likes: 0,
        comments: 0,
        liked: false,
    };

    Ok(post)
}

/// Get the posts module configuration
pub fn get_posts_config() -> Result<PostsModuleConfig, ModulePostsError> {
    let config = load_config()?;
    Ok(PostsModuleConfig {
        channels: config.channels,
        default_channel: config.default_channel,
        max_post_length: config.max_post_length,
    })
}

// =============================================================================
// Remote Synapse Service Functions
// =============================================================================

/// Get posts module configuration from a remote synapse
pub async fn get_remote_posts_config(
    deps: PostsDeps,
    synapse_public_key: String,
) -> Result<PostsModuleConfig, ModulePostsError> {
    let inner = CreateEventCommand {
        event_type: "posts:get_config".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: None,
        agent: "guest".to_string(), // Read operations don't require specific identity
        target: None,
        previous: None,
        content: None,
        artifacts: None,
        metadata: None,
        links: None,
        data: None,
        expiration: None,
        agent_signature: None, // Read operations don't require signature
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
        event: inner,
    };

    let events = deps.create_remote_event.execute(cmd).await?;

    // Parse the config from the first event's data field
    if let Some(event) = events.first() {
        if let Some(data) = &event.data {
            if let Ok(config) = serde_json::from_slice::<PostsModuleConfig>(data) {
                return Ok(config);
            }
        }
    }

    // Fallback to default config if remote didn't return valid config
    Ok(PostsModuleConfig::default())
}

/// Create a post on a remote synapse
pub async fn create_remote_post(
    deps: PostsDeps,
    synapse_public_key: String,
    request: CreatePostRequest,
) -> Result<Post, ModulePostsError> {
    let inner = CreateEventCommand {
        event_type: "posts:create_post".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: request.module_slug,
        agent: request.agent.clone(),
        target: request.target,
        previous: request.previous,
        content: request.content,
        artifacts: request.artifacts,
        metadata: request.metadata,
        links: request.links,
        data: request.data,
        expiration: request.expiration,
        agent_signature: request.agent_signature, // Required for remote post authentication
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
        event: inner,
    };

    let events = deps.create_remote_event.execute(cmd).await?;

    // The remote synapse should return the created event
    if let Some(event) = events.first() {
        // Try to get profile info for the author
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

        Ok(Post {
            id: event.id.to_string(),
            author_public_key: event.agent.clone(),
            author_name,
            author_handle,
            author_avatar: "AvatarPath".to_string(),
            timestamp: "TimeStamp".to_string(),
            content: event.content.clone().unwrap_or_default(),
            posted_in: event.module_slug.clone().unwrap_or_default(),
            likes: 0,
            comments: 0,
            liked: false,
        })
    } else {
        Err(ModulePostsError::Internal(
            "Remote synapse did not return created post".to_string(),
        ))
    }
}

/// List posts for a specific channel from a remote synapse
pub async fn list_remote_posts_for_channel(
    deps: PostsDeps,
    synapse_public_key: String,
    channel: String,
) -> Result<Vec<Post>, ModulePostsError> {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert("channel".to_string(), channel);

    let inner = CreateEventCommand {
        event_type: "posts:list_posts_for_channel".to_string(),
        module_kind: Some("posts".to_string()),
        module_slug: None,
        agent: "guest".to_string(), // Read operations don't require specific identity
        target: None,
        previous: None,
        content: None,
        artifacts: None,
        metadata: Some(metadata),
        links: None,
        data: None,
        expiration: None,
        agent_signature: None, // Read operations don't require signature
    };

    let cmd = CreateRemoteEventCommand {
        synapse_public_key,
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

    Ok(posts)
}
