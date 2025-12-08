// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

#[cfg(feature = "hydrate")]
#[component]
pub fn PostCard(
    #[prop(into)] id: String,
    #[prop(into)] author_public_key: String,
    #[prop(into)] author_name: String,
    #[prop(into)] author_handle: String,
    #[prop(into)] author_avatar: String,
    #[prop(into)] timestamp: String,
    #[prop(into)] content: String,
    #[prop(into)] posted_in: String,
    #[prop(into)] likes: u32,
    #[prop(into)] comments: u32,
    #[prop(into)] liked: bool,
) -> impl IntoView {
    view! {
        <div class="p-4 bg-card border border-border w-full">
            <div>{id}</div>
            <div>{author_public_key}</div>
            <div>{author_name}</div>
            <div>{author_handle}</div>
            <div>{author_avatar}</div>
            <div>{timestamp}</div>
            <div>{content}</div>
            <div>{posted_in}</div>
            <div>{likes}</div>
            <div>{comments}</div>
            <div>{liked}</div>
        </div>
    }
}
