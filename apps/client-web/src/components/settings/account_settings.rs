// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

/// Account settings panel
#[component]
pub fn AccountSettings() -> impl IntoView {
    let (show_password_form, set_show_password_form) = signal(false);
    let (show_delete_modal, set_show_delete_modal) = signal(false);

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Account Settings"</h2>
                <p class="text-sm text-foreground/50">"Manage your account credentials and preferences"</p>
            </div>

            // Email section
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                    </svg>
                    "Email Address"
                </h3>

                <div class="flex items-center justify-between p-4 bg-foreground/5 rounded-xl">
                    <div>
                        <p class="text-foreground font-medium">"neo@matrix.network"</p>
                        <div class="flex items-center gap-2 mt-1">
                            <span class="flex items-center gap-1 text-xs text-emerald-400">
                                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                </svg>
                                "Verified"
                            </span>
                            <span class="text-xs text-foreground/40">"Primary email"</span>
                        </div>
                    </div>
                    <button class="px-3 py-1.5 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 rounded-lg transition-colors">
                        "Change"
                    </button>
                </div>

                <button class="text-sm text-brand hover:underline">"+ Add backup email"</button>
            </div>

            // Password section
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                    </svg>
                    "Password"
                </h3>

                {move || {
                    if !show_password_form.get() {
                        view! {
                            <div class="flex items-center justify-between p-4 bg-foreground/5 rounded-xl">
                                <div>
                                    <p class="text-foreground font-medium">"••••••••••••"</p>
                                    <p class="text-xs text-foreground/40 mt-1">"Last changed 3 months ago"</p>
                                </div>
                                <button
                                    class="px-3 py-1.5 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 rounded-lg transition-colors"
                                    on:click=move |_| set_show_password_form.set(true)
                                >
                                    "Change"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="space-y-4 p-4 bg-foreground/5 rounded-xl">
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-foreground">"Current Password"</label>
                                    <input
                                        type="password"
                                        class="w-full bg-card border border-border/50 rounded-xl px-4 py-2.5 text-foreground focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                                        placeholder="Enter current password"
                                    />
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-foreground">"New Password"</label>
                                    <input
                                        type="password"
                                        class="w-full bg-card border border-border/50 rounded-xl px-4 py-2.5 text-foreground focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                                        placeholder="Enter new password"
                                    />
                                    <div class="flex gap-1 mt-2">
                                        <div class="flex-1 h-1 rounded-full bg-emerald-400"></div>
                                        <div class="flex-1 h-1 rounded-full bg-emerald-400"></div>
                                        <div class="flex-1 h-1 rounded-full bg-emerald-400"></div>
                                        <div class="flex-1 h-1 rounded-full bg-foreground/10"></div>
                                    </div>
                                    <p class="text-xs text-foreground/40">"Strong password"</p>
                                </div>
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-foreground">"Confirm New Password"</label>
                                    <input
                                        type="password"
                                        class="w-full bg-card border border-border/50 rounded-xl px-4 py-2.5 text-foreground focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                                        placeholder="Confirm new password"
                                    />
                                </div>
                                <div class="flex gap-2 pt-2">
                                    <button
                                        class="px-4 py-2 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 rounded-xl transition-colors"
                                        on:click=move |_| set_show_password_form.set(false)
                                    >
                                        "Cancel"
                                    </button>
                                    <button class="px-4 py-2 bg-brand hover:bg-brand/90 text-white rounded-xl text-sm font-medium transition-colors">
                                        "Update Password"
                                    </button>
                                </div>
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Two-Factor Authentication
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start justify-between gap-4">
                    <div class="flex gap-4">
                        <div class="w-10 h-10 rounded-xl bg-emerald-500/15 flex items-center justify-center flex-shrink-0">
                            <svg class="w-5 h-5 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                            </svg>
                        </div>
                        <div>
                            <h3 class="font-semibold text-foreground">"Two-Factor Authentication"</h3>
                            <p class="text-sm text-foreground/50 mt-1">"Add an extra layer of security to your account using TOTP."</p>
                            <div class="flex items-center gap-2 mt-2">
                                <span class="flex items-center gap-1 text-xs text-emerald-400 bg-emerald-500/15 px-2 py-0.5 rounded-lg">
                                    <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                    </svg>
                                    "Enabled"
                                </span>
                                <span class="text-xs text-foreground/40">"via Authenticator App"</span>
                            </div>
                        </div>
                    </div>
                    <button class="px-3 py-1.5 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 rounded-lg transition-colors">
                        "Manage"
                    </button>
                </div>
            </div>

            // Recovery Options
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
                    </svg>
                    "Recovery Options"
                </h3>

                <div class="space-y-3">
                    <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
                            </svg>
                            <div>
                                <p class="text-sm font-medium text-foreground">"Recovery Codes"</p>
                                <p class="text-xs text-foreground/40">"8 codes remaining"</p>
                            </div>
                        </div>
                        <button class="text-sm text-brand hover:underline">"View"</button>
                    </div>

                    <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
                        <div class="flex items-center gap-3">
                            <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"/>
                            </svg>
                            <div>
                                <p class="text-sm font-medium text-foreground">"Recovery Phone"</p>
                                <p class="text-xs text-foreground/40">"Not configured"</p>
                            </div>
                        </div>
                        <button class="text-sm text-brand hover:underline">"Add"</button>
                    </div>
                </div>
            </div>

            // Danger Zone
            <div class="bg-card border border-rose-500/30 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-rose-400 flex items-center gap-2">
                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                    </svg>
                    "Danger Zone"
                </h3>

                <div class="space-y-3">
                    <div class="flex items-center justify-between p-4 border border-border/30 rounded-xl">
                        <div>
                            <p class="text-sm font-medium text-foreground">"Deactivate Account"</p>
                            <p class="text-xs text-foreground/40">"Temporarily disable your account. You can reactivate anytime."</p>
                        </div>
                        <button class="px-3 py-1.5 text-sm text-foreground/60 hover:text-amber-400 hover:bg-amber-500/10 border border-border/50 rounded-lg transition-colors">
                            "Deactivate"
                        </button>
                    </div>

                    <div class="flex items-center justify-between p-4 border border-rose-500/30 rounded-xl bg-rose-500/5">
                        <div>
                            <p class="text-sm font-medium text-foreground">"Delete Account"</p>
                            <p class="text-xs text-foreground/40">"Permanently delete your account and all data. This cannot be undone."</p>
                        </div>
                        <button
                            class="px-3 py-1.5 text-sm text-rose-400 hover:bg-rose-500/15 border border-rose-500/30 rounded-lg transition-colors"
                            on:click=move |_| set_show_delete_modal.set(true)
                        >
                            "Delete"
                        </button>
                    </div>
                </div>
            </div>

            // Delete confirmation modal (simplified - shown inline for demo)
            {move || {
                if show_delete_modal.get() {
                    view! {
                        <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50 p-4">
                            <div class="bg-panel border border-border/50 rounded-2xl p-6 max-w-md w-full shadow-2xl">
                                <div class="flex items-center gap-3 mb-4">
                                    <div class="w-10 h-10 rounded-full bg-rose-500/15 flex items-center justify-center">
                                        <svg class="w-5 h-5 text-rose-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"/>
                                        </svg>
                                    </div>
                                    <h3 class="text-lg font-semibold text-foreground">"Delete Account?"</h3>
                                </div>
                                <p class="text-sm text-foreground/60 mb-4">
                                    "This action is irreversible. All your data, posts, and connections will be permanently deleted."
                                </p>
                                <div class="space-y-3 mb-6">
                                    <label class="text-sm font-medium text-foreground">"Type your handle to confirm:"</label>
                                    <input
                                        type="text"
                                        class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-foreground placeholder-foreground/30 focus:outline-none focus:border-rose-500/50 transition-all font-mono"
                                        placeholder="@neo"
                                    />
                                </div>
                                <div class="flex gap-3">
                                    <button
                                        class="flex-1 px-4 py-2 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 rounded-xl transition-colors"
                                        on:click=move |_| set_show_delete_modal.set(false)
                                    >
                                        "Cancel"
                                    </button>
                                    <button class="flex-1 px-4 py-2 bg-rose-500 hover:bg-rose-600 text-white rounded-xl text-sm font-medium transition-colors">
                                        "Delete Forever"
                                    </button>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
