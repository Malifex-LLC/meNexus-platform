// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::ProfileSettingsData;
use leptos::prelude::*;

/// Profile settings panel
#[component]
pub fn ProfileSettings() -> impl IntoView {
    let profile = ProfileSettingsData::default();

    let (display_name, set_display_name) = signal(profile.display_name);
    let (handle, set_handle) = signal(profile.handle);
    let (bio, set_bio) = signal(profile.bio);
    let (location, set_location) = signal(profile.location);
    let (website, set_website) = signal(profile.website);
    let (pronouns, set_pronouns) = signal(profile.pronouns);
    let (has_changes, set_has_changes) = signal(false);

    // Track changes
    let mark_changed = move || set_has_changes.set(true);

    view! {
        <div class="space-y-6">
            // Header
            <div class="flex items-center justify-between">
                <div>
                    <h2 class="text-xl font-semibold text-foreground">"Profile Settings"</h2>
                    <p class="text-sm text-foreground/50">"Manage how others see you on the network"</p>
                </div>
                {move || {
                    if has_changes.get() {
                        view! {
                            <div class="flex items-center gap-2">
                                <button
                                    class="px-4 py-2 rounded-xl text-sm font-medium text-foreground/60 hover:text-foreground hover:bg-foreground/5 transition-colors"
                                    on:click=move |_| set_has_changes.set(false)
                                >
                                    "Discard"
                                </button>
                                <button class="px-4 py-2 bg-brand hover:bg-brand/90 text-white rounded-xl text-sm font-medium shadow-lg shadow-brand/20 transition-all">
                                    "Save Changes"
                                </button>
                            </div>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }
                }}
            </div>

            // Avatar and Banner section
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                // Banner
                <div class="relative h-32 bg-gradient-to-br from-brand/20 via-violet-500/20 to-emerald-500/20">
                    <button class="absolute inset-0 flex items-center justify-center bg-black/0 hover:bg-black/30 transition-colors group">
                        <div class="flex items-center gap-2 text-white/0 group-hover:text-white/90 transition-colors">
                            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                            <span class="text-sm font-medium">"Change Banner"</span>
                        </div>
                    </button>
                </div>

                // Avatar
                <div class="px-6 pb-6 -mt-12 relative">
                    <div class="relative w-24 h-24 group">
                        <div class="w-24 h-24 rounded-full bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center ring-4 ring-card">
                            <span class="text-brand font-bold text-2xl">"NA"</span>
                        </div>
                        <button class="absolute inset-0 rounded-full flex items-center justify-center bg-black/0 group-hover:bg-black/50 transition-colors">
                            <svg class="w-6 h-6 text-white/0 group-hover:text-white transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z"/>
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 13a3 3 0 11-6 0 3 3 0 016 0z"/>
                            </svg>
                        </button>
                    </div>
                    <p class="mt-3 text-xs text-foreground/40">"Recommended: Square image, at least 400x400px"</p>
                </div>
            </div>

            // Basic info
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                    </svg>
                    "Basic Information"
                </h3>

                // Display Name
                <div class="space-y-2">
                    <label class="flex items-center justify-between">
                        <span class="text-sm font-medium text-foreground">"Display Name"</span>
                        <span class="text-xs text-foreground/40">{move || display_name.get().len()}"/50"</span>
                    </label>
                    <input
                        type="text"
                        class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                        placeholder="Your display name"
                        maxlength="50"
                        prop:value=move || display_name.get()
                        on:input=move |ev| {
                            set_display_name.set(event_target_value(&ev));
                            mark_changed();
                        }
                    />
                    <p class="text-xs text-foreground/40">"This is how your name appears to others"</p>
                </div>

                // Handle
                <div class="space-y-2">
                    <label class="flex items-center justify-between">
                        <span class="text-sm font-medium text-foreground">"Handle"</span>
                        <span class="text-xs text-foreground/40">"@"{move || handle.get()}</span>
                    </label>
                    <div class="relative">
                        <span class="absolute left-4 top-1/2 -translate-y-1/2 text-brand font-mono">"@"</span>
                        <input
                            type="text"
                            class="w-full bg-foreground/5 border border-border/50 rounded-xl pl-8 pr-4 py-2.5 text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all font-mono"
                            placeholder="handle"
                            maxlength="30"
                            prop:value=move || handle.get()
                            on:input=move |ev| {
                                set_handle.set(event_target_value(&ev));
                                mark_changed();
                            }
                        />
                    </div>
                    <p class="text-xs text-foreground/40">"Your unique identifier on the network. Changing this may break existing links."</p>
                </div>

                // Bio
                <div class="space-y-2">
                    <label class="flex items-center justify-between">
                        <span class="text-sm font-medium text-foreground">"Bio"</span>
                        <span class="text-xs text-foreground/40">{move || bio.get().len()}"/300"</span>
                    </label>
                    <textarea
                        class="w-full bg-foreground/5 border border-border/50 rounded-xl px-4 py-2.5 text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all resize-none min-h-[100px]"
                        placeholder="Tell others about yourself..."
                        maxlength="300"
                        prop:value=move || bio.get()
                        on:input=move |ev| {
                            set_bio.set(event_target_value(&ev));
                            mark_changed();
                        }
                    ></textarea>
                    <p class="text-xs text-foreground/40">"A brief description that appears on your profile"</p>
                </div>
                // Location
                <div class="space-y-2">
                    <label class="text-sm font-medium text-foreground">"Location"</label>
                    <div class="relative">
                        <svg class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"/>
                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"/>
                        </svg>
                        <input
                            type="text"
                            class="w-full bg-foreground/5 border border-border/50 rounded-xl pl-10 pr-4 py-2.5 text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                            placeholder="City, Country"
                            prop:value=move || location.get()
                            on:input=move |ev| {
                                set_location.set(event_target_value(&ev));
                                mark_changed();
                            }
                        />
                    </div>
                </div>
            </div>

            // Links
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/>
                    </svg>
                    "Links"
                </h3>


                // Website
                <div class="space-y-2">
                    <label class="text-sm font-medium text-foreground">"Website"</label>
                    <div class="relative">
                        <svg class="absolute left-4 top-1/2 -translate-y-1/2 w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9"/>
                        </svg>
                        <input
                            type="url"
                            class="w-full bg-foreground/5 border border-border/50 rounded-xl pl-10 pr-4 py-2.5 text-foreground placeholder-foreground/30 focus:outline-none focus:border-brand/50 focus:ring-2 focus:ring-brand/20 transition-all"
                            placeholder="https://your-website.com"
                            prop:value=move || website.get()
                            on:input=move |ev| {
                                set_website.set(event_target_value(&ev));
                                mark_changed();
                            }
                        />
                    </div>
                </div>

                // Social links
                <div class="space-y-3">
                    <div class="flex items-center justify-between">
                        <label class="text-sm font-medium text-foreground">"Social Links"</label>
                        <button class="text-xs text-brand hover:underline">"+ Add link"</button>
                    </div>
                    <div class="space-y-2">
                        <div class="flex items-center gap-2 p-3 bg-foreground/5 rounded-xl">
                            <svg class="w-5 h-5 text-foreground/50" viewBox="0 0 24 24" fill="currentColor">
                                <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-5.214-6.817L4.99 21.75H1.68l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
                            </svg>
                            <span class="flex-1 text-sm text-foreground font-mono">"@neo_matrix"</span>
                            <button class="p-1 text-foreground/40 hover:text-rose-400 transition-colors">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                </svg>
                            </button>
                        </div>
                        <div class="flex items-center gap-2 p-3 bg-foreground/5 rounded-xl">
                            <svg class="w-5 h-5 text-foreground/50" viewBox="0 0 24 24" fill="currentColor">
                                <path fill-rule="evenodd" d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z" clip-rule="evenodd"/>
                            </svg>
                            <span class="flex-1 text-sm text-foreground font-mono">"github.com/neo"</span>
                            <button class="p-1 text-foreground/40 hover:text-rose-400 transition-colors">
                                <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            // Identity verification
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start gap-4">
                    <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-brand" viewBox="0 0 24 24" fill="currentColor">
                            <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h3 class="font-semibold text-foreground">"Identity Verification"</h3>
                        <p class="text-sm text-foreground/50 mt-1">"Verify your identity to get a checkmark badge and build trust with the community."</p>
                        <button class="mt-3 px-4 py-2 bg-brand/15 text-brand rounded-xl text-sm font-medium hover:bg-brand/25 transition-colors">
                            "Start Verification"
                        </button>
                    </div>
                </div>
            </div>

            // Public key info
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2 mb-4">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 7a2 2 0 012 2m4 0a6 6 0 01-7.743 5.743L11 17H9v2H7v2H4a1 1 0 01-1-1v-2.586a1 1 0 01.293-.707l5.964-5.964A6 6 0 1121 9z"/>
                    </svg>
                    "Cryptographic Identity"
                </h3>
                <div class="space-y-3">
                    <div class="p-3 bg-foreground/5 rounded-xl">
                        <p class="text-xs text-foreground/40 mb-1">"Public Key (Ed25519)"</p>
                        <p class="text-xs text-foreground font-mono break-all">"ed25519:HjK9mN2pL4qR8sT6wX1yZ3bV5cF7gD9hJ2kM4nP6qS8t"</p>
                    </div>
                    <div class="flex items-center gap-2 text-xs text-foreground/40">
                        <svg class="w-4 h-4 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                        </svg>
                        <span>"Your identity is cryptographically secured and portable"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
