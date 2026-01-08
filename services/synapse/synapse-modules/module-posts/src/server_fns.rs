// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

use crate::types::{CreatePostRequest, ListPostsForChannelRequest, Post, PostsModuleConfig};

#[cfg(feature = "ssr")]
use crate::types::PostsDeps;

#[cfg(feature = "ssr")]
#[server(ListPosts, "/api/posts")]
pub async fn list_posts_server() -> Result<Vec<Post>, ServerFnError> {
    use crate::service::list_posts;
    let deps: PostsDeps = expect_context();
    let posts = list_posts(deps).await.unwrap();
    Ok(posts)
}

#[server(ListPostsForChannel, "/api/posts")]
pub async fn list_posts_for_channel_server(
    request: ListPostsForChannelRequest,
) -> Result<Vec<Post>, ServerFnError> {
    use crate::service::list_posts_for_channel;
    let deps: PostsDeps = expect_context();
    let posts = list_posts_for_channel(deps, request).await.unwrap();
    Ok(posts)
}

#[server(CreatePostServer, "/api/posts")]
pub async fn create_post_server(request: CreatePostRequest) -> Result<Post, ServerFnError> {
    use crate::service::create_post;
    let deps: PostsDeps = expect_context();
    let post = create_post(deps, request).await.unwrap();
    Ok(post)
}

/// Get the posts module configuration (channels, default channel, max post length)
#[server(GetPostsConfig, "/api/posts")]
pub async fn get_posts_config_server() -> Result<PostsModuleConfig, ServerFnError> {
    use crate::service::get_posts_config;
    let config = get_posts_config().map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(config)
}

// =============================================================================
// Remote Synapse Server Functions
// =============================================================================

/// Get posts module configuration from a remote synapse
#[server(GetRemotePostsConfig, "/api/posts")]
pub async fn get_remote_posts_config_server(
    synapse_public_key: String,
) -> Result<PostsModuleConfig, ServerFnError> {
    use crate::service::get_remote_posts_config;
    let deps: PostsDeps = expect_context();
    let config = get_remote_posts_config(deps, synapse_public_key)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(config)
}

/// List posts for a specific channel from a remote synapse
#[server(ListRemotePostsForChannel, "/api/posts")]
pub async fn list_remote_posts_for_channel_server(
    synapse_public_key: String,
    channel: String,
) -> Result<Vec<Post>, ServerFnError> {
    use crate::service::list_remote_posts_for_channel;
    let deps: PostsDeps = expect_context();
    let posts = list_remote_posts_for_channel(deps, synapse_public_key, channel)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(posts)
}
