// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::errors::ModulePostsError;
use crate::types::CreatePostRequest;
use crate::types::ListPostsForChannelRequest;
use crate::types::Post;
use crate::types::PostsDeps;
use synapse_application::events::CreateEventCommand;
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
