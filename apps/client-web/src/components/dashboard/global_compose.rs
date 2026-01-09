// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::SynapseWithChannels;
use leptos::prelude::*;

/// Enhanced compose bar with Synapse and Channel selection
#[component]
pub fn GlobalCompose() -> impl IntoView {
    // Store synapses in a signal so it can be accessed reactively
    let synapses = StoredValue::new(SynapseWithChannels::mock_synapses());

    let (content, set_content) = signal(String::new());
    let (is_expanded, set_is_expanded) = signal(false);
    let (selected_synapse_idx, set_selected_synapse_idx) = signal(0usize);
    let (selected_channel_idx, set_selected_channel_idx) = signal(0usize);
    let (show_synapse_dropdown, set_show_synapse_dropdown) = signal(false);
    let (show_channel_dropdown, set_show_channel_dropdown) = signal(false);

    view! {
        <div class="bg-card border border-border/50 rounded-2xl overflow-hidden transition-all duration-300">
            // Collapsed state - just a prompt
            {move || {
                if !is_expanded.get() {
                    view! {
                        <button
                            class="w-full p-4 flex items-center gap-3 hover:bg-foreground/[0.02] transition-colors"
                            on:click=move |_| set_is_expanded.set(true)
                        >
                            // Avatar placeholder
                            <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 ring-border/30">
                                <span class="text-brand font-bold text-sm">"NA"</span>
                            </div>
                            
                            <span class="flex-1 text-left text-foreground/40">"What's on your mind?"</span>
                            
                            // Quick post indicators
                            <div class="flex items-center gap-2 text-foreground/30">
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                </svg>
                                <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                </svg>
                            </div>
                        </button>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}

            // Expanded state - full composer
            {move || {
                if is_expanded.get() {
                    let synapses_data = synapses.get_value();
                    let synapses_for_dropdown = synapses_data.clone();
                    let synapses_for_channel = synapses_data.clone();
                    let synapses_for_button = synapses_data.clone();
                    let synapses_for_channel_name = synapses_data.clone();
                    
                    // Get current synapse for display
                    let current_syn = synapses_data.get(selected_synapse_idx.get()).cloned();
                    
                    view! {
                        <div class="p-4 space-y-4">
                            // Header with destination selector
                            <div class="flex items-center justify-between">
                                <div class="flex items-center gap-2">
                                    // Synapse selector
                                    <div class="relative">
                                        <button
                                            class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-foreground/5 border border-border/50 hover:bg-foreground/10 hover:border-border transition-all text-sm"
                                            on:click=move |_| {
                                                set_show_synapse_dropdown.update(|v| *v = !*v);
                                                set_show_channel_dropdown.set(false);
                                            }
                                        >
                                            {if let Some(syn) = current_syn.clone() {
                                                let first_char = syn.name.chars().next().unwrap_or('S').to_uppercase().to_string();
                                                view! {
                                                    <div class="w-5 h-5 rounded bg-violet-500/20 flex items-center justify-center">
                                                        <span class="text-violet-400 font-bold text-[10px]">{first_char}</span>
                                                    </div>
                                                    <span class="text-foreground font-medium">{syn.name}</span>
                                                }.into_any()
                                            } else {
                                                view! { <span class="text-foreground/50">"Select Synapse"</span> }.into_any()
                                            }}
                                            <svg class="w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                                            </svg>
                                        </button>

                                        // Synapse dropdown
                                        {move || {
                                            if show_synapse_dropdown.get() {
                                                let list = synapses_for_dropdown.clone();
                                                view! {
                                                    <div class="absolute top-full left-0 mt-1 w-56 bg-panel border border-border/50 rounded-xl shadow-xl z-50 py-1 max-h-64 overflow-y-auto">
                                                        {list.iter().enumerate().map(|(idx, syn)| {
                                                            let name = syn.name.clone();
                                                            let first_char = name.chars().next().unwrap_or('S').to_uppercase().to_string();
                                                            view! {
                                                                <button
                                                                    class=move || format!(
                                                                        "w-full flex items-center gap-2 px-3 py-2 text-sm transition-colors {}",
                                                                        if selected_synapse_idx.get() == idx {
                                                                            "bg-brand/10 text-brand"
                                                                        } else {
                                                                            "text-foreground/70 hover:bg-foreground/5"
                                                                        }
                                                                    )
                                                                    on:click=move |_| {
                                                                        set_selected_synapse_idx.set(idx);
                                                                        set_selected_channel_idx.set(0);
                                                                        set_show_synapse_dropdown.set(false);
                                                                    }
                                                                >
                                                                    <div class="w-6 h-6 rounded bg-violet-500/20 flex items-center justify-center">
                                                                        <span class="text-violet-400 font-bold text-[10px]">{first_char}</span>
                                                                    </div>
                                                                    <span class="flex-1 text-left truncate">{name}</span>
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }}
                                    </div>

                                    <svg class="w-4 h-4 text-foreground/20" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M9 5l7 7-7 7"/>
                                    </svg>

                                    // Channel selector
                                    <div class="relative">
                                        <button
                                            class="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-foreground/5 border border-border/50 hover:bg-foreground/10 hover:border-border transition-all text-sm"
                                            on:click=move |_| {
                                                set_show_channel_dropdown.update(|v| *v = !*v);
                                                set_show_synapse_dropdown.set(false);
                                            }
                                        >
                                            <span class="text-brand font-mono">"#"</span>
                                            <span class="text-foreground font-medium">
                                                {move || {
                                                    let syns = synapses_for_channel_name.clone();
                                                    syns.get(selected_synapse_idx.get())
                                                        .and_then(|s| s.channels.get(selected_channel_idx.get()))
                                                        .map(|c| c.name.clone())
                                                        .unwrap_or_else(|| "general".to_string())
                                                }}
                                            </span>
                                            <svg class="w-4 h-4 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                                            </svg>
                                        </button>

                                        // Channel dropdown
                                        {move || {
                                            if show_channel_dropdown.get() {
                                                let syns = synapses_for_channel.clone();
                                                let channels = syns.get(selected_synapse_idx.get()).map(|s| s.channels.clone()).unwrap_or_default();
                                                view! {
                                                    <div class="absolute top-full left-0 mt-1 w-48 bg-panel border border-border/50 rounded-xl shadow-xl z-50 py-1">
                                                        {channels.into_iter().enumerate().map(|(idx, ch)| {
                                                            let name = ch.name.clone();
                                                            view! {
                                                                <button
                                                                    class=move || format!(
                                                                        "w-full flex items-center gap-2 px-3 py-2 text-sm transition-colors {}",
                                                                        if selected_channel_idx.get() == idx {
                                                                            "bg-brand/10 text-brand"
                                                                        } else {
                                                                            "text-foreground/70 hover:bg-foreground/5"
                                                                        }
                                                                    )
                                                                    on:click=move |_| {
                                                                        set_selected_channel_idx.set(idx);
                                                                        set_show_channel_dropdown.set(false);
                                                                    }
                                                                >
                                                                    <span class="text-brand/60 font-mono">"#"</span>
                                                                    <span class="flex-1 text-left">{name}</span>
                                                                </button>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }
                                        }}
                                    </div>
                                </div>

                                // Close button
                                <button
                                    class="p-1.5 rounded-lg text-foreground/40 hover:text-foreground hover:bg-foreground/5 transition-colors"
                                    on:click=move |_| {
                                        set_is_expanded.set(false);
                                        set_content.set(String::new());
                                    }
                                >
                                    <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
                                    </svg>
                                </button>
                            </div>

                            // Text input
                            <div class="flex gap-3">
                                <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-2 ring-border/30 flex-shrink-0">
                                    <span class="text-brand font-bold text-sm">"NA"</span>
                                </div>
                                <textarea
                                    class="flex-1 bg-transparent text-foreground placeholder-foreground/30 focus:outline-none resize-none text-base leading-relaxed min-h-[100px]"
                                    placeholder="What's happening across the network?"
                                    prop:value=move || content.get()
                                    on:input=move |ev| set_content.set(event_target_value(&ev))
                                ></textarea>
                            </div>

                            // Attachments bar
                            <div class="flex items-center justify-between pt-3 border-t border-border/30">
                                <div class="flex items-center gap-1">
                                    <button class="p-2 rounded-lg text-foreground/40 hover:text-brand hover:bg-brand/10 transition-colors" title="Add image">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                        </svg>
                                    </button>
                                    <button class="p-2 rounded-lg text-foreground/40 hover:text-brand hover:bg-brand/10 transition-colors" title="Add GIF">
                                        <svg class="w-5 h-5" viewBox="0 0 24 24" fill="currentColor">
                                            <path d="M11.5 9H13v6h-1.5V9zM9 9H6c-.6 0-1 .5-1 1v4c0 .5.4 1 1 1h3c.6 0 1-.5 1-1v-2H8.5v1.5h-2v-3H10V10c0-.5-.4-1-1-1zm10 1.5V9h-4.5v6H16v-2h2v-1.5h-2v-1h3z"/>
                                        </svg>
                                    </button>
                                    <button class="p-2 rounded-lg text-foreground/40 hover:text-brand hover:bg-brand/10 transition-colors" title="Add poll">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                        </svg>
                                    </button>
                                    <button class="p-2 rounded-lg text-foreground/40 hover:text-brand hover:bg-brand/10 transition-colors" title="Add emoji">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
                                        </svg>
                                    </button>
                                    <div class="w-px h-5 bg-border/50 mx-1"></div>
                                    <button class="p-2 rounded-lg text-foreground/40 hover:text-brand hover:bg-brand/10 transition-colors" title="Schedule">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                            <path stroke-linecap="round" stroke-linejoin="round" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                        </svg>
                                    </button>
                                </div>

                                <div class="flex items-center gap-2">
                                    // Character count
                                    <span class=move || format!(
                                        "text-xs font-mono {}",
                                        if content.get().len() > 500 { "text-rose-400" } else { "text-foreground/30" }
                                    )>
                                        {move || content.get().len()}"/500"
                                    </span>
                                    
                                    // Post button
                                    <button
                                        class=move || format!(
                                            "px-4 py-2 rounded-xl font-semibold text-sm transition-all {}",
                                            if content.get().trim().is_empty() {
                                                "bg-brand/30 text-white/50 cursor-not-allowed"
                                            } else {
                                                "bg-brand hover:bg-brand/90 text-white shadow-lg shadow-brand/20"
                                            }
                                        )
                                        prop:disabled=move || content.get().trim().is_empty()
                                    >
                                        "Post"
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
