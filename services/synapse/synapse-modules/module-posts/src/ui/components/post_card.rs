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
    let (is_liked, set_is_liked) = signal(liked);
    let (like_count, set_like_count) = signal(likes);

    // Get initials for avatar fallback
    let initials = author_name
        .split_whitespace()
        .take(2)
        .map(|s| s.chars().next().unwrap_or(' '))
        .collect::<String>()
        .to_uppercase();

    // Truncate public key for display
    let short_key = if author_public_key.len() > 12 {
        format!("{}...{}", &author_public_key[..6], &author_public_key[author_public_key.len()-4..])
    } else {
        author_public_key.clone()
    };

    view! {
        <article class="group relative bg-gradient-to-br from-panel to-card border border-border/50 rounded-xl overflow-hidden transition-all duration-300 hover:border-brand/30 hover:shadow-lg hover:shadow-brand/5">
            // Subtle top accent line
            <div class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-brand/20 to-transparent"></div>
            
            <div class="p-5">
                // Header: Avatar + Author Info + Timestamp
                <div class="flex items-start gap-4">
                    // Avatar
                    <div class="relative flex-shrink-0">
                        {if !author_avatar.is_empty() {
                            view! {
                                <img 
                                    src=author_avatar.clone()
                                    alt=format!("{}'s avatar", author_name)
                                    class="w-12 h-12 rounded-full object-cover ring-2 ring-border/50 group-hover:ring-brand/30 transition-all"
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-12 h-12 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 ring-2 ring-border/50 group-hover:ring-brand/30 flex items-center justify-center transition-all">
                                    <span class="text-brand font-bold text-sm">{initials.clone()}</span>
                                </div>
                            }.into_any()
                        }}
                        // Online indicator dot
                        <div class="absolute -bottom-0.5 -right-0.5 w-3.5 h-3.5 bg-emerald-500 rounded-full border-2 border-panel"></div>
                    </div>

                    // Author Info
                    <div class="flex-1 min-w-0">
                        <div class="flex items-center gap-2 flex-wrap">
                            <h3 class="font-semibold text-foreground truncate">{author_name.clone()}</h3>
                            <span class="text-brand/80 font-mono text-sm">{"@"}{author_handle.clone()}</span>
                        </div>
                        <div class="flex items-center gap-2 mt-0.5">
                            <span class="text-foreground/40 font-mono text-xs tracking-tight" title=author_public_key.clone()>
                                {short_key}
                            </span>
                            <span class="text-foreground/20">{"·"}</span>
                            <time class="text-foreground/40 text-xs">{timestamp.clone()}</time>
                        </div>
                    </div>

                    // Channel badge
                    <div class="flex-shrink-0">
                        <span class="inline-flex items-center px-2.5 py-1 rounded-md bg-brand/10 text-brand text-xs font-medium border border-brand/20">
                            <svg class="w-3 h-3 mr-1.5 opacity-70" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 20l4-16m2 16l4-16M6 9h14M4 15h14"></path>
                            </svg>
                            {posted_in.clone()}
                        </span>
                    </div>
                </div>

                // Content
                <div class="mt-4 pl-16">
                    <p class="text-foreground/90 leading-relaxed whitespace-pre-wrap break-words">
                        {content.clone()}
                    </p>
                </div>

                // Actions Bar
                <div class="mt-5 pl-16 flex items-center gap-1">
                    // Like button
                    <button 
                        class="group/btn flex items-center gap-2 px-3 py-1.5 rounded-lg transition-all duration-200 hover:bg-brand/10"
                        on:click=move |_| {
                            if is_liked.get() {
                                set_like_count.update(|c| *c = c.saturating_sub(1));
                            } else {
                                set_like_count.update(|c| *c += 1);
                            }
                            set_is_liked.update(|l| *l = !*l);
                        }
                    >
                        <svg 
                            class=move || format!(
                                "w-5 h-5 transition-all duration-200 {} {}",
                                if is_liked.get() { "fill-brand text-brand scale-110" } else { "fill-none text-foreground/50 group-hover/btn:text-brand" },
                                "group-hover/btn:scale-110"
                            )
                            viewBox="0 0 24 24" 
                            stroke="currentColor" 
                            stroke-width="2"
                        >
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"></path>
                        </svg>
                        <span class=move || format!(
                            "text-sm font-medium transition-colors {}",
                            if is_liked.get() { "text-brand" } else { "text-foreground/50 group-hover/btn:text-brand" }
                        )>
                            {move || like_count.get()}
                        </span>
                    </button>

                    // Comment button
                    <button class="group/btn flex items-center gap-2 px-3 py-1.5 rounded-lg transition-all duration-200 hover:bg-sky-500/10">
                        <svg class="w-5 h-5 text-foreground/50 group-hover/btn:text-sky-400 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                        </svg>
                        <span class="text-sm font-medium text-foreground/50 group-hover/btn:text-sky-400 transition-colors">{comments}</span>
                    </button>

                    // Share button
                    <button class="group/btn flex items-center gap-2 px-3 py-1.5 rounded-lg transition-all duration-200 hover:bg-emerald-500/10">
                        <svg class="w-5 h-5 text-foreground/50 group-hover/btn:text-emerald-400 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"></path>
                        </svg>
                    </button>

                    // More options
                    <button class="ml-auto group/btn p-1.5 rounded-lg transition-all duration-200 hover:bg-foreground/5">
                        <svg class="w-5 h-5 text-foreground/30 group-hover/btn:text-foreground/60 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path>
                        </svg>
                    </button>
                </div>
            </div>

            // Bottom subtle gradient
            <div class="absolute bottom-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-border/30 to-transparent"></div>
        </article>
    }
}
