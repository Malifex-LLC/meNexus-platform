// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::reputation_card::ReputationCard;
use super::types::{Credential, ProfileData};
use leptos::prelude::*;

/// Overview tab content - bio, basic info, and reputation
#[component]
pub fn ProfileOverview(#[prop(into)] profile: ProfileData) -> impl IntoView {
    let bio = profile.bio.clone();
    let location = profile.location.clone();
    let joined_at = profile.joined_at.clone();
    let reputation = profile.reputation.clone();
    let credentials = Credential::mock_credentials();

    view! {
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 p-4 sm:p-6">
            // Left column - Bio and Info
            <div class="lg:col-span-2 space-y-6">
                // Bio Section
                <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                    <div class="px-4 py-3 border-b border-border/30">
                        <div class="flex items-center gap-2">
                            <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 6a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0zM4.501 20.118a7.5 7.5 0 0114.998 0A17.933 17.933 0 0112 21.75c-2.676 0-5.216-.584-7.499-1.632z"/>
                            </svg>
                            <h3 class="font-semibold text-foreground">"About"</h3>
                        </div>
                    </div>
                    <div class="p-4">
                        {if let Some(bio_text) = bio {
                            view! {
                                <p class="text-foreground/80 leading-relaxed">{bio_text}</p>
                            }.into_any()
                        } else {
                            view! {
                                <p class="text-foreground/40 italic">"No bio yet"</p>
                            }.into_any()
                        }}
                    </div>
                </div>

                // Info Grid
                <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                    <div class="px-4 py-3 border-b border-border/30">
                        <div class="flex items-center gap-2">
                            <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M11.25 11.25l.041-.02a.75.75 0 011.063.852l-.708 2.836a.75.75 0 001.063.853l.041-.021M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9-3.75h.008v.008H12V8.25z"/>
                            </svg>
                            <h3 class="font-semibold text-foreground">"Info"</h3>
                        </div>
                    </div>
                    <div class="p-4 grid grid-cols-1 sm:grid-cols-2 gap-4">
                        // Location
                        {if let Some(loc) = location {
                            view! {
                                <div class="flex items-center gap-3">
                                    <div class="w-10 h-10 rounded-xl bg-foreground/5 flex items-center justify-center">
                                        <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M15 10.5a3 3 0 11-6 0 3 3 0 016 0z"/>
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 10.5c0 7.142-7.5 11.25-7.5 11.25S4.5 17.642 4.5 10.5a7.5 7.5 0 1115 0z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-xs text-foreground/40">"Location"</p>
                                        <p class="text-foreground font-medium">{loc}</p>
                                    </div>
                                </div>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}

                        // Joined date
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 rounded-xl bg-foreground/5 flex items-center justify-center">
                                <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 012.25-2.25h13.5A2.25 2.25 0 0121 7.5v11.25m-18 0A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75m-18 0v-7.5A2.25 2.25 0 015.25 9h13.5A2.25 2.25 0 0121 11.25v7.5"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-xs text-foreground/40">"Joined"</p>
                                <p class="text-foreground font-medium">{joined_at}</p>
                            </div>
                        </div>

                        // Network stats
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 rounded-xl bg-foreground/5 flex items-center justify-center">
                                <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M7.5 21L3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-xs text-foreground/40">"Network"</p>
                                <p class="text-foreground font-medium">"P2P Connected"</p>
                            </div>
                        </div>

                        // Identity status
                        <div class="flex items-center gap-3">
                            <div class="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center">
                                <svg class="w-5 h-5 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M9 12.75L11.25 15 15 9.75m-3-7.036A11.959 11.959 0 013.598 6 11.99 11.99 0 003 9.749c0 5.592 3.824 10.29 9 11.623 5.176-1.332 9-6.03 9-11.622 0-1.31-.21-2.571-.598-3.751h-.152c-3.196 0-6.1-1.248-8.25-3.285z"/>
                                </svg>
                            </div>
                            <div>
                                <p class="text-xs text-foreground/40">"Identity"</p>
                                <p class="text-emerald-400 font-medium">"Verified"</p>
                            </div>
                        </div>
                    </div>
                </div>

                // Credentials Section
                <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                    <div class="px-4 py-3 border-b border-border/30 flex items-center justify-between">
                        <div class="flex items-center gap-2">
                            <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 18.75h-9m9 0a3 3 0 013 3h-15a3 3 0 013-3m9 0v-3.375c0-.621-.503-1.125-1.125-1.125h-.871M7.5 18.75v-3.375c0-.621.504-1.125 1.125-1.125h.872m5.007 0H9.497m5.007 0a7.454 7.454 0 01-.982-3.172M9.497 14.25a7.454 7.454 0 00.981-3.172M5.25 4.236c-.982.143-1.954.317-2.916.52A6.003 6.003 0 007.73 9.728M5.25 4.236V4.5c0 2.108.966 3.99 2.48 5.228M5.25 4.236V2.721C7.456 2.41 9.71 2.25 12 2.25c2.291 0 4.545.16 6.75.47v1.516M7.73 9.728a6.726 6.726 0 002.748 1.35m8.272-6.842V4.5c0 2.108-.966 3.99-2.48 5.228m2.48-5.492a46.32 46.32 0 012.916.52 6.003 6.003 0 01-5.395 4.972m0 0a6.726 6.726 0 01-2.749 1.35m0 0a6.772 6.772 0 01-3.044 0"/>
                            </svg>
                            <h3 class="font-semibold text-foreground">"Credentials"</h3>
                        </div>
                        <span class="text-xs text-foreground/40 font-mono">{credentials.len()}" earned"</span>
                    </div>
                    <div class="p-4">
                        <div class="flex flex-wrap gap-3">
                            {credentials.into_iter().map(|cred| {
                                let rarity_gradient = cred.rarity.color_class();
                                let border_class = cred.rarity.border_class();
                                view! {
                                    <div class=format!(
                                        "group relative flex items-center gap-2 px-3 py-2 rounded-xl border bg-card hover:scale-105 transition-transform cursor-pointer {}",
                                        border_class
                                    )>
                                        // Badge icon
                                        <div class=format!(
                                            "w-8 h-8 rounded-lg bg-gradient-to-br flex items-center justify-center {}",
                                            rarity_gradient
                                        )>
                                            {if cred.is_verified {
                                                view! {
                                                    <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2.5">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5"/>
                                                    </svg>
                                                }.into_any()
                                            } else {
                                                view! {
                                                    <svg class="w-4 h-4 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09z"/>
                                                    </svg>
                                                }.into_any()
                                            }}
                                        </div>
                                        <div>
                                            <p class="text-sm font-medium text-foreground">{cred.name}</p>
                                            <p class="text-xs text-foreground/40">{cred.issuer_name}</p>
                                        </div>
                                        
                                        // Verified indicator
                                        {if cred.is_verified {
                                            view! {
                                                <svg class="w-4 h-4 text-emerald-400 ml-auto" viewBox="0 0 24 24" fill="currentColor">
                                                    <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                                                </svg>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}

                                        // Tooltip on hover
                                        <div class="absolute bottom-full left-1/2 -translate-x-1/2 mb-2 px-3 py-2 bg-panel border border-border/50 rounded-lg shadow-xl opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none whitespace-nowrap z-10">
                                            <p class="text-xs text-foreground/70">{cred.description}</p>
                                            <p class="text-xs text-foreground/40 mt-1">"Earned "{cred.earned_at}</p>
                                        </div>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </div>

            // Right column - Reputation
            <div class="space-y-6">
                <ReputationCard reputation=reputation />
                
                // Quick Stats
                <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                    <div class="px-4 py-3 border-b border-border/30">
                        <h3 class="font-semibold text-foreground">"Activity"</h3>
                    </div>
                    <div class="p-4 space-y-3">
                        <ActivityStat 
                            label="Posts this month"
                            value="47"
                            change="+12%"
                            positive=true
                        />
                        <ActivityStat 
                            label="Interactions"
                            value="1.2K"
                            change="+28%"
                            positive=true
                        />
                        <ActivityStat 
                            label="Avg. engagement"
                            value="8.4%"
                            change="-2%"
                            positive=false
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ActivityStat(
    #[prop(into)] label: String,
    #[prop(into)] value: String,
    #[prop(into)] change: String,
    #[prop(into)] positive: bool,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between">
            <span class="text-sm text-foreground/60">{label}</span>
            <div class="flex items-center gap-2">
                <span class="font-semibold text-foreground">{value}</span>
                <span class=format!(
                    "text-xs font-mono {}",
                    if positive { "text-emerald-400" } else { "text-rose-400" }
                )>
                    {change}
                </span>
            </div>
        </div>
    }
}
