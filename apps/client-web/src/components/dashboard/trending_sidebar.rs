// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{ActiveSynapse, TrendDirection, TrendingTopic, TrendingUser};
use leptos::prelude::*;
use leptos_router::components::A;

/// Format large numbers
fn format_count(count: u32) -> String {
    if count >= 1000 {
        format!("{:.1}K", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}

/// Trending content sidebar
#[component]
pub fn TrendingSidebar() -> impl IntoView {
    let topics = TrendingTopic::mock_topics();
    let users = TrendingUser::mock_users();
    let synapses = ActiveSynapse::mock_synapses();

    view! {
        <div class="space-y-4">
            // Trending Topics
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"/>
                    </svg>
                    <h3 class="font-semibold text-sm text-foreground">"Trending"</h3>
                </div>
                <div class="divide-y divide-border/30">
                    {topics.into_iter().enumerate().map(|(idx, topic)| {
                        view! {
                            <A
                                href=format!("/search?tag={}", topic.tag)
                                attr:class="flex items-center gap-3 px-4 py-2.5 hover:bg-foreground/[0.02] transition-colors"
                            >
                                <span class="text-xs text-foreground/30 font-mono w-4">{idx + 1}</span>
                                <div class="flex-1 min-w-0">
                                    <p class="font-medium text-foreground text-sm truncate">"#"{topic.tag}</p>
                                    <p class="text-xs text-foreground/40">{format_count(topic.post_count)}" posts"</p>
                                </div>
                                <span class=format!(
                                    "w-5 h-5 {}",
                                    match topic.trend_direction {
                                        TrendDirection::Up => "text-emerald-400",
                                        TrendDirection::Down => "text-rose-400",
                                        TrendDirection::Stable => "text-foreground/30",
                                    }
                                )>
                                    {match topic.trend_direction {
                                        TrendDirection::Up => view! {
                                            <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 10l7-7m0 0l7 7m-7-7v18"/>
                                            </svg>
                                        }.into_any(),
                                        TrendDirection::Down => view! {
                                            <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 14l-7 7m0 0l-7-7m7 7V3"/>
                                            </svg>
                                        }.into_any(),
                                        TrendDirection::Stable => view! {
                                            <svg fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14"/>
                                            </svg>
                                        }.into_any(),
                                    }}
                                </span>
                            </A>
                        }
                    }).collect_view()}
                </div>
                <div class="px-4 py-2 border-t border-border/30">
                    <A href="/trending" attr:class="text-xs text-brand hover:underline">"See all trending"</A>
                </div>
            </div>

            // Who to Follow
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                    </svg>
                    <h3 class="font-semibold text-sm text-foreground">"Who to Follow"</h3>
                </div>
                <div class="divide-y divide-border/30">
                    {users.into_iter().map(|user| {
                        let initials: String = user.display_name
                            .split_whitespace()
                            .take(2)
                            .filter_map(|s| s.chars().next())
                            .collect::<String>()
                            .to_uppercase();
                        
                        view! {
                            <div class="flex items-center gap-3 px-4 py-2.5">
                                <A href=format!("/profiles/{}", user.handle) attr:class="flex-shrink-0">
                                    {if let Some(url) = user.avatar_url {
                                        view! {
                                            <img src=url alt="" class="w-9 h-9 rounded-full object-cover"/>
                                        }.into_any()
                                    } else {
                                        view! {
                                            <div class="w-9 h-9 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center">
                                                <span class="text-brand font-bold text-xs">{initials}</span>
                                            </div>
                                        }.into_any()
                                    }}
                                </A>
                                <div class="flex-1 min-w-0">
                                    <div class="flex items-center gap-1">
                                        <A 
                                            href=format!("/profiles/{}", user.handle)
                                            attr:class="font-medium text-sm text-foreground hover:text-brand transition-colors truncate"
                                        >
                                            {user.display_name}
                                        </A>
                                        {if user.is_verified {
                                            view! {
                                                <svg class="w-3.5 h-3.5 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </div>
                                    <div class="flex items-center gap-2 text-xs">
                                        <span class="text-brand/60 font-mono">{"@"}{user.handle}</span>
                                        <span class="text-emerald-400">"+"{ user.follower_growth }</span>
                                    </div>
                                </div>
                                <button class="px-3 py-1 rounded-lg bg-brand/15 text-brand text-xs font-medium hover:bg-brand/25 transition-colors">
                                    "Follow"
                                </button>
                            </div>
                        }
                    }).collect_view()}
                </div>
                <div class="px-4 py-2 border-t border-border/30">
                    <A href="/discover/people" attr:class="text-xs text-brand hover:underline">"Discover more"</A>
                </div>
            </div>

            // Active Synapses
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="px-4 py-3 border-b border-border/30 flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                    </svg>
                    <h3 class="font-semibold text-sm text-foreground">"Active Synapses"</h3>
                </div>
                <div class="divide-y divide-border/30">
                    {synapses.into_iter().map(|synapse| {
                        let first_char = synapse.name.chars().next().unwrap_or('S').to_uppercase().to_string();
                        
                        view! {
                            <A
                                href=format!("/synapses/{}", synapse.id)
                                attr:class="flex items-center gap-3 px-4 py-2.5 hover:bg-foreground/[0.02] transition-colors"
                            >
                                <div class="w-8 h-8 rounded-lg bg-violet-500/20 flex items-center justify-center flex-shrink-0">
                                    <span class="text-violet-400 font-bold text-xs">{first_char}</span>
                                </div>
                                <div class="flex-1 min-w-0">
                                    <p class="font-medium text-sm text-foreground truncate">{synapse.name}</p>
                                    <div class="flex items-center gap-2 text-xs">
                                        <span class="flex items-center gap-1 text-emerald-400">
                                            <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                                            {format_count(synapse.online_count)}
                                        </span>
                                        <span class="text-foreground/30">"|"</span>
                                        <span class="text-foreground/40">{synapse.new_posts}" new"</span>
                                    </div>
                                </div>
                            </A>
                        }
                    }).collect_view()}
                </div>
                <div class="px-4 py-2 border-t border-border/30">
                    <A href="/synapses" attr:class="text-xs text-brand hover:underline">"Browse synapses"</A>
                </div>
            </div>

            // Quick links footer
            <div class="px-4 py-3 text-xs text-foreground/30 space-y-2">
                <div class="flex flex-wrap gap-x-3 gap-y-1">
                    <A href="/about" attr:class="hover:text-foreground/50 transition-colors">"About"</A>
                    <A href="/terms" attr:class="hover:text-foreground/50 transition-colors">"Terms"</A>
                    <A href="/privacy" attr:class="hover:text-foreground/50 transition-colors">"Privacy"</A>
                    <A href="/docs" attr:class="hover:text-foreground/50 transition-colors">"Docs"</A>
                </div>
                <p>"© 2025 meNexus • v0.3.0-alpha"</p>
            </div>
        </div>
    }
}
