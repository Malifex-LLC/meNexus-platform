// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{Livestream, QualityOption, StreamStatus};
use leptos::prelude::*;

/// The main stream player component with video area and controls
#[component]
pub fn StreamPlayer(
    #[prop(into)] stream: Livestream,
) -> impl IntoView {
    let (is_playing, set_is_playing) = signal(true);
    let (is_muted, set_is_muted) = signal(false);
    let (volume, set_volume) = signal(80u8);
    let (is_fullscreen, set_is_fullscreen) = signal(false);
    let (show_controls, set_show_controls) = signal(true);
    let (show_quality_menu, set_show_quality_menu) = signal(false);
    let (selected_quality, set_selected_quality) = signal("Auto".to_string());
    let (is_theater_mode, set_is_theater_mode) = signal(false);

    let is_live = stream.status == StreamStatus::Live;
    let quality_options = stream.quality_options.clone();
    let stats = stream.stats.clone();

    // Format duration
    let duration_formatted = format_duration(stats.duration_seconds);
    let duration_for_overlay = duration_formatted.clone();
    let duration_for_controls = duration_formatted.clone();

    view! {
        <div 
            class="relative w-full aspect-video bg-black rounded-xl overflow-hidden group"
            on:mouseenter=move |_| set_show_controls.set(true)
            on:mouseleave=move |_| {
                if is_playing.get() {
                    set_show_controls.set(false);
                }
            }
        >
            // Video area (placeholder - would be actual video element)
            <div class="absolute inset-0 bg-gradient-to-br from-gray-900 via-black to-gray-900">
                {if is_live {
                    view! {
                        // Simulated video content with animated gradient
                        <div class="absolute inset-0 bg-[radial-gradient(ellipse_at_center,_var(--tw-gradient-stops))] from-brand/5 via-transparent to-transparent animate-pulse"></div>
                        
                        // Center play indicator (shows when paused)
                        {move || {
                            if !is_playing.get() {
                                view! {
                                    <div class="absolute inset-0 flex items-center justify-center bg-black/40 backdrop-blur-sm">
                                        <button 
                                            class="w-20 h-20 rounded-full bg-brand/90 hover:bg-brand flex items-center justify-center transition-all hover:scale-110 shadow-xl shadow-brand/30"
                                            on:click=move |_| set_is_playing.set(true)
                                        >
                                            <svg class="w-10 h-10 text-white ml-1" fill="currentColor" viewBox="0 0 24 24">
                                                <path d="M8 5v14l11-7z"></path>
                                            </svg>
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }
                        }}
                    }.into_any()
                } else {
                    view! {
                        // Offline state
                        <div class="absolute inset-0 flex flex-col items-center justify-center bg-gradient-to-br from-gray-900 to-black">
                            <div class="w-24 h-24 rounded-full bg-foreground/5 flex items-center justify-center mb-6">
                                <svg class="w-12 h-12 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25h-9A2.25 2.25 0 002.25 7.5v9a2.25 2.25 0 002.25 2.25z"></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold text-foreground/70 mb-2">"Stream Offline"</h3>
                            <p class="text-foreground/40 text-sm">"The host is not currently streaming"</p>
                            <button class="mt-6 px-6 py-2.5 bg-brand/20 hover:bg-brand/30 text-brand rounded-lg font-medium transition-colors flex items-center gap-2">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"></path>
                                </svg>
                                "Notify me when live"
                            </button>
                        </div>
                    }.into_any()
                }}
            </div>

            // Top overlay - LIVE badge and viewer count
            {if is_live {
                view! {
                    <div class="absolute top-4 left-4 right-4 flex items-start justify-between pointer-events-none">
                        <div class="flex items-center gap-3">
                            // LIVE badge
                            <div class="flex items-center gap-2 px-3 py-1.5 bg-rose-600 rounded-lg shadow-lg">
                                <span class="w-2 h-2 rounded-full bg-white animate-pulse"></span>
                                <span class="text-white text-sm font-bold tracking-wide">"LIVE"</span>
                            </div>
                            
                            // Viewer count
                            <div class="flex items-center gap-2 px-3 py-1.5 bg-black/60 backdrop-blur-sm rounded-lg">
                                <svg class="w-4 h-4 text-white/80" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"></path>
                                </svg>
                                <span class="text-white text-sm font-medium">{stats.viewer_count.to_string()}</span>
                            </div>
                            
                            // Duration
                            <div class="flex items-center gap-2 px-3 py-1.5 bg-black/60 backdrop-blur-sm rounded-lg">
                                <svg class="w-4 h-4 text-white/80" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                </svg>
                                <span class="text-white text-sm font-mono">{duration_for_overlay}</span>
                            </div>
                        </div>

                        // Latency indicator
                        <div class="flex items-center gap-2 px-3 py-1.5 bg-black/60 backdrop-blur-sm rounded-lg">
                            <span class="w-2 h-2 rounded-full bg-emerald-400"></span>
                            <span class="text-white/80 text-xs font-mono">{format!("{}ms", stats.latency_ms)}</span>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}

            // Bottom controls overlay
            {if is_live {
                view! {
                    <div 
                        class=move || format!(
                            "absolute bottom-0 left-0 right-0 bg-gradient-to-t from-black/90 via-black/50 to-transparent pt-16 pb-4 px-4 transition-opacity duration-300 {}",
                            if show_controls.get() { "opacity-100" } else { "opacity-0" }
                        )
                    >
                        // Progress bar (for VOD this would be seekable)
                        <div class="w-full h-1 bg-white/20 rounded-full mb-4">
                            <div class="h-full bg-brand rounded-full" style="width: 100%;"></div>
                        </div>

                        // Control buttons
                        <div class="flex items-center justify-between">
                            <div class="flex items-center gap-2">
                                // Play/Pause
                                <button 
                                    class="p-2 rounded-lg hover:bg-white/10 text-white transition-colors"
                                    on:click=move |_| set_is_playing.update(|p| *p = !*p)
                                    title=move || if is_playing.get() { "Pause" } else { "Play" }
                                >
                                    {move || {
                                        if is_playing.get() {
                                            view! {
                                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                                    <path d="M6 4h4v16H6V4zm8 0h4v16h-4V4z"></path>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 24 24">
                                                    <path d="M8 5v14l11-7z"></path>
                                                </svg>
                                            }.into_any()
                                        }
                                    }}
                                </button>

                                // Volume
                                <div class="flex items-center gap-2 group/vol">
                                    <button 
                                        class="p-2 rounded-lg hover:bg-white/10 text-white transition-colors"
                                        on:click=move |_| set_is_muted.update(|m| *m = !*m)
                                        title=move || if is_muted.get() { "Unmute" } else { "Mute" }
                                    >
                                        {move || {
                                            let vol = volume.get();
                                            let muted = is_muted.get();
                                            if muted || vol == 0 {
                                                view! {
                                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" clip-rule="evenodd"></path>
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M17 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2"></path>
                                                    </svg>
                                                }.into_any()
                                            } else if vol < 50 {
                                                view! {
                                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.536 8.464a5 5 0 010 7.072M12 6v12m0 0l-4-4H5a1 1 0 01-1-1v-2a1 1 0 011-1h3l4-4z"></path>
                                                    </svg>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"></path>
                                                    </svg>
                                                }.into_any()
                                            }
                                        }}
                                    </button>
                                    
                                    // Volume slider
                                    <div class="w-0 group-hover/vol:w-20 overflow-hidden transition-all duration-200">
                                        <input 
                                            type="range"
                                            min="0"
                                            max="100"
                                            prop:value=move || volume.get()
                                            on:input=move |ev| {
                                                let val: u8 = event_target_value(&ev).parse().unwrap_or(80);
                                                set_volume.set(val);
                                                if val > 0 {
                                                    set_is_muted.set(false);
                                                }
                                            }
                                            class="w-full h-1 bg-white/30 rounded-full appearance-none cursor-pointer accent-brand"
                                        />
                                    </div>
                                </div>

                                // Stream time
                                <span class="text-white/70 text-sm font-mono ml-2">
                                    {duration_for_controls}
                                </span>
                            </div>

                            <div class="flex items-center gap-2">
                                // Quality selector
                                <div class="relative">
                                    <button 
                                        class="flex items-center gap-2 px-3 py-1.5 rounded-lg hover:bg-white/10 text-white text-sm transition-colors"
                                        on:click=move |_| set_show_quality_menu.update(|s| *s = !*s)
                                    >
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path>
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                        </svg>
                                        <span>{move || selected_quality.get()}</span>
                                    </button>

                                    // Quality dropdown
                                    {move || {
                                        if show_quality_menu.get() {
                                            view! {
                                                <div class="absolute bottom-full right-0 mb-2 w-48 bg-gray-900/95 backdrop-blur-sm border border-white/10 rounded-xl shadow-xl py-2 z-50">
                                                    <div class="px-3 py-2 border-b border-white/10">
                                                        <span class="text-xs font-semibold text-white/50 uppercase tracking-wider">"Quality"</span>
                                                    </div>
                                                    {quality_options.iter().map(|opt| {
                                                        let label = opt.label.clone();
                                                        let label_for_click = opt.label.clone();
                                                        let is_source = opt.is_source;
                                                        view! {
                                                            <button 
                                                                class=move || format!(
                                                                    "w-full flex items-center justify-between px-3 py-2 text-sm transition-colors {}",
                                                                    if selected_quality.get() == label {
                                                                        "bg-brand/20 text-brand"
                                                                    } else {
                                                                        "text-white/80 hover:bg-white/10"
                                                                    }
                                                                )
                                                                on:click=move |_| {
                                                                    set_selected_quality.set(label_for_click.clone());
                                                                    set_show_quality_menu.set(false);
                                                                }
                                                            >
                                                                <span>{label.clone()}</span>
                                                                {if is_source {
                                                                    view! {
                                                                        <span class="px-1.5 py-0.5 bg-brand/30 text-brand text-xs rounded">"Source"</span>
                                                                    }.into_any()
                                                                } else {
                                                                    view! { <span></span> }.into_any()
                                                                }}
                                                            </button>
                                                        }
                                                    }).collect_view()}
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }
                                    }}
                                </div>

                                // Theater mode
                                <button 
                                    class=move || format!(
                                        "p-2 rounded-lg transition-colors {}",
                                        if is_theater_mode.get() { "bg-brand/30 text-brand" } else { "hover:bg-white/10 text-white" }
                                    )
                                    on:click=move |_| set_is_theater_mode.update(|t| *t = !*t)
                                    title="Theater mode"
                                >
                                    <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 8V4m0 0h4M4 4l5 5m11-1V4m0 0h-4m4 0l-5 5M4 16v4m0 0h4m-4 0l5-5m11 5l-5-5m5 5v-4m0 4h-4"></path>
                                    </svg>
                                </button>

                                // Fullscreen
                                <button 
                                    class="p-2 rounded-lg hover:bg-white/10 text-white transition-colors"
                                    on:click=move |_| set_is_fullscreen.update(|f| *f = !*f)
                                    title=move || if is_fullscreen.get() { "Exit fullscreen" } else { "Fullscreen" }
                                >
                                    {move || {
                                        if is_fullscreen.get() {
                                            view! {
                                                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 9V4.5M9 9H4.5M9 9L3.75 3.75M9 15v4.5M9 15H4.5M9 15l-5.25 5.25M15 9h4.5M15 9V4.5M15 9l5.25-5.25M15 15h4.5M15 15v4.5m0-4.5l5.25 5.25"></path>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <svg class="w-6 h-6" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M3.75 3.75v4.5m0-4.5h4.5m-4.5 0L9 9M3.75 20.25v-4.5m0 4.5h4.5m-4.5 0L9 15M20.25 3.75h-4.5m4.5 0v4.5m0-4.5L15 9m5.25 11.25h-4.5m4.5 0v-4.5m0 4.5L15 15"></path>
                                                </svg>
                                            }.into_any()
                                        }
                                    }}
                                </button>
                            </div>
                        </div>
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, minutes, secs)
    } else {
        format!("{:02}:{:02}", minutes, secs)
    }
}

