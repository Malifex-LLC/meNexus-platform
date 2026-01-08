// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{NetworkStats, SyncStatus};
use leptos::prelude::*;

/// Network statistics widget
#[component]
pub fn NetworkStatsWidget() -> impl IntoView {
    let stats = NetworkStats::mock();

    view! {
        <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            <div class="px-4 py-3 border-b border-border/30 flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <div class="w-6 h-6 rounded-lg bg-brand/15 flex items-center justify-center">
                        <svg class="w-3.5 h-3.5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </div>
                    <h3 class="font-semibold text-sm text-foreground">"Network"</h3>
                </div>

                // Sync status
                <div class=format!(
                    "flex items-center gap-1.5 px-2 py-0.5 rounded-full text-xs font-medium {}",
                    match stats.sync_status {
                        SyncStatus::Synced => "bg-status-online/15 text-status-online",
                        SyncStatus::Syncing => "bg-warning/15 text-warning",
                        SyncStatus::Offline => "bg-error/15 text-error",
                    }
                )>
                    <span class=format!(
                        "w-1.5 h-1.5 rounded-full {}",
                        match stats.sync_status {
                            SyncStatus::Synced => "status-online",
                            SyncStatus::Syncing => "status-away animate-pulse",
                            SyncStatus::Offline => "status-offline",
                        }
                    )></span>
                    {match stats.sync_status {
                        SyncStatus::Synced => "Synced",
                        SyncStatus::Syncing => "Syncing",
                        SyncStatus::Offline => "Offline",
                    }}
                </div>
            </div>

            <div class="p-4 grid grid-cols-2 gap-3">
                // Connected peers
                <StatCard
                    label="Peers"
                    value=stats.connected_peers.to_string()
                    icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z"/></svg>"#
                    color="text-stat-primary"
                />

                // Online users
                <StatCard
                    label="Online"
                    value=format_count(stats.online_users)
                    icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M5.121 17.804A13.937 13.937 0 0112 16c2.5 0 4.847.655 6.879 1.804M15 10a3 3 0 11-6 0 3 3 0 016 0zm6 2a9 9 0 11-18 0 9 9 0 0118 0z"/></svg>"#
                    color="text-status-online"
                />

                // Synapses
                <StatCard
                    label="Synapses"
                    value=stats.total_synapses.to_string()
                    icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>"#
                    color="text-stat-secondary"
                />

                // Latency
                <StatCard
                    label="Latency"
                    value=format!("{}ms", stats.latency_ms)
                    icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>"#
                    color="text-stat-tertiary"
                />
            </div>

            // Activity bar
            <div class="px-4 pb-4">
                <div class="flex items-center justify-between text-xs mb-1.5">
                    <span class="text-foreground/50">"Posts today"</span>
                    <span class="text-foreground font-mono">{format_count(stats.posts_today)}</span>
                </div>
                <div class="h-1.5 bg-foreground/10 rounded-full overflow-hidden">
                    <div
                        class="h-full bg-gradient-to-r from-brand to-brand/70 rounded-full"
                        style=format!("width: {}%", (stats.posts_today as f32 / 2000.0 * 100.0).min(100.0))
                    ></div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatCard(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(into)] icon: String,
    #[prop(into)] color: String,
) -> impl IntoView {
    view! {
        <div class="p-3 bg-foreground/[0.02] rounded-xl border border-border/30">
            <div class="flex items-center gap-2 mb-1">
                <span class=format!("w-4 h-4 {}", color) inner_html=icon></span>
                <span class="text-xs text-foreground/50">{label}</span>
            </div>
            <div class="text-lg font-bold text-foreground font-mono">{value}</div>
        </div>
    }
}

fn format_count(count: u32) -> String {
    if count >= 1000 {
        format!("{:.1}K", count as f64 / 1000.0)
    } else {
        count.to_string()
    }
}
