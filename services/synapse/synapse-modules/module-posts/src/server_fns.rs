// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

use crate::types::{CreatePostRequest, ListPostsForChannelRequest, Post};

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
