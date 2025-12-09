// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{FontSize, UserTheme};
use leptos::prelude::*;

/// Appearance settings panel
#[component]
pub fn AppearanceSettings() -> impl IntoView {
    let (theme, set_theme) = signal(UserTheme::Midnight);
    let (font_size, set_font_size) = signal(FontSize::Medium);
    let (compact_mode, set_compact_mode) = signal(false);
    let (reduce_animations, set_reduce_animations) = signal(false);
    let (high_contrast, set_high_contrast) = signal(false);
    let (show_avatars, set_show_avatars) = signal(true);
    let (show_previews, set_show_previews) = signal(true);
    let (accent_color, set_accent_color) = signal("#6366f1".to_string());

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Appearance Settings"</h2>
                <p class="text-sm text-foreground/50">"Customize your visual experience"</p>
            </div>

            // Theme explanation banner
            <div class="bg-gradient-to-r from-brand/10 via-violet-500/5 to-transparent border border-brand/20 rounded-2xl p-4">
                <div class="flex items-start gap-3">
                    <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                        </svg>
                    </div>
                    <div>
                        <h3 class="font-semibold text-foreground mb-1">"Two Theme Systems"</h3>
                        <p class="text-sm text-foreground/60 mb-2">
                            "Your app has two separate theme systems:"
                        </p>
                        <ul class="text-sm text-foreground/50 space-y-1">
                            <li class="flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-brand"></span>
                                <span><strong class="text-foreground/70">"User Theme"</strong>" — Controls your personal app experience (Dashboard, Messages, Settings, etc.)"</span>
                            </li>
                            <li class="flex items-center gap-2">
                                <span class="w-2 h-2 rounded-full bg-violet-400"></span>
                                <span><strong class="text-foreground/70">"Synapse Theme"</strong>" — Set by each Synapse's host, creating unique community experiences"</span>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>

            // User Theme Selection
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <div class="flex items-center justify-between">
                    <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                        <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"/>
                        </svg>
                        "Your Theme"
                    </h3>
                    <span class="text-xs text-foreground/40 bg-foreground/5 px-2 py-1 rounded-lg">"App-wide"</span>
                </div>
                <p class="text-xs text-foreground/50 -mt-3">
                    "Applies to Control Panel, Dashboard, Messages, Notifications, Search, Profile, and Settings"
                </p>

                <div class="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-6 gap-3">
                    {UserTheme::all().into_iter().map(|option| {
                        let (bg, card, accent) = option.colors();
                        
                        view! {
                            <button
                                class=move || format!(
                                    "relative p-1 rounded-xl transition-all {}",
                                    if theme.get() == option {
                                        "ring-2 ring-brand ring-offset-2 ring-offset-card"
                                    } else {
                                        "ring-1 ring-border/50 hover:ring-border"
                                    }
                                )
                                on:click=move |_| set_theme.set(option)
                            >
                                // Theme preview
                                <div 
                                    class="w-full aspect-[4/3] rounded-lg overflow-hidden"
                                    style=format!("background-color: {}", bg)
                                >
                                    <div class="p-2 space-y-1.5">
                                        <div 
                                            class="h-2 w-8 rounded"
                                            style=format!("background-color: {}", card)
                                        ></div>
                                        <div 
                                            class="h-2 w-12 rounded"
                                            style=format!("background-color: {}", card)
                                        ></div>
                                        <div 
                                            class="h-2 w-6 rounded"
                                            style=format!("background-color: {}", accent)
                                        ></div>
                                    </div>
                                </div>
                                <p class="text-xs text-center mt-2 text-foreground/70 font-medium">{option.label()}</p>
                                {move || {
                                    if theme.get() == option {
                                        view! {
                                            <div class="absolute -top-1 -right-1 w-5 h-5 bg-brand rounded-full flex items-center justify-center">
                                                <svg class="w-3 h-3 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="3">
                                                    <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                                </svg>
                                            </div>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Current theme info
                <div class="p-3 bg-foreground/5 rounded-xl">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-sm font-medium text-foreground">{move || theme.get().label()}</p>
                            <p class="text-xs text-foreground/50">{move || theme.get().description()}</p>
                        </div>
                        <div class="flex gap-1">
                            {move || {
                                let (bg, card, accent) = theme.get().colors();
                                view! {
                                    <span class="w-6 h-6 rounded-lg border border-border/30" style=format!("background-color: {}", bg) title="Background"></span>
                                    <span class="w-6 h-6 rounded-lg border border-border/30" style=format!("background-color: {}", card) title="Card"></span>
                                    <span class="w-6 h-6 rounded-lg border border-border/30" style=format!("background-color: {}", accent) title="Accent"></span>
                                }
                            }}
                        </div>
                    </div>
                </div>
            </div>

            // Synapse Theme Info
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <div class="flex items-center justify-between">
                    <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                        <svg class="w-4 h-4 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>
                        </svg>
                        "Synapse Themes"
                    </h3>
                    <span class="text-xs text-violet-400 bg-violet-500/10 px-2 py-1 rounded-lg">"Per-Synapse"</span>
                </div>
                
                <div class="p-4 bg-violet-500/5 border border-violet-500/20 rounded-xl">
                    <p class="text-sm text-foreground/70 mb-3">
                        "Each Synapse can have its own unique theme, configured by the Synapse host. When you visit different Synapses, they may look completely different!"
                    </p>
                    <div class="flex flex-wrap gap-2">
                        <span class="px-2 py-1 text-xs bg-foreground/5 rounded-lg text-foreground/50">"Cyberpunk"</span>
                        <span class="px-2 py-1 text-xs bg-foreground/5 rounded-lg text-foreground/50">"Nature"</span>
                        <span class="px-2 py-1 text-xs bg-foreground/5 rounded-lg text-foreground/50">"Ocean"</span>
                        <span class="px-2 py-1 text-xs bg-foreground/5 rounded-lg text-foreground/50">"Sunset"</span>
                        <span class="px-2 py-1 text-xs bg-foreground/5 rounded-lg text-foreground/50">"Retro"</span>
                        <span class="px-2 py-1 text-xs bg-foreground/5 rounded-lg text-foreground/50">"+ more"</span>
                    </div>
                </div>
            </div>

            // Accent Color
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M7 21a4 4 0 01-4-4V5a2 2 0 012-2h4a2 2 0 012 2v12a4 4 0 01-4 4zm0 0h12a2 2 0 002-2v-4a2 2 0 00-2-2h-2.343M11 7.343l1.657-1.657a2 2 0 012.828 0l2.829 2.829a2 2 0 010 2.828l-8.486 8.485M7 17h.01"/>
                    </svg>
                    "Custom Accent Color"
                </h3>
                <p class="text-xs text-foreground/50 -mt-3">
                    "Override the theme's accent color (optional)"
                </p>

                <div class="flex flex-wrap gap-3">
                    {[
                        ("#6366f1", "Indigo"),
                        ("#8b5cf6", "Violet"),
                        ("#ec4899", "Pink"),
                        ("#ef4444", "Red"),
                        ("#f97316", "Orange"),
                        ("#eab308", "Yellow"),
                        ("#22c55e", "Green"),
                        ("#06b6d4", "Cyan"),
                        ("#3b82f6", "Blue"),
                    ].into_iter().map(|(color, name)| {
                        view! {
                            <button
                                class=move || format!(
                                    "w-10 h-10 rounded-xl transition-all {}",
                                    if accent_color.get() == color {
                                        "ring-2 ring-offset-2 ring-offset-card ring-foreground/50 scale-110"
                                    } else {
                                        "hover:scale-105"
                                    }
                                )
                                style=format!("background-color: {}", color)
                                title=name
                                on:click=move |_| set_accent_color.set(color.to_string())
                            >
                                {move || {
                                    if accent_color.get() == color {
                                        view! {
                                            <svg class="w-5 h-5 text-white mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7"/>
                                            </svg>
                                        }.into_any()
                                    } else {
                                        view! { <span></span> }.into_any()
                                    }
                                }}
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Font Size
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-5">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M4 6h16M4 12h16m-7 6h7"/>
                    </svg>
                    "Font Size"
                </h3>

                <div class="space-y-4">
                    <div class="flex gap-2">
                        {FontSize::all().into_iter().map(|size| {
                            let text_class = match size {
                                FontSize::Small => "text-sm",
                                FontSize::Medium => "text-base",
                                FontSize::Large => "text-lg",
                                FontSize::ExtraLarge => "text-xl",
                            };
                            
                            view! {
                                <button
                                    class=move || format!(
                                        "flex-1 px-4 py-3 rounded-xl font-medium transition-all {}",
                                        if font_size.get() == size {
                                            "bg-brand text-white"
                                        } else {
                                            "bg-foreground/5 text-foreground/60 hover:bg-foreground/10 hover:text-foreground"
                                        }
                                    )
                                    on:click=move |_| set_font_size.set(size)
                                >
                                    <span class=text_class>"Aa"</span>
                                </button>
                            }
                        }).collect_view()}
                    </div>

                    // Preview
                    <div class="p-4 bg-foreground/5 rounded-xl">
                        <p class="text-xs text-foreground/40 mb-2">"Preview"</p>
                        <p class=move || format!(
                            "text-foreground {}",
                            match font_size.get() {
                                FontSize::Small => "text-sm",
                                FontSize::Medium => "text-base",
                                FontSize::Large => "text-lg",
                                FontSize::ExtraLarge => "text-xl",
                            }
                        )>
                            "The quick brown fox jumps over the lazy dog."
                        </p>
                    </div>
                </div>
            </div>

            // Display Options
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                    </svg>
                    "Display Options"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Compact Mode"
                        description="Use smaller spacing and elements"
                        checked=compact_mode
                        on_change=move |v| set_compact_mode.set(v)
                    />
                    <ToggleSetting
                        label="Show Avatars"
                        description="Display user avatars in feeds and lists"
                        checked=show_avatars
                        on_change=move |v| set_show_avatars.set(v)
                    />
                    <ToggleSetting
                        label="Link Previews"
                        description="Show previews for shared links"
                        checked=show_previews
                        on_change=move |v| set_show_previews.set(v)
                    />
                </div>
            </div>

            // Accessibility
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                    </svg>
                    "Accessibility"
                </h3>

                <div class="space-y-3">
                    <ToggleSetting
                        label="Reduce Animations"
                        description="Minimize motion for accessibility"
                        checked=reduce_animations
                        on_change=move |v| set_reduce_animations.set(v)
                    />
                    <ToggleSetting
                        label="High Contrast"
                        description="Increase contrast for better visibility"
                        checked=high_contrast
                        on_change=move |v| set_high_contrast.set(v)
                    />
                </div>
            </div>

            // Reset
            <div class="flex justify-end">
                <button class="px-4 py-2 text-sm text-foreground/60 hover:text-foreground hover:bg-foreground/5 rounded-xl transition-colors">
                    "Reset to Defaults"
                </button>
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
