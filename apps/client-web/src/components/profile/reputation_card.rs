// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::Reputation;
use leptos::prelude::*;

/// Reputation display card with score, level, and breakdown
#[component]
pub fn ReputationCard(#[prop(into)] reputation: Reputation) -> impl IntoView {
    let score = reputation.score;
    let level = reputation.level.clone();
    let creator_pct = reputation.creator_percentage;
    let curator_pct = reputation.curator_percentage;
    let endorsements = reputation.peer_endorsements;
    let credentials = reputation.credentials_count;

    view! {
        <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            // Header
            <div class="px-4 py-3 border-b border-border/30 bg-gradient-to-r from-brand/5 to-transparent">
                <div class="flex items-center gap-2">
                    <div class="w-8 h-8 rounded-lg bg-brand/15 flex items-center justify-center">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"/>
                        </svg>
                    </div>
                    <div>
                        <h3 class="font-semibold text-foreground">"Reputation"</h3>
                        <p class="text-xs text-foreground/40">"ELO-based trust score"</p>
                    </div>
                </div>
            </div>

            <div class="p-4 space-y-4">
                // Main score display
                <div class="flex items-center justify-between">
                    <div>
                        <div class="text-4xl font-bold text-foreground tracking-tight">{score}</div>
                        <div class="text-sm text-foreground/50">"reputation score"</div>
                    </div>
                    <div class=format!(
                        "px-3 py-1.5 rounded-xl border flex items-center gap-2 {}",
                        level.color_class()
                    )>
                        <span class="w-5 h-5" inner_html=level.icon_svg()></span>
                        <span class="font-semibold">{level.label()}</span>
                    </div>
                </div>

                // Progress to next level
                <div>
                    <div class="flex items-center justify-between text-xs mb-1.5">
                        <span class="text-foreground/50">"Progress to next level"</span>
                        <span class="text-foreground/70 font-mono">{score}" / 1500"</span>
                    </div>
                    <div class="h-2 bg-foreground/10 rounded-full overflow-hidden">
                        <div 
                            class="h-full bg-gradient-to-r from-brand to-brand/70 rounded-full transition-all duration-500"
                            style=format!("width: {}%", ((score as f32 - 1000.0) / 500.0 * 100.0).min(100.0).max(0.0))
                        ></div>
                    </div>
                </div>

                // Divider
                <div class="h-px bg-border/30"></div>

                // Creator vs Curator ratio
                <div>
                    <div class="flex items-center justify-between mb-2">
                        <span class="text-sm font-medium text-foreground">"Activity Ratio"</span>
                        <span class="text-xs text-foreground/40">"Creates vs. Curates"</span>
                    </div>
                    
                    // Visual bar
                    <div class="flex h-3 rounded-full overflow-hidden mb-2">
                        <div 
                            class="bg-gradient-to-r from-sky-500 to-sky-400 transition-all duration-500"
                            style=format!("width: {}%", creator_pct)
                        ></div>
                        <div 
                            class="bg-gradient-to-r from-violet-500 to-violet-400 transition-all duration-500"
                            style=format!("width: {}%", curator_pct)
                        ></div>
                    </div>
                    
                    // Legend
                    <div class="flex items-center justify-between text-xs">
                        <div class="flex items-center gap-1.5">
                            <span class="w-2.5 h-2.5 rounded-full bg-sky-500"></span>
                            <span class="text-foreground/70">"Creator"</span>
                            <span class="font-mono font-medium text-foreground">{creator_pct}"%"</span>
                        </div>
                        <div class="flex items-center gap-1.5">
                            <span class="w-2.5 h-2.5 rounded-full bg-violet-500"></span>
                            <span class="text-foreground/70">"Curator"</span>
                            <span class="font-mono font-medium text-foreground">{curator_pct}"%"</span>
                        </div>
                    </div>
                </div>

                // Divider
                <div class="h-px bg-border/30"></div>

                // Endorsements and Credentials
                <div class="grid grid-cols-2 gap-3">
                    // Peer Endorsements
                    <div class="p-3 bg-background/50 rounded-xl border border-border/30">
                        <div class="flex items-center gap-2 mb-1">
                            <svg class="w-4 h-4 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M6.633 10.5c.806 0 1.533-.446 2.031-1.08a9.041 9.041 0 012.861-2.4c.723-.384 1.35-.956 1.653-1.715a4.498 4.498 0 00.322-1.672V3a.75.75 0 01.75-.75A2.25 2.25 0 0116.5 4.5c0 1.152-.26 2.243-.723 3.218-.266.558.107 1.282.725 1.282h3.126c1.026 0 1.945.694 2.054 1.715.045.422.068.85.068 1.285a11.95 11.95 0 01-2.649 7.521c-.388.482-.987.729-1.605.729H13.48c-.483 0-.964-.078-1.423-.23l-3.114-1.04a4.501 4.501 0 00-1.423-.23H5.904M14.25 9h2.25M5.904 18.75c.083.205.173.405.27.602.197.4-.078.898-.523.898h-.908c-.889 0-1.713-.518-1.972-1.368a12 12 0 01-.521-3.507c0-1.553.295-3.036.831-4.398C3.387 10.203 4.167 9.75 5 9.75h1.053c.472 0 .745.556.5.96a8.958 8.958 0 00-1.302 4.665c0 1.194.232 2.333.654 3.375z"/>
                            </svg>
                            <span class="text-xs text-foreground/50">"Endorsements"</span>
                        </div>
                        <div class="text-2xl font-bold text-foreground">{endorsements}</div>
                        <div class="text-xs text-emerald-400">"from peers"</div>
                    </div>

                    // Credentials
                    <div class="p-3 bg-background/50 rounded-xl border border-border/30">
                        <div class="flex items-center gap-2 mb-1">
                            <svg class="w-4 h-4 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 18.75h-9m9 0a3 3 0 013 3h-15a3 3 0 013-3m9 0v-3.375c0-.621-.503-1.125-1.125-1.125h-.871M7.5 18.75v-3.375c0-.621.504-1.125 1.125-1.125h.872m5.007 0H9.497m5.007 0a7.454 7.454 0 01-.982-3.172M9.497 14.25a7.454 7.454 0 00.981-3.172M5.25 4.236c-.982.143-1.954.317-2.916.52A6.003 6.003 0 007.73 9.728M5.25 4.236V4.5c0 2.108.966 3.99 2.48 5.228M5.25 4.236V2.721C7.456 2.41 9.71 2.25 12 2.25c2.291 0 4.545.16 6.75.47v1.516M7.73 9.728a6.726 6.726 0 002.748 1.35m8.272-6.842V4.5c0 2.108-.966 3.99-2.48 5.228m2.48-5.492a46.32 46.32 0 012.916.52 6.003 6.003 0 01-5.395 4.972m0 0a6.726 6.726 0 01-2.749 1.35m0 0a6.772 6.772 0 01-3.044 0"/>
                            </svg>
                            <span class="text-xs text-foreground/50">"Credentials"</span>
                        </div>
                        <div class="text-2xl font-bold text-foreground">{credentials}</div>
                        <div class="text-xs text-amber-400">"earned"</div>
                    </div>
                </div>
            </div>
        </div>
    }
}
