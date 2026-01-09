// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{Livestream, StreamStatus};
use crate::ui::components::{
    stream_info::StreamInfoPanel,
    stream_player::StreamPlayer,
    streamer_card::StreamerCard,
};
use leptos::prelude::*;

/// Main livestream feed component
#[component]
pub fn LivestreamFeed() -> impl IntoView {
    // Mock data - in real app this would come from server
    let stream = Livestream::mock_live();
    
    let is_live = stream.status == StreamStatus::Live;
    let streamer = stream.streamer.clone();
    let info = stream.info.clone();
    let stats = stream.stats.clone();
    let status = stream.status.clone();
    let stream_for_player = stream.clone();

    view! {
        <div class="flex flex-col h-full w-full bg-background overflow-hidden">
            // Header bar
            <header class="flex-shrink-0 flex items-center justify-between px-4 py-3 border-b border-border/50 bg-panel/30 backdrop-blur-sm">
                <div class="flex items-center gap-3">
                    // Livestream icon
                    <div class="w-9 h-9 rounded-lg bg-gradient-to-br from-rose-500 to-rose-600 flex items-center justify-center">
                        <svg class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25h-9A2.25 2.25 0 002.25 7.5v9a2.25 2.25 0 002.25 2.25z"></path>
                        </svg>
                    </div>
                    <div>
                        <h1 class="font-bold text-foreground">"Livestream"</h1>
                        <p class="text-xs text-foreground/50">"Synapse broadcast channel"</p>
                    </div>
                </div>

                <div class="flex items-center gap-3">
                    // Status indicator
                    {if is_live {
                        view! {
                            <div class="flex items-center gap-2 px-3 py-1.5 bg-rose-500/20 border border-rose-500/30 rounded-full">
                                <span class="w-2 h-2 rounded-full bg-rose-500 animate-pulse"></span>
                                <span class="text-rose-400 text-sm font-medium">"LIVE NOW"</span>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="flex items-center gap-2 px-3 py-1.5 bg-foreground/5 border border-border/50 rounded-full">
                                <span class="w-2 h-2 rounded-full bg-foreground/30"></span>
                                <span class="text-foreground/50 text-sm font-medium">"OFFLINE"</span>
                            </div>
                        }.into_any()
                    }}

                    // Network status
                    <div class="flex items-center gap-2 px-3 py-1.5 bg-emerald-500/10 border border-emerald-500/20 rounded-full">
                        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                        <span class="text-emerald-400 text-xs font-mono">"P2P Connected"</span>
                    </div>
                </div>
            </header>

            // Main content - scrollable
            <div class="flex-1 overflow-y-auto scrollbar-styled">
                <div class="max-w-6xl mx-auto p-4 lg:p-6 space-y-4">
                    // Video player
                    <StreamPlayer stream=stream_for_player />

                    // Stream info and streamer card in a grid on larger screens
                    <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
                        // Stream info (takes 2 columns)
                        <div class="lg:col-span-2">
                            <StreamInfoPanel 
                                info=info 
                                stats=stats 
                                status=status 
                            />
                        </div>

                        // Streamer card (takes 1 column)
                        <div class="lg:col-span-1">
                            <StreamerCard 
                                streamer=streamer 
                                is_live=is_live 
                            />

                            // Quick links card
                            <div class="mt-4 bg-panel/50 border border-border/50 rounded-xl p-4">
                                <h3 class="text-sm font-semibold text-foreground/70 uppercase tracking-wider mb-3">"Quick Links"</h3>
                                <div class="space-y-2">
                                    <a href="#" class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 text-foreground/70 hover:text-foreground transition-colors">
                                        <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <span class="text-sm">"Past Broadcasts"</span>
                                    </a>
                                    <a href="#" class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 text-foreground/70 hover:text-foreground transition-colors">
                                        <svg class="w-5 h-5 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"></path>
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                        </svg>
                                        <span class="text-sm">"Clips"</span>
                                    </a>
                                    <a href="#" class="flex items-center gap-3 p-2 rounded-lg hover:bg-foreground/5 text-foreground/70 hover:text-foreground transition-colors">
                                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                        </svg>
                                        <span class="text-sm">"Schedule"</span>
                                    </a>
                                </div>
                            </div>

                            // Stream schedule (if offline)
                            {if !is_live {
                                view! {
                                    <div class="mt-4 bg-panel/50 border border-border/50 rounded-xl p-4">
                                        <h3 class="text-sm font-semibold text-foreground/70 uppercase tracking-wider mb-3">"Upcoming Streams"</h3>
                                        <div class="space-y-3">
                                            <div class="p-3 rounded-lg bg-foreground/5 border border-border/30">
                                                <div class="flex items-center gap-2 text-sm">
                                                    <span class="text-brand font-mono">"Tomorrow"</span>
                                                    <span class="text-foreground/30">"•"</span>
                                                    <span class="text-foreground/60">"3:00 PM EST"</span>
                                                </div>
                                                <p class="text-foreground/80 text-sm mt-1">"Weekly Dev Update & Q&A"</p>
                                            </div>
                                            <div class="p-3 rounded-lg bg-foreground/5 border border-border/30">
                                                <div class="flex items-center gap-2 text-sm">
                                                    <span class="text-brand font-mono">"Friday"</span>
                                                    <span class="text-foreground/30">"•"</span>
                                                    <span class="text-foreground/60">"7:00 PM EST"</span>
                                                </div>
                                                <p class="text-foreground/80 text-sm mt-1">"Chill Coding Session"</p>
                                            </div>
                                        </div>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                    </div>

                    // Recent clips section (when live)
                    {if is_live {
                        view! {
                            <div class="bg-panel/50 border border-border/50 rounded-xl p-4">
                                <div class="flex items-center justify-between mb-4">
                                    <h3 class="font-semibold text-foreground">"Recent Clips"</h3>
                                    <a href="#" class="text-sm text-brand hover:underline">"View all →"</a>
                                </div>
                                <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-4">
                                    {(0..4).map(|i| {
                                        view! {
                                            <div class="group cursor-pointer">
                                                <div class="relative aspect-video bg-black/50 rounded-lg overflow-hidden">
                                                    <div class="absolute inset-0 bg-gradient-to-br from-brand/10 to-violet-500/10"></div>
                                                    <div class="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity bg-black/40">
                                                        <svg class="w-10 h-10 text-white" fill="currentColor" viewBox="0 0 24 24">
                                                            <path d="M8 5v14l11-7z"></path>
                                                        </svg>
                                                    </div>
                                                    <div class="absolute bottom-2 right-2 px-1.5 py-0.5 bg-black/80 rounded text-white text-xs font-mono">
                                                        "0:"{30 + i * 15}
                                                    </div>
                                                </div>
                                                <p class="mt-2 text-sm text-foreground/80 truncate group-hover:text-foreground transition-colors">
                                                    "Amazing moment #{"{}"}"
                                                    {i + 1}
                                                </p>
                                                <p class="text-xs text-foreground/40">{(i + 1) * 12}" views"</p>
                                            </div>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
            </div>
        </div>
    }
}

/// Compact livestream player for embedding in other layouts
#[component]
pub fn LivestreamPlayerCompact() -> impl IntoView {
    let stream = Livestream::mock_live();
    let is_live = stream.status == StreamStatus::Live;
    let viewer_count = stream.stats.viewer_count;
    let title = stream.info.title.clone();

    view! {
        <div class="relative w-full aspect-video bg-black rounded-xl overflow-hidden group cursor-pointer">
            // Video placeholder
            <div class="absolute inset-0 bg-gradient-to-br from-gray-900 via-black to-gray-900">
                <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-brand/5 via-transparent to-transparent animate-pulse"></div>
            </div>

            // Overlay info
            <div class="absolute inset-0 flex flex-col justify-between p-3 bg-gradient-to-t from-black/80 via-transparent to-black/40 opacity-0 group-hover:opacity-100 transition-opacity">
                // Top
                <div class="flex items-center justify-between">
                    {if is_live {
                        view! {
                            <div class="flex items-center gap-2 px-2 py-1 bg-rose-600 rounded text-white text-xs font-bold">
                                <span class="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
                                "LIVE"
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="flex items-center gap-2 px-2 py-1 bg-gray-600 rounded text-white text-xs font-bold">
                                "OFFLINE"
                            </div>
                        }.into_any()
                    }}
                    
                    <div class="flex items-center gap-1.5 px-2 py-1 bg-black/60 rounded text-white text-xs">
                        <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5z"></path>
                        </svg>
                        {viewer_count.to_string()}
                    </div>
                </div>

                // Bottom
                <div>
                    <p class="text-white text-sm font-medium line-clamp-2">{title}</p>
                </div>
            </div>

            // Play button
            <div class="absolute inset-0 flex items-center justify-center">
                <div class="w-14 h-14 rounded-full bg-white/20 backdrop-blur-sm flex items-center justify-center group-hover:bg-brand/80 transition-all">
                    <svg class="w-6 h-6 text-white ml-0.5" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M8 5v14l11-7z"></path>
                    </svg>
                </div>
            </div>
        </div>
    }
}

