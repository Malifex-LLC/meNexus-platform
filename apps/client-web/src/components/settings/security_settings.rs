// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{ActiveSession, ConnectedApp};
use leptos::prelude::*;

/// Security settings panel
#[component]
pub fn SecuritySettings() -> impl IntoView {
    let sessions = ActiveSession::mock_sessions();
    let apps = ConnectedApp::mock_apps();

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Security Settings"</h2>
                <p class="text-sm text-foreground/50">"Manage your sessions, devices, and connected applications"</p>
            </div>

            // Active Sessions
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="px-6 py-4 border-b border-border/30 flex items-center justify-between">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center">
                            <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                            </svg>
                        </div>
                        <div>
                            <h3 class="font-semibold text-foreground">"Active Sessions"</h3>
                            <p class="text-sm text-foreground/50">"Devices where you're currently logged in"</p>
                        </div>
                    </div>
                    <button class="text-sm text-rose-400 hover:text-rose-300 transition-colors">
                        "Sign out all"
                    </button>
                </div>

                <div class="divide-y divide-border/30">
                    {sessions.into_iter().map(|session| {
                        let device_icon = match session.device_type.as_str() {
                            "mobile" => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"/>"#,
                            _ => r#"<path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>"#,
                        };
                        
                        view! {
                            <div class="px-6 py-4 flex items-center justify-between">
                                <div class="flex items-center gap-4">
                                    <div class=format!(
                                        "w-10 h-10 rounded-xl flex items-center justify-center {}",
                                        if session.is_current { "bg-emerald-500/15" } else { "bg-foreground/5" }
                                    )>
                                        <svg
                                            class=format!(
                                                "w-5 h-5 {}",
                                                if session.is_current { "text-emerald-400" } else { "text-foreground/40" }
                                            )
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                            stroke-width="2"
                                            inner_html=device_icon
                                        ></svg>
                                    </div>
                                    <div>
                                        <div class="flex items-center gap-2">
                                            <p class="font-medium text-foreground">{session.device_name}</p>
                                            {if session.is_current {
                                                view! {
                                                    <span class="text-xs text-emerald-400 bg-emerald-500/15 px-2 py-0.5 rounded-lg">
                                                        "Current"
                                                    </span>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                        <div class="flex items-center gap-2 text-xs text-foreground/40 mt-0.5">
                                            <span>{session.location}</span>
                                            <span>"·"</span>
                                            <span class="font-mono">{session.ip_address}</span>
                                            <span>"·"</span>
                                            <span>{session.last_active}</span>
                                        </div>
                                    </div>
                                </div>
                                {if !session.is_current {
                                    view! {
                                        <button class="px-3 py-1.5 text-sm text-foreground/60 hover:text-rose-400 hover:bg-rose-500/10 rounded-lg transition-colors">
                                            "Revoke"
                                        </button>
                                    }.into_any()
                                } else {
                                    view! { <span></span> }.into_any()
                                }}
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Connected Applications
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="px-6 py-4 border-b border-border/30">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl bg-violet-500/15 flex items-center justify-center">
                            <svg class="w-5 h-5 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6zM14 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2V6zM4 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2v-2zM14 16a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2h-2a2 2 0 01-2-2v-2z"/>
                            </svg>
                        </div>
                        <div>
                            <h3 class="font-semibold text-foreground">"Connected Applications"</h3>
                            <p class="text-sm text-foreground/50">"Third-party apps with access to your account"</p>
                        </div>
                    </div>
                </div>

                <div class="divide-y divide-border/30">
                    {apps.into_iter().map(|app| {
                        let permissions = app.permissions.clone();
                        view! {
                            <div class="px-6 py-4">
                                <div class="flex items-start justify-between gap-4">
                                    <div class="flex items-start gap-4">
                                        <div class="w-12 h-12 rounded-xl bg-foreground/5 flex items-center justify-center flex-shrink-0">
                                            <svg class="w-6 h-6 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M4 6a2 2 0 012-2h2a2 2 0 012 2v2a2 2 0 01-2 2H6a2 2 0 01-2-2V6z"/>
                                            </svg>
                                        </div>
                                        <div>
                                            <p class="font-medium text-foreground">{app.name}</p>
                                            <p class="text-sm text-foreground/50">{app.description}</p>
                                            <div class="flex flex-wrap gap-1.5 mt-2">
                                                {permissions.into_iter().map(|perm| {
                                                    view! {
                                                        <span class="text-xs text-foreground/50 bg-foreground/5 px-2 py-0.5 rounded">
                                                            {perm}
                                                        </span>
                                                    }
                                                }).collect_view()}
                                            </div>
                                            <p class="text-xs text-foreground/30 mt-2">
                                                "Connected "{app.connected_date}" · Last used "{app.last_used}
                                            </p>
                                        </div>
                                    </div>
                                    <button class="px-3 py-1.5 text-sm text-rose-400 hover:bg-rose-500/10 rounded-lg transition-colors flex-shrink-0">
                                        "Revoke"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>

                <div class="px-6 py-3 border-t border-border/30 bg-foreground/[0.02]">
                    <a href="/settings/applications" class="text-sm text-brand hover:underline">"Manage all applications →"</a>
                </div>
            </div>

            // Security Keys
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start gap-4">
                    <div class="w-10 h-10 rounded-xl bg-amber-500/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="font-semibold text-foreground">"Hardware Security Keys"</h3>
                        <p class="text-sm text-foreground/50 mt-1">"Add a hardware security key (like YubiKey) for additional protection."</p>
                        <div class="flex items-center gap-2 mt-3">
                            <button class="px-4 py-2 bg-brand/15 text-brand rounded-xl text-sm font-medium hover:bg-brand/25 transition-colors">
                                "Add Security Key"
                            </button>
                            <span class="text-xs text-foreground/40">"0 keys registered"</span>
                        </div>
                    </div>
                </div>
            </div>

            // Login History
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                        "Recent Login Activity"
                    </h3>
                    <button class="text-xs text-brand hover:underline">"View all"</button>
                </div>

                <div class="space-y-3">
                    {[
                        ("Successful login", "MacBook Pro", "San Francisco, CA", "Just now", true),
                        ("Successful login", "iPhone 15", "San Francisco, CA", "2 hours ago", true),
                        ("Failed login attempt", "Unknown", "Moscow, Russia", "Yesterday", false),
                        ("Successful login", "Firefox on Linux", "New York, NY", "3 days ago", true),
                    ].into_iter().map(|(action, device, location, time, success)| {
                        view! {
                            <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
                                <div class="flex items-center gap-3">
                                    <div class=format!(
                                        "w-8 h-8 rounded-lg flex items-center justify-center {}",
                                        if success { "bg-emerald-500/15" } else { "bg-rose-500/15" }
                                    )>
                                        <svg
                                            class=format!("w-4 h-4 {}", if success { "text-emerald-400" } else { "text-rose-400" })
                                            fill="none"
                                            viewBox="0 0 24 24"
                                            stroke="currentColor"
                                            stroke-width="2"
                                        >
                                            {if success {
                                                view! {
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                                }.into_any()
                                            }}
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-sm font-medium text-foreground">{action}</p>
                                        <p class="text-xs text-foreground/40">{device}" · "{location}</p>
                                    </div>
                                </div>
                                <span class="text-xs text-foreground/40">{time}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            // API Keys (for developers)
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start gap-4">
                    <div class="w-10 h-10 rounded-xl bg-emerald-500/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="font-semibold text-foreground">"API Keys"</h3>
                        <p class="text-sm text-foreground/50 mt-1">"Manage API keys for programmatic access to your account."</p>
                        <div class="flex items-center gap-2 mt-3">
                            <button class="px-4 py-2 bg-foreground/5 text-foreground/70 rounded-xl text-sm font-medium hover:bg-foreground/10 hover:text-foreground transition-colors border border-border/50">
                                "Generate New Key"
                            </button>
                            <span class="text-xs text-foreground/40">"1 active key"</span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
