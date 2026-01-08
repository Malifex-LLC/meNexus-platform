// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::TypingUser;
use leptos::prelude::*;

/// Typing indicator showing who's currently typing
#[component]
pub fn TypingIndicator(
    /// List of users who are currently typing
    #[prop(into)]
    typing_users: Signal<Vec<TypingUser>>,
) -> impl IntoView {
    view! {
        {move || {
            let users = typing_users.get();
            if users.is_empty() {
                return view! { <span></span> }.into_any();
            }

            let typing_text = format_typing_text(&users);

            view! {
                <div class="flex items-center gap-2 px-4 py-2 text-sm text-foreground/50 animate-fade-in">
                    // Animated dots
                    <div class="flex items-center gap-0.5">
                        <span class="w-1.5 h-1.5 bg-brand rounded-full animate-bounce" style="animation-delay: 0ms;"></span>
                        <span class="w-1.5 h-1.5 bg-brand rounded-full animate-bounce" style="animation-delay: 150ms;"></span>
                        <span class="w-1.5 h-1.5 bg-brand rounded-full animate-bounce" style="animation-delay: 300ms;"></span>
                    </div>

                    // Typing text
                    <span>
                        <span class="font-semibold text-brand">{typing_text.0}</span>
                        {typing_text.1}
                    </span>
                </div>
            }.into_any()
        }}
    }
}

/// Compact typing indicator (for smaller spaces)
#[component]
pub fn TypingIndicatorCompact(
    /// List of users who are currently typing
    #[prop(into)]
    typing_users: Signal<Vec<TypingUser>>,
) -> impl IntoView {
    view! {
        {move || {
            let users = typing_users.get();
            if users.is_empty() {
                return view! { <span></span> }.into_any();
            }

            view! {
                <div class="flex items-center gap-1.5 text-xs text-foreground/40">
                    // Mini animated dots
                    <div class="flex items-center gap-0.5">
                        <span class="w-1 h-1 bg-brand/70 rounded-full animate-pulse"></span>
                        <span class="w-1 h-1 bg-brand/70 rounded-full animate-pulse" style="animation-delay: 100ms;"></span>
                        <span class="w-1 h-1 bg-brand/70 rounded-full animate-pulse" style="animation-delay: 200ms;"></span>
                    </div>
                    <span>{users.len()}" typing"</span>
                </div>
            }.into_any()
        }}
    }
}

/// Formats the typing text based on number of users
fn format_typing_text(users: &[TypingUser]) -> (String, &'static str) {
    match users.len() {
        0 => (String::new(), ""),
        1 => (format!("@{}", users[0].handle), " is typing..."),
        2 => (
            format!("@{} and @{}", users[0].handle, users[1].handle),
            " are typing...",
        ),
        3 => (
            format!(
                "@{}, @{}, and @{}",
                users[0].handle, users[1].handle, users[2].handle
            ),
            " are typing...",
        ),
        n => (format!("{} people", n), " are typing..."),
    }
}

