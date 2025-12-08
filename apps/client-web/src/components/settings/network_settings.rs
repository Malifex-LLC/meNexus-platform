// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

/// Network and sync settings panel
#[component]
pub fn NetworkSettings() -> impl IntoView {
    let (auto_sync, set_auto_sync) = signal(true);
    let (sync_on_wifi_only, set_sync_on_wifi_only) = signal(false);
    let (peer_discovery, set_peer_discovery) = signal(true);
    let (relay_enabled, set_relay_enabled) = signal(true);
    let (max_peers, set_max_peers) = signal(25u32);
    let (bandwidth_limit, set_bandwidth_limit) = signal(0u32); // 0 = unlimited

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Network Settings"</h2>
                <p class="text-sm text-foreground/50">"Configure P2P networking and sync preferences"</p>
            </div>

            // Connection Status
            <div class="bg-gradient-to-br from-emerald-500/10 via-brand/5 to-violet-500/10 border border-border/50 rounded-2xl p-6">
                <div class="flex items-center justify-between">
                    <div class="flex items-center gap-4">
                        <div class="relative">
                            <div class="w-14 h-14 rounded-full bg-emerald-500/20 flex items-center justify-center">
                                <svg class="w-7 h-7 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"/>
                                </svg>
                            </div>
                            <span class="absolute -bottom-1 -right-1 w-4 h-4 bg-emerald-400 rounded-full border-2 border-card"></span>
                        </div>
                        <div>
                            <h3 class="font-semibold text-foreground">"Connected to Network"</h3>
                            <p class="text-sm text-foreground/50">"Syncing with 24 peers"</p>
                        </div>
                    </div>
                    <div class="text-right">
                        <p class="text-2xl font-bold text-foreground">"18"<span class="text-sm text-foreground/40">"ms"</span></p>
                        <p class="text-xs text-foreground/40">"Latency"</p>
                    </div>
                </div>

                <div class="grid grid-cols-3 gap-4 mt-6 pt-6 border-t border-border/30">
                    <div class="text-center">
                        <p class="text-lg font-bold text-foreground">"24"</p>
                        <p class="text-xs text-foreground/40">"Connected Peers"</p>
                    </div>
                    <div class="text-center">
                        <p class="text-lg font-bold text-foreground">"1.2 GB"</p>
                        <p class="text-xs text-foreground/40">"Data Synced"</p>
                    </div>
                    <div class="text-center">
                        <p class="text-lg font-bold text-emerald-400">"99.9%"</p>
                        <p class="text-xs text-foreground/40">"Uptime"</p>
                    </div>
                </div>
            </div>

            // Sync Settings
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                    </svg>
                    "Sync Settings"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Auto-sync"
                        description="Automatically sync data in the background"
                        checked=auto_sync
                        on_change=move |v| set_auto_sync.set(v)
                    />
                    <ToggleSetting
                        label="Sync on Wi-Fi only"
                        description="Don't use mobile data for syncing"
                        checked=sync_on_wifi_only
                        on_change=move |v| set_sync_on_wifi_only.set(v)
                    />
                </div>
            </div>

            // P2P Settings
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0"/>
                    </svg>
                    "Peer-to-Peer"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Peer discovery"
                        description="Automatically discover and connect to nearby peers"
                        checked=peer_discovery
                        on_change=move |v| set_peer_discovery.set(v)
                    />
                    <ToggleSetting
                        label="Relay connections"
                        description="Allow relayed connections when direct P2P isn't possible"
                        checked=relay_enabled
                        on_change=move |v| set_relay_enabled.set(v)
                    />
                </div>

                // Max peers slider
                <div class="space-y-3 pt-2">
                    <div class="flex items-center justify-between">
                        <label class="text-sm font-medium text-foreground">"Maximum Peers"</label>
                        <span class="text-sm text-brand font-mono">{move || max_peers.get()}</span>
                    </div>
                    <input
                        type="range"
                        min="5"
                        max="100"
                        class="w-full accent-brand"
                        prop:value=move || max_peers.get()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                set_max_peers.set(v);
                            }
                        }
                    />
                    <div class="flex justify-between text-xs text-foreground/40">
                        <span>"5"</span>
                        <span>"50"</span>
                        <span>"100"</span>
                    </div>
                </div>

                // Bandwidth limit
                <div class="space-y-3 pt-2">
                    <div class="flex items-center justify-between">
                        <label class="text-sm font-medium text-foreground">"Bandwidth Limit"</label>
                        <span class="text-sm text-brand font-mono">
                            {move || if bandwidth_limit.get() == 0 { "Unlimited".to_string() } else { format!("{} MB/s", bandwidth_limit.get()) }}
                        </span>
                    </div>
                    <input
                        type="range"
                        min="0"
                        max="100"
                        step="10"
                        class="w-full accent-brand"
                        prop:value=move || bandwidth_limit.get()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<u32>() {
                                set_bandwidth_limit.set(v);
                            }
                        }
                    />
                    <div class="flex justify-between text-xs text-foreground/40">
                        <span>"∞"</span>
                        <span>"50 MB/s"</span>
                        <span>"100 MB/s"</span>
                    </div>
                </div>
            </div>

            // Advanced
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                    </svg>
                    "Advanced"
                </h3>

                <div class="space-y-3">
                    <button class="w-full flex items-center justify-between p-3 bg-foreground/5 rounded-xl hover:bg-foreground/10 transition-colors">
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 12h14M5 12a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v4a2 2 0 01-2 2M5 12a2 2 0 00-2 2v4a2 2 0 002 2h14a2 2 0 002-2v-4a2 2 0 00-2-2m-2-4h.01M17 16h.01"/>
                            </svg>
                            <span class="text-sm text-foreground">"Custom Bootstrap Nodes"</span>
                        </div>
                        <svg class="w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                        </svg>
                    </button>

                    <button class="w-full flex items-center justify-between p-3 bg-foreground/5 rounded-xl hover:bg-foreground/10 transition-colors">
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                            </svg>
                            <span class="text-sm text-foreground">"View Connection Logs"</span>
                        </div>
                        <svg class="w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                        </svg>
                    </button>
                </div>
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
