// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::notification_cards::*;
use super::types::*;
use leptos::prelude::*;
use leptos_router::components::A;

/// Scrollable panel for network broadcasts
#[component]
pub fn BroadcastsPanel() -> impl IntoView {
    let alerts = BroadcastAlert::mock_alerts();
    let unread_count = alerts.iter().filter(|a| !a.is_read).count();

    view! {
        <div class="flex flex-col h-full bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Panel header
            <div class="flex-shrink-0 flex items-center justify-between p-4 border-b border-border/30 bg-gradient-to-r from-brand/5 to-transparent">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-brand/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="font-semibold text-foreground">"Network Broadcasts"</h2>
                        <p class="text-xs text-foreground/50">"Official announcements"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-brand/15 text-brand text-xs font-semibold animate-pulse">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Scrollable content
            <div class="flex-1 overflow-y-auto scrollbar-thin p-3 space-y-3">
                {alerts.into_iter().map(|alert| {
                    view! { <BroadcastAlertCard alert=alert /> }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Scrollable panel for content notifications
#[component]
pub fn ContentPanel() -> impl IntoView {
    let notifications = ContentNotification::mock_notifications();
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();

    view! {
        <div class="flex flex-col h-full bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Panel header
            <div class="flex-shrink-0 flex items-center justify-between p-4 border-b border-border/30 bg-gradient-to-r from-rose-500/5 to-transparent">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-rose-500/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="font-semibold text-foreground">"Content Activity"</h2>
                        <p class="text-xs text-foreground/50">"Likes, comments, shares"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-rose-500/15 text-rose-400 text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Scrollable content
            <div class="flex-1 overflow-y-auto scrollbar-thin divide-y divide-border/20">
                {notifications.into_iter().map(|notification| {
                    view! { <ContentNotificationCard notification=notification /> }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Scrollable panel for connection notifications
#[component]
pub fn ConnectionsPanel() -> impl IntoView {
    let notifications = ConnectionNotification::mock_notifications();
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();

    view! {
        <div class="flex flex-col h-full bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Panel header
            <div class="flex-shrink-0 flex items-center justify-between p-4 border-b border-border/30 bg-gradient-to-r from-sky-500/5 to-transparent">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-sky-500/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-sky-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="font-semibold text-foreground">"Connections"</h2>
                        <p class="text-xs text-foreground/50">"Followers & invites"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-sky-500/15 text-sky-400 text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Scrollable content
            <div class="flex-1 overflow-y-auto scrollbar-thin divide-y divide-border/20">
                {notifications.into_iter().map(|notification| {
                    view! { <ConnectionNotificationCard notification=notification /> }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Scrollable panel for system notifications
#[component]
pub fn SystemPanel() -> impl IntoView {
    let notifications = SystemNotification::mock_notifications();
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();

    view! {
        <div class="flex flex-col h-full bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Panel header
            <div class="flex-shrink-0 flex items-center justify-between p-4 border-b border-border/30 bg-gradient-to-r from-stat-positive/5 to-transparent">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-stat-positive/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-stat-positive" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="font-semibold text-foreground">"System & Security"</h2>
                        <p class="text-xs text-foreground/50">"Account activity"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-stat-positive/15 text-stat-positive text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>

            // Scrollable content
            <div class="flex-1 overflow-y-auto scrollbar-thin divide-y divide-border/20">
                {notifications.into_iter().map(|notification| {
                    view! { <SystemNotificationCard notification=notification /> }
                }).collect_view()}
            </div>
        </div>
    }
}

/// Detailed analytics panel
#[component]
pub fn AnalyticsPanel() -> impl IntoView {
    view! {
        <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Header
            <div class="flex items-center justify-between p-4 border-b border-border/30 bg-gradient-to-r from-violet-500/5 to-transparent">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-violet-500/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="font-semibold text-foreground">"Engagement Analytics"</h2>
                        <p class="text-xs text-foreground/50">"Your activity insights"</p>
                    </div>
                </div>
                <select class="bg-foreground/5 border border-border/50 rounded-lg px-2 py-1 text-xs text-foreground/70 focus:outline-none focus:border-brand/50">
                    <option>"This Week"</option>
                    <option>"This Month"</option>
                    <option>"Last 30 Days"</option>
                    <option>"All Time"</option>
                </select>
            </div>

            // Stats grid
            <div class="p-4 space-y-4">
                // Top row - key metrics
                <div class="grid grid-cols-2 lg:grid-cols-4 gap-3">
                    <div class="p-3 rounded-xl bg-foreground/[0.03] border border-border/30">
                        <div class="flex items-center gap-2 mb-2">
                            <svg class="w-4 h-4 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                            </svg>
                            <span class="text-xs text-foreground/50">"Total Likes"</span>
                        </div>
                        <p class="text-2xl font-bold text-foreground">"1,247"</p>
                        <p class="text-xs text-stat-positive mt-1">"↑ 12.4% vs last week"</p>
                    </div>

                    <div class="p-3 rounded-xl bg-foreground/[0.03] border border-border/30">
                        <div class="flex items-center gap-2 mb-2">
                            <svg class="w-4 h-4 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                            </svg>
                            <span class="text-xs text-foreground/50">"Comments"</span>
                        </div>
                        <p class="text-2xl font-bold text-foreground">"342"</p>
                        <p class="text-xs text-stat-positive mt-1">"↑ 8.2% vs last week"</p>
                    </div>

                    <div class="p-3 rounded-xl bg-foreground/[0.03] border border-border/30">
                        <div class="flex items-center gap-2 mb-2">
                            <svg class="w-4 h-4 text-sky-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                            </svg>
                            <span class="text-xs text-foreground/50">"New Followers"</span>
                        </div>
                        <p class="text-2xl font-bold text-foreground">"89"</p>
                        <p class="text-xs text-stat-positive mt-1">"↑ 24.1% vs last week"</p>
                    </div>

                    <div class="p-3 rounded-xl bg-foreground/[0.03] border border-border/30">
                        <div class="flex items-center gap-2 mb-2">
                            <svg class="w-4 h-4 text-stat-positive" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/>
                            </svg>
                            <span class="text-xs text-foreground/50">"Shares"</span>
                        </div>
                        <p class="text-2xl font-bold text-foreground">"156"</p>
                        <p class="text-xs text-stat-negative mt-1">"↓ 3.2% vs last week"</p>
                    </div>
                </div>

                // Engagement breakdown
                <div class="p-4 rounded-xl bg-foreground/[0.02] border border-border/30">
                    <h4 class="text-sm font-medium text-foreground mb-3">"Engagement by Type"</h4>
                    <div class="space-y-3">
                        <div>
                            <div class="flex items-center justify-between text-xs mb-1">
                                <span class="text-foreground/60">"Reactions"</span>
                                <span class="text-foreground font-medium">"68%"</span>
                            </div>
                            <div class="h-2 bg-foreground/10 rounded-full overflow-hidden">
                                <div class="h-full w-[68%] bg-gradient-to-r from-rose-500 to-rose-400 rounded-full"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex items-center justify-between text-xs mb-1">
                                <span class="text-foreground/60">"Comments"</span>
                                <span class="text-foreground font-medium">"19%"</span>
                            </div>
                            <div class="h-2 bg-foreground/10 rounded-full overflow-hidden">
                                <div class="h-full w-[19%] bg-gradient-to-r from-violet-500 to-violet-400 rounded-full"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex items-center justify-between text-xs mb-1">
                                <span class="text-foreground/60">"Shares"</span>
                                <span class="text-foreground font-medium">"8%"</span>
                            </div>
                            <div class="h-2 bg-foreground/10 rounded-full overflow-hidden">
                                <div class="h-full w-[8%] bg-gradient-to-r from-stat-positive to-stat-primary rounded-full"></div>
                            </div>
                        </div>
                        <div>
                            <div class="flex items-center justify-between text-xs mb-1">
                                <span class="text-foreground/60">"Mentions"</span>
                                <span class="text-foreground font-medium">"5%"</span>
                            </div>
                            <div class="h-2 bg-foreground/10 rounded-full overflow-hidden">
                                <div class="h-full w-[5%] bg-gradient-to-r from-amber-500 to-amber-400 rounded-full"></div>
                            </div>
                        </div>
                    </div>
                </div>

                // Peak hours
                <div class="p-4 rounded-xl bg-foreground/[0.02] border border-border/30">
                    <h4 class="text-sm font-medium text-foreground mb-3">"Peak Activity Hours"</h4>
                    <div class="flex items-end justify-between gap-1 h-16">
                        {["6am", "9am", "12pm", "3pm", "6pm", "9pm", "12am"].into_iter().enumerate().map(|(i, label)| {
                            let heights = [15, 45, 65, 55, 85, 100, 35];
                            let height = heights.get(i).unwrap_or(&50);
                            view! {
                                <div class="flex-1 flex flex-col items-center gap-1">
                                    <div
                                        class="w-full bg-gradient-to-t from-brand to-brand/60 rounded-t-sm transition-all hover:from-brand/80"
                                        style=format!("height: {}%", height)
                                    ></div>
                                    <span class="text-[10px] text-foreground/40">{label}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Quick actions panel
#[component]
pub fn QuickActionsPanel() -> impl IntoView {
    view! {
        <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Header
            <div class="flex items-center justify-between p-4 border-b border-border/30 bg-gradient-to-r from-amber-500/5 to-transparent">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-amber-500/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="font-semibold text-foreground">"Quick Actions"</h2>
                        <p class="text-xs text-foreground/50">"Manage notifications"</p>
                    </div>
                </div>
            </div>

            // Actions grid
            <div class="p-4 space-y-3">
                // Primary actions
                <div class="grid grid-cols-2 gap-2">
                    <button class="flex items-center gap-2 p-3 rounded-xl bg-brand/10 border border-brand/20 text-brand hover:bg-brand/15 transition-colors">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                        </svg>
                        <span class="text-sm font-medium">"Mark All Read"</span>
                    </button>
                    <button class="flex items-center gap-2 p-3 rounded-xl bg-foreground/5 border border-border/50 text-foreground/70 hover:bg-foreground/10 hover:text-foreground transition-colors">
                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M3 4a1 1 0 011-1h16a1 1 0 011 1v2.586a1 1 0 01-.293.707l-6.414 6.414a1 1 0 00-.293.707V17l-4 4v-6.586a1 1 0 00-.293-.707L3.293 7.293A1 1 0 013 6.586V4z"/>
                        </svg>
                        <span class="text-sm font-medium">"Filter"</span>
                    </button>
                </div>

                // Secondary actions
                <div class="space-y-2">
                    <A href="/settings/notifications" attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-foreground/5 transition-colors group">
                        <div class="w-10 h-10 rounded-xl bg-violet-500/10 flex items-center justify-center group-hover:bg-violet-500/15 transition-colors">
                            <svg class="w-5 h-5 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </div>
                        <div class="flex-1">
                            <p class="text-sm font-medium text-foreground">"Notification Settings"</p>
                            <p class="text-xs text-foreground/50">"Configure alerts, sounds, and preferences"</p>
                        </div>
                        <svg class="w-4 h-4 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                        </svg>
                    </A>

                    <button class="w-full flex items-center gap-3 p-3 rounded-xl hover:bg-foreground/5 transition-colors group">
                        <div class="w-10 h-10 rounded-xl bg-sky-500/10 flex items-center justify-center group-hover:bg-sky-500/15 transition-colors">
                            <svg class="w-5 h-5 text-sky-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15.536a5 5 0 001.414 1.414m2.828-9.9a9 9 0 0112.728 0"/>
                            </svg>
                        </div>
                        <div class="flex-1 text-left">
                            <p class="text-sm font-medium text-foreground">"Mute for 1 Hour"</p>
                            <p class="text-xs text-foreground/50">"Temporarily pause all notifications"</p>
                        </div>
                    </button>

                    <button class="w-full flex items-center gap-3 p-3 rounded-xl hover:bg-foreground/5 transition-colors group">
                        <div class="w-10 h-10 rounded-xl bg-status-online/10 flex items-center justify-center group-hover:bg-stat-positive/15 transition-colors">
                            <svg class="w-5 h-5 text-status-online" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"/>
                            </svg>
                        </div>
                        <div class="flex-1 text-left">
                            <p class="text-sm font-medium text-foreground">"Export History"</p>
                            <p class="text-xs text-foreground/50">"Download notification archive"</p>
                        </div>
                    </button>

                    <button class="w-full flex items-center gap-3 p-3 rounded-xl hover:bg-foreground/5 transition-colors group">
                        <div class="w-10 h-10 rounded-xl bg-rose-500/10 flex items-center justify-center group-hover:bg-rose-500/15 transition-colors">
                            <svg class="w-5 h-5 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                            </svg>
                        </div>
                        <div class="flex-1 text-left">
                            <p class="text-sm font-medium text-foreground">"Clear Old Notifications"</p>
                            <p class="text-xs text-foreground/50">"Remove notifications older than 30 days"</p>
                        </div>
                    </button>
                </div>

                // Notification summary
                <div class="mt-4 p-3 rounded-xl bg-foreground/[0.02] border border-border/30">
                    <div class="flex items-center justify-between text-xs mb-2">
                        <span class="text-foreground/50">"Storage Used"</span>
                        <span class="text-foreground font-mono">"2.4 MB / 50 MB"</span>
                    </div>
                    <div class="h-1.5 bg-foreground/10 rounded-full overflow-hidden">
                        <div class="h-full w-[4.8%] bg-brand rounded-full"></div>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Status overview widget
#[component]
pub fn StatusOverview() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-br from-brand/10 via-violet-500/5 to-stat-positive/5 border border-border/50 rounded-2xl p-4">
            <div class="flex flex-col sm:flex-row sm:items-center justify-between gap-4">
                // Unread summary
                <div class="flex items-center gap-4">
                    <div class="relative">
                        <div class="w-14 h-14 rounded-2xl bg-brand/20 flex items-center justify-center">
                            <svg class="w-7 h-7 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
                            </svg>
                        </div>
                        <span class="absolute -top-1 -right-1 w-6 h-6 rounded-full bg-brand text-white text-xs font-bold flex items-center justify-center animate-pulse">
                            "17"
                        </span>
                    </div>
                    <div>
                        <h3 class="text-lg font-bold text-foreground">"17 Unread Notifications"</h3>
                        <p class="text-sm text-foreground/50">"Last checked 5 minutes ago"</p>
                    </div>
                </div>

                // Quick stats row
                <div class="flex items-center gap-4 text-sm">
                    <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-rose-500/10 border border-rose-500/20">
                        <svg class="w-4 h-4 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                        </svg>
                        <span class="font-medium text-rose-400">"3"</span>
                    </div>
                    <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-sky-500/10 border border-sky-500/20">
                        <svg class="w-4 h-4 text-sky-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                        </svg>
                        <span class="font-medium text-sky-400">"5"</span>
                    </div>
                    <div class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-brand/10 border border-brand/20">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>
                        </svg>
                        <span class="font-medium text-brand">"1"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}

// Keep the old section components for backwards compatibility if needed
/// Section for network-wide broadcast alerts
#[component]
pub fn BroadcastsSection(#[prop(optional)] show_all: bool) -> impl IntoView {
    let alerts = BroadcastAlert::mock_alerts();
    let unread_count = alerts.iter().filter(|a| !a.is_read).count();
    let display_alerts = if show_all {
        alerts
    } else {
        alerts.into_iter().take(3).collect()
    };

    view! {
        <section class="space-y-4">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-brand/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="text-lg font-semibold text-foreground">"Network Broadcasts"</h2>
                        <p class="text-xs text-foreground/50">"Official announcements from meNexus"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-brand/15 text-brand text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>
            <div class="space-y-3">
                {display_alerts.into_iter().map(|alert| {
                    view! { <BroadcastAlertCard alert=alert /> }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Section for content-related notifications
#[component]
pub fn ContentSection(#[prop(optional)] show_all: bool) -> impl IntoView {
    let notifications = ContentNotification::mock_notifications();
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();
    let display_notifications = if show_all {
        notifications
    } else {
        notifications.into_iter().take(5).collect()
    };

    view! {
        <section class="space-y-4">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-rose-500/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="text-lg font-semibold text-foreground">"Content Activity"</h2>
                        <p class="text-xs text-foreground/50">"Likes, comments, shares, and mentions"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-rose-500/15 text-rose-400 text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>
            <div class="bg-card border border-border/50 rounded-xl divide-y divide-border/30 overflow-hidden">
                {display_notifications.into_iter().map(|notification| {
                    view! { <ContentNotificationCard notification=notification /> }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Section for connection-related notifications
#[component]
pub fn ConnectionsSection(#[prop(optional)] show_all: bool) -> impl IntoView {
    let notifications = ConnectionNotification::mock_notifications();
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();
    let display_notifications = if show_all {
        notifications
    } else {
        notifications.into_iter().take(5).collect()
    };

    view! {
        <section class="space-y-4">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-sky-500/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-sky-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="text-lg font-semibold text-foreground">"Connections"</h2>
                        <p class="text-xs text-foreground/50">"Followers, invites, and network activity"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-sky-500/15 text-sky-400 text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>
            <div class="bg-card border border-border/50 rounded-xl divide-y divide-border/30 overflow-hidden">
                {display_notifications.into_iter().map(|notification| {
                    view! { <ConnectionNotificationCard notification=notification /> }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Section for system notifications
#[component]
pub fn SystemSection(#[prop(optional)] show_all: bool) -> impl IntoView {
    let notifications = SystemNotification::mock_notifications();
    let unread_count = notifications.iter().filter(|n| !n.is_read).count();
    let display_notifications = if show_all {
        notifications
    } else {
        notifications.into_iter().take(4).collect()
    };

    view! {
        <section class="space-y-4">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-3">
                    <div class="w-9 h-9 rounded-xl bg-status-online/15 flex items-center justify-center">
                        <svg class="w-5 h-5 text-status-online" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                    </div>
                    <div>
                        <h2 class="text-lg font-semibold text-foreground">"System & Security"</h2>
                        <p class="text-xs text-foreground/50">"Account activity and security alerts"</p>
                    </div>
                </div>
                {if unread_count > 0 {
                    view! {
                        <span class="px-2.5 py-1 rounded-full bg-status-online/15 text-status-online text-xs font-semibold">
                            {unread_count}" new"
                        </span>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
            </div>
            <div class="bg-card border border-border/50 rounded-xl divide-y divide-border/30 overflow-hidden">
                {display_notifications.into_iter().map(|notification| {
                    view! { <SystemNotificationCard notification=notification /> }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Sidebar with notification filters and quick stats (kept for backwards compatibility)
#[component]
pub fn NotificationsSidebar(active_category: RwSignal<NotificationCategory>) -> impl IntoView {
    let categories = NotificationCategory::all();
    let counts: Vec<(NotificationCategory, usize)> = vec![
        (NotificationCategory::All, 17),
        (NotificationCategory::Broadcasts, 1),
        (NotificationCategory::Content, 8),
        (NotificationCategory::Connections, 5),
        (NotificationCategory::System, 3),
    ];

    view! {
        <aside class="w-64 flex-shrink-0 space-y-4">
            <div class="bg-card border border-border/50 rounded-2xl p-4">
                <h3 class="text-xs font-semibold text-foreground/40 uppercase tracking-wider mb-3">"Categories"</h3>
                <nav class="space-y-1">
                    {categories.into_iter().map(|cat| {
                        let count = counts.iter().find(|(c, _)| *c == cat).map(|(_, c)| *c).unwrap_or(0);
                        view! {
                            <button
                                class=move || format!(
                                    "w-full flex items-center justify-between gap-2 px-3 py-2.5 rounded-xl text-sm font-medium transition-all {}",
                                    if active_category.get() == cat {
                                        "bg-brand/15 text-brand"
                                    } else {
                                        "text-foreground/60 hover:bg-foreground/5 hover:text-foreground"
                                    }
                                )
                                on:click=move |_| active_category.set(cat)
                            >
                                <div class="flex items-center gap-2.5">
                                    <svg
                                        class=move || format!(
                                            "w-4 h-4 {}",
                                            if active_category.get() == cat { "text-brand" } else { "text-foreground/40" }
                                        )
                                        fill="none"
                                        viewBox="0 0 24 24"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        inner_html=cat.icon_svg()
                                    ></svg>
                                    <span>{cat.label()}</span>
                                </div>
                                {if count > 0 {
                                    view! {
                                        <span class=move || format!(
                                            "px-2 py-0.5 rounded-full text-xs font-medium {}",
                                            if active_category.get() == cat {
                                                "bg-brand/20 text-brand"
                                            } else {
                                                "bg-foreground/10 text-foreground/50"
                                            }
                                        )>
                                            {count}
                                        </span>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </button>
                        }
                    }).collect_view()}
                </nav>
            </div>
        </aside>
    }
}

/// All notifications view
#[component]
pub fn AllNotificationsView() -> impl IntoView {
    view! {
        <div class="space-y-8">
            <BroadcastsSection show_all=false />
            <ContentSection show_all=false />
            <ConnectionsSection show_all=false />
            <SystemSection show_all=false />
        </div>
    }
}
