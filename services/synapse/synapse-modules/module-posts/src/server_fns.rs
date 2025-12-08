// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::server::PostsDeps;
#[cfg(feature = "ssr")]
use synapse_core::ports::events::event_repository::EventFilter;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: String,
    pub author_public_key: String,
    pub author_name: String,
    pub author_handle: String,
    pub author_avatar: String,
    pub timestamp: String,
    pub content: String,
    pub posted_in: String,
    pub likes: u32,
    pub comments: u32,
    pub liked: bool,
}

#[server(ListPosts, "/api/posts/list")]
pub async fn list_posts() -> Result<Vec<Post>, ServerFnError> {
    let deps: PostsDeps = expect_context();

    let events = deps
        .repo
        .retrieve(EventFilter {
            event_type: Some("posts:create_post".to_string()),
            module_kind: None,
            module_slug: None,
        })
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let mut posts: Vec<Post> = events
        .into_iter()
        .map(|event| Post {
            id: event.id.to_string(),
            author_public_key: event.agent.clone(),
            author_name: event.agent.clone(),
            author_handle: event.agent.clone(),
            author_avatar: "AvatarPath".to_string(),
            timestamp: "TimeStamp".to_string(),
            content: event.content.unwrap_or_default(),
            posted_in: event.module_slug.unwrap(),
            likes: 0,
            comments: 0,
            liked: false,
        })
        .collect();

    Ok(posts)
}

#[server(ListPostsForChannel, "/api/posts/list_for_channel")]
pub async fn list_posts_for_channel(channel: String) -> Result<Vec<Post>, ServerFnError> {
    let deps: PostsDeps = expect_context();

    let events = deps
        .repo
        .retrieve(EventFilter {
            event_type: Some("posts:create_post".to_string()),
            module_kind: None,
            module_slug: Some(channel),
        })
        .await
        .map_err(|e| ServerFnError::new(format!("Database error: {}", e)))?;

    let mut posts: Vec<Post> = events
        .into_iter()
        .map(|event| Post {
            id: event.id.to_string(),
            author_public_key: event.agent.clone(),
            author_name: event.agent.clone(),
            author_handle: event.agent.clone(),
            author_avatar: "AvatarPath".to_string(),
            timestamp: "TimeStamp".to_string(),
            content: event.content.unwrap_or_default(),
            posted_in: event.module_slug.unwrap(),
            likes: 0,
            comments: 0,
            liked: false,
        })
        .collect();

    Ok(posts)
}

// #[server(ListPosts, "/api/posts/list")]
// pub async fn list_posts() -> Result<Vec<Post>, ServerFnError> {
//     let mut posts: Vec<Post> = vec![Post {
//         id: "ID".to_string(),
//         author_public_key: "Agent".to_string(),
//         author_name: "Authoer".to_string(),
//         author_handle: "Handle".to_string(),
//         author_avatar: "AvatarPath".to_string(),
//         timestamp: "TimeStamp".to_string(),
//         content: "COntent".to_string(),
//         posted_in: "PostedIn".to_string(),
//         likes: 0,
//         comments: 0,
//         liked: false,
//     }];
//
//     Ok(posts)
// }
