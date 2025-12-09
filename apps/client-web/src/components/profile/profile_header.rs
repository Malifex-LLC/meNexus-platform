// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::ProfileData;
use leptos::prelude::*;

/// Format large numbers with K/M suffixes
fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

/// Profile header with banner, avatar, name, and stats
#[component]
pub fn ProfileHeader(#[prop(into)] profile: ProfileData) -> impl IntoView {
    let initials = profile.get_initials();
    let display_name = profile.display_name.clone();
    let handle = profile.handle.clone();
    let is_verified = profile.is_verified;
    let is_online = profile.is_online;
    let avatar_url = profile.avatar_url.clone();
    let banner_url = profile.banner_url.clone();
    let followers = profile.followers_count;
    let following = profile.following_count;
    let posts = profile.posts_count;
    let reputation_score = profile.reputation.score;
    let reputation_level = profile.reputation.level.clone();
    let public_key = profile.public_key.clone();

    // Truncate public key for display
    let short_key = format!(
        "{}...{}",
        &public_key[..8],
        &public_key[public_key.len() - 6..]
    );

    view! {
        <div class="relative">
            // Banner
            <div class="h-32 sm:h-40 md:h-48 w-full relative overflow-hidden">
                {if let Some(url) = banner_url {
                    view! {
                        <img 
                            src=url 
                            alt="Profile banner" 
                            class="w-full h-full object-cover"
                        />
                    }.into_any()
                } else {
                    view! {
                        <div class="w-full h-full bg-gradient-to-br from-brand/30 via-brand/10 to-background">
                            // Decorative grid pattern
                            <div class="absolute inset-0 opacity-10">
                                <div class="absolute inset-0" style="background-image: linear-gradient(rgba(255,255,255,.1) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,.1) 1px, transparent 1px); background-size: 20px 20px;"></div>
                            </div>
                            // Decorative elements
                            <div class="absolute top-4 right-4 w-24 h-24 rounded-full bg-brand/20 blur-2xl"></div>
                            <div class="absolute bottom-4 left-1/4 w-32 h-32 rounded-full bg-brand/10 blur-3xl"></div>
                        </div>
                    }.into_any()
                }}
                
                // Gradient overlay at bottom
                <div class="absolute inset-x-0 bottom-0 h-24 bg-gradient-to-t from-panel to-transparent"></div>
            </div>

            // Profile info section
            <div class="relative px-4 sm:px-6 pb-4">
                // Avatar - positioned to overlap banner
                <div class="absolute -top-16 sm:-top-20 left-4 sm:left-6">
                    <div class="relative">
                        {if let Some(url) = avatar_url {
                            view! {
                                <img 
                                    src=url 
                                    alt=format!("{}'s avatar", display_name)
                                    class="w-28 h-28 sm:w-36 sm:h-36 rounded-2xl object-cover ring-4 ring-panel shadow-xl"
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-28 h-28 sm:w-36 sm:h-36 rounded-2xl bg-gradient-to-br from-brand/40 to-brand/20 ring-4 ring-panel shadow-xl flex items-center justify-center">
                                    <span class="text-brand font-bold text-3xl sm:text-4xl">{initials}</span>
                                </div>
                            }.into_any()
                        }}
                        
                        // Online indicator
                        {if is_online {
                            view! {
                                <div class="absolute bottom-2 right-2 w-5 h-5 status-online rounded-full ring-4 ring-panel"></div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </div>
                </div>

                // Action buttons (top right)
                <div class="flex items-center gap-2 justify-end pt-2 mb-12 sm:mb-16">
                    // Edit profile (shown if own profile)
                    <button class="px-4 py-2 rounded-xl bg-foreground/10 border border-border/50 text-foreground/80 text-sm font-medium hover:bg-foreground/15 hover:border-border transition-all">
                        "Edit Profile"
                    </button>
                    
                    // Follow button
                    <button class="px-4 py-2 rounded-xl bg-brand text-white text-sm font-medium hover:bg-brand/90 transition-all shadow-lg shadow-brand/20">
                        "Follow"
                    </button>
                    
                    // More options
                    <button class="p-2 rounded-xl bg-foreground/10 border border-border/50 text-foreground/60 hover:text-foreground hover:bg-foreground/15 transition-all">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z"/>
                        </svg>
                    </button>
                </div>

                // Name and verification
                <div class="flex items-center gap-2 flex-wrap mb-1">
                    <h1 class="text-2xl sm:text-3xl font-bold text-foreground">{display_name}</h1>
                    {if is_verified {
                        view! {
                            <svg class="w-6 h-6 text-brand" viewBox="0 0 24 24" fill="currentColor" title="Verified">
                                <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                            </svg>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                    
                    // Reputation badge
                    <span class=format!(
                        "px-2 py-0.5 rounded-lg text-xs font-semibold border flex items-center gap-1 {}",
                        reputation_level.color_class()
                    )>
                        <span class="w-3.5 h-3.5" inner_html=reputation_level.icon_svg()></span>
                        {reputation_level.label()}
                    </span>
                </div>

                // Handle and public key
                <div class="flex items-center gap-3 text-sm mb-4">
                    <span class="text-brand font-mono">{"@"}{handle}</span>
                    <span class="text-foreground/30">"·"</span>
                    <button 
                        class="flex items-center gap-1.5 text-foreground/40 hover:text-foreground/60 transition-colors font-mono text-xs group"
                        title="Copy public key"
                    >
                        <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 5.25a3 3 0 013 3m3 0a6 6 0 01-7.029 5.912c-.563-.097-1.159.026-1.563.43L10.5 17.25H8.25v2.25H6v2.25H2.25v-2.818c0-.597.237-1.17.659-1.591l6.499-6.499c.404-.404.527-1 .43-1.563A6 6 0 1121.75 8.25z"/>
                        </svg>
                        {short_key}
                        <svg class="w-3 h-3 opacity-0 group-hover:opacity-100 transition-opacity" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184"/>
                        </svg>
                    </button>
                </div>

                // Stats row
                <div class="flex items-center gap-6 flex-wrap">
                    <div class="flex items-center gap-1.5">
                        <span class="font-bold text-foreground">{format_count(posts)}</span>
                        <span class="text-foreground/50 text-sm">"posts"</span>
                    </div>
                    <div class="flex items-center gap-1.5 cursor-pointer hover:text-brand transition-colors">
                        <span class="font-bold text-foreground">{format_count(followers)}</span>
                        <span class="text-foreground/50 text-sm">"followers"</span>
                    </div>
                    <div class="flex items-center gap-1.5 cursor-pointer hover:text-brand transition-colors">
                        <span class="font-bold text-foreground">{format_count(following)}</span>
                        <span class="text-foreground/50 text-sm">"following"</span>
                    </div>
                    <div class="flex items-center gap-1.5">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"/>
                        </svg>
                        <span class="font-bold text-foreground">{format_count(reputation_score)}</span>
                        <span class="text-foreground/50 text-sm">"reputation"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
