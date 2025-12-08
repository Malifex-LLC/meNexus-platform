// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::{
    server_fns::{Post, list_posts_for_channel},
    ui::components::post_card::PostCard,
};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostsConfig {
    pub channels: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PostsConfigResponse {
    pub config: PostsConfig,
}

pub fn click(posts: Vec<Post>) {
    leptos::logging::log!("Button clicked!");
    leptos::logging::log!("Posts fetched: {:#?}", posts);
}

#[component]
pub fn PostsFeed() -> impl IntoView {
    let channels = vec![
        "general".to_string(),
        "memes".to_string(),
        "support".to_string(),
    ];
    let (active_channel, set_active_channel) = signal(channels[0].clone());

    let posts = Resource::new(
        move || active_channel.get(), // Track active_channel changes
        |channel: String| async move { list_posts_for_channel(channel).await.unwrap_or_default() },
    );

    view! {
        <Suspense fallback=|| view! { <div>"Loading posts..."</div> }>
            <div class="flex p-4 gap-4 bg-panel text-foreground h-full w-full">
                <div>
                    <ul>
                        {channels.iter().map(|channel| {
                            let channel_for_active = channel.clone();
                            let channel_for_inactive = channel.clone();
                            let channel_for_click = channel.clone();

                            view! {
                                <li>
                                    <button
                                        class=move || {
                                            let base = "px-4 py-4 my-4 rounded transition-colors hover:cursor-pointer";
                                            if active_channel.get() == channel_for_active {
                                                format!("{} bg-brand text-white font-bold", base)
                                            } else {
                                                format!("{} bg-gray-200 text-gray-700", base)
                                            }
                                        }
                                        on:click=move |_| {
                                            set_active_channel.set(channel_for_click.clone());
                                        }
                                    >
                                        {channel.clone()}
                                    </button>
                                </li>
                            }
                        }).collect_view()}
                    </ul>
                </div>
                <div class="w-full">
                    <ul class="w-full">
                        {move || posts.get().map(|posts_list| view! {
                            {posts_list.iter().map(|post| view! {
                                <li class="my-4 w-full">
                                    <PostCard
                                    id=post.id.clone()
                                    author_public_key=post.author_public_key.clone()
                                    author_name=post.author_name.clone()
                                    author_handle=post.author_handle.clone()
                                    author_avatar=post.author_avatar.clone()
                                    timestamp=post.timestamp.clone()
                                    content=post.content.clone()
                                    posted_in=post.posted_in.clone()
                                    likes=post.likes
                                    comments=post.comments
                                    liked=post.liked
                                />
                                </li>
                            }).collect_view()}
                        })}
                    </ul>
                </div>
            </div>
        </Suspense>
    }
}
