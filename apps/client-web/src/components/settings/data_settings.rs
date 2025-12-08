// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

/// Data and storage settings panel
#[component]
pub fn DataSettings() -> impl IntoView {
    let (auto_download_media, set_auto_download_media) = signal(true);
    let (save_original_quality, set_save_original_quality) = signal(false);
    let (auto_clear_cache, set_auto_clear_cache) = signal(false);

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Data & Storage"</h2>
                <p class="text-sm text-foreground/50">"Manage storage, cache, and data export"</p>
            </div>

            // Storage Overview
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <div class="flex items-center justify-between">
                    <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4m0 5c0 2.21-3.582 4-8 4s-8-1.79-8-4"/>
                        </svg>
                        "Storage Usage"
                    </h3>
                    <span class="text-sm text-foreground/50">"847 MB used"</span>
                </div>

                // Storage breakdown
                <div class="space-y-3">
                    <StorageBar label="Messages & Posts" size="324 MB" percentage=38 color="bg-brand"/>
                    <StorageBar label="Media & Images" size="256 MB" percentage=30 color="bg-violet-500"/>
                    <StorageBar label="Cache" size="189 MB" percentage=22 color="bg-amber-500"/>
                    <StorageBar label="Other" size="78 MB" percentage=10 color="bg-foreground/30"/>
                </div>

                // Total bar
                <div class="pt-4 border-t border-border/30">
                    <div class="flex items-center justify-between text-sm mb-2">
                        <span class="text-foreground/50">"Total Local Storage"</span>
                        <span class="font-medium text-foreground">"847 MB / 2 GB"</span>
                    </div>
                    <div class="h-3 bg-foreground/10 rounded-full overflow-hidden">
                        <div class="h-full bg-gradient-to-r from-brand via-violet-500 to-amber-500 rounded-full" style="width: 42%"></div>
                    </div>
                </div>
            </div>

            // Media Settings
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                    </svg>
                    "Media"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Auto-download media"
                        description="Automatically download images and videos"
                        checked=auto_download_media
                        on_change=move |v| set_auto_download_media.set(v)
                    />
                    <ToggleSetting
                        label="Save original quality"
                        description="Keep full resolution copies (uses more storage)"
                        checked=save_original_quality
                        on_change=move |v| set_save_original_quality.set(v)
                    />
                </div>
            </div>

            // Cache Management
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                    </svg>
                    "Cache"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Auto-clear cache"
                        description="Automatically clear cache when storage is low"
                        checked=auto_clear_cache
                        on_change=move |v| set_auto_clear_cache.set(v)
                    />

                    <div class="flex items-center justify-between p-4 bg-foreground/5 rounded-xl">
                        <div>
                            <p class="text-sm font-medium text-foreground">"Clear Cache Now"</p>
                            <p class="text-xs text-foreground/40">"Free up 189 MB of space"</p>
                        </div>
                        <button class="px-4 py-2 bg-foreground/5 hover:bg-foreground/10 text-foreground/70 hover:text-foreground rounded-xl text-sm font-medium transition-colors border border-border/50">
                            "Clear"
                        </button>
                    </div>
                </div>
            </div>

            // Data Export
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start gap-4">
                    <div class="w-10 h-10 rounded-xl bg-emerald-500/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="font-semibold text-foreground">"Export Your Data"</h3>
                        <p class="text-sm text-foreground/50 mt-1">"Download a copy of all your data including posts, messages, and media."</p>
                        <div class="flex flex-wrap gap-2 mt-3">
                            <button class="px-4 py-2 bg-brand/15 text-brand rounded-xl text-sm font-medium hover:bg-brand/25 transition-colors">
                                "Request Export"
                            </button>
                            <span class="text-xs text-foreground/40 self-center">"Usually ready within 24 hours"</span>
                        </div>
                    </div>
                </div>
            </div>

            // Data Import
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start gap-4">
                    <div class="w-10 h-10 rounded-xl bg-violet-500/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="font-semibold text-foreground">"Import Data"</h3>
                        <p class="text-sm text-foreground/50 mt-1">"Import your data from another instance or a backup."</p>
                        <button class="mt-3 px-4 py-2 bg-foreground/5 hover:bg-foreground/10 text-foreground/70 hover:text-foreground rounded-xl text-sm font-medium transition-colors border border-border/50">
                            "Import Archive"
                        </button>
                    </div>
                </div>
            </div>

            // Danger Zone
            <div class="bg-card border border-rose-500/30 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-rose-400 flex items-center gap-2">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                    </svg>
                    "Data Management"
                </h3>

                <div class="space-y-3">
                    <div class="flex items-center justify-between p-4 border border-border/30 rounded-xl">
                        <div>
                            <p class="text-sm font-medium text-foreground">"Clear All Messages"</p>
                            <p class="text-xs text-foreground/40">"Delete all local message history"</p>
                        </div>
                        <button class="px-3 py-1.5 text-sm text-foreground/60 hover:text-amber-400 hover:bg-amber-500/10 border border-border/50 rounded-lg transition-colors">
                            "Clear"
                        </button>
                    </div>

                    <div class="flex items-center justify-between p-4 border border-rose-500/30 rounded-xl bg-rose-500/5">
                        <div>
                            <p class="text-sm font-medium text-foreground">"Delete All Local Data"</p>
                            <p class="text-xs text-foreground/40">"Remove all locally stored data. This cannot be undone."</p>
                        </div>
                        <button class="px-3 py-1.5 text-sm text-rose-400 hover:bg-rose-500/15 border border-rose-500/30 rounded-lg transition-colors">
                            "Delete"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Storage breakdown bar
#[component]
fn StorageBar(
    label: &'static str,
    size: &'static str,
    percentage: u32,
    color: &'static str,
) -> impl IntoView {
    view! {
        <div class="space-y-1.5">
            <div class="flex items-center justify-between text-sm">
                <span class="text-foreground/70">{label}</span>
                <span class="text-foreground/50 font-mono text-xs">{size}</span>
            </div>
            <div class="h-2 bg-foreground/10 rounded-full overflow-hidden">
                <div class=format!("h-full {} rounded-full transition-all", color) style=format!("width: {}%", percentage)></div>
            </div>
        </div>
    }
}

/// Reusable toggle setting component
#[component]
fn ToggleSetting(
    label: &'static str,
    description: &'static str,
    #[prop(into)] checked: ReadSignal<bool>,
    on_change: impl Fn(bool) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
            <div>
                <p class="text-sm font-medium text-foreground">{label}</p>
                <p class="text-xs text-foreground/40">{description}</p>
            </div>
            <button
                class=move || format!(
                    "relative w-11 h-6 rounded-full transition-colors {}",
                    if checked.get() { "bg-brand" } else { "bg-foreground/20" }
                )
                on:click=move |_| on_change(!checked.get())
            >
                <span class=move || format!(
                    "absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform {}",
                    if checked.get() { "translate-x-5" } else { "translate-x-0" }
                )></span>
            </button>
        </div>
    }
}
