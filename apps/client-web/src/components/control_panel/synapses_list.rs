// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use leptos_router::components::A;

#[derive(Clone)]
pub struct JoinedSynapse {
    pub public_key: String,
    pub name: String,
    pub icon: Option<String>,
    pub member_count: u32,
    pub unread_count: u32,
    pub is_online: bool,
}

fn get_mock_synapses() -> Vec<JoinedSynapse> {
    vec![
        JoinedSynapse {
            public_key: "syn_02a3b5c7d9e1f2a4b6c8".to_string(),
            name: "Zion HQ".to_string(),
            icon: None,
            member_count: 1_247,
            unread_count: 5,
            is_online: true,
        },
        JoinedSynapse {
            public_key: "syn_03c4d6e8f0a2b4c6d8e0".to_string(),
            name: "Nebuchadnezzar".to_string(),
            icon: None,
            member_count: 8,
            unread_count: 0,
            is_online: true,
        },
        JoinedSynapse {
            public_key: "syn_04e5f7a9b1c3d5e7f9a1".to_string(),
            name: "Oracle's Temple".to_string(),
            icon: None,
            member_count: 342,
            unread_count: 12,
            is_online: true,
        },
        JoinedSynapse {
            public_key: "syn_05f6a8b0c2d4e6f8a0b2".to_string(),
            name: "Construct".to_string(),
            icon: None,
            member_count: 56,
            unread_count: 0,
            is_online: false,
        },
    ]
}

#[component]
pub fn SynapsesList() -> impl IntoView {
    let (is_expanded, set_is_expanded) = signal(true);
    let synapses = get_mock_synapses();
    let total_unread: u32 = synapses.iter().map(|s| s.unread_count).sum();

    view! {
        <div class="space-y-1">
            // Header
            <button 
                class="w-full flex items-center justify-between px-1 py-1 text-[10px] font-medium text-foreground/40 uppercase tracking-wider hover:text-foreground/60 transition-colors"
                on:click=move |_| set_is_expanded.update(|v| *v = !*v)
            >
                <div class="flex items-center gap-1.5">
                    <svg class="w-3 h-3 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                    </svg>
                    <span>"Synapses"</span>
                    <span class="text-foreground/30">"("{synapses.len()}")"</span>
                    {if total_unread > 0 {
                        view! {
                            <span class="min-w-4 h-4 px-1 flex items-center justify-center rounded-full bg-brand text-white text-[10px] font-bold">
                                {total_unread}
                            </span>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <svg 
                    class=move || format!("w-3 h-3 transition-transform {}", if is_expanded.get() { "" } else { "-rotate-90" })
                    fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"></path>
                </svg>
            </button>

            // List
            {move || {
                if is_expanded.get() {
                    view! {
                        <div class="space-y-0.5">
                            {synapses.iter().map(|synapse| {
                                let name = synapse.name.clone();
                                let public_key = synapse.public_key.clone();
                                let unread = synapse.unread_count;
                                let is_online = synapse.is_online;
                                
                                let first_char = name.chars().next().unwrap_or('S').to_uppercase().to_string();

                                view! {
                                    <A 
                                        href=format!("/synapses/{}", public_key)
                                        attr:class="group flex items-center gap-2 px-2 py-1 rounded-lg hover:bg-foreground/5 transition-colors"
                                    >
                                        // Icon
                                        <div class="relative flex-shrink-0">
                                            <div class="w-6 h-6 rounded bg-gradient-to-br from-violet-500/20 to-violet-500/5 flex items-center justify-center">
                                                <span class="text-violet-400 font-bold text-[10px]">{first_char}</span>
                                            </div>
                                            {if is_online {
                                                view! {
                                                    <div class="absolute -bottom-0.5 -right-0.5 w-2 h-2 bg-emerald-500 rounded-full border border-panel"></div>
                                                }.into_any()
                                            } else {
                                                view! { <span></span> }.into_any()
                                            }}
                                        </div>
                                        
                                        <span class="flex-1 text-xs text-foreground/70 group-hover:text-foreground truncate">{name}</span>
                                        
                                        {if unread > 0 {
                                            view! {
                                                <span class="min-w-4 h-4 px-1 flex items-center justify-center rounded-full bg-violet-500 text-white text-[10px] font-bold">
                                                    {unread}
                                                </span>
                                            }.into_any()
                                        } else {
                                            view! { <span></span> }.into_any()
                                        }}
                                    </A>
                                }
                            }).collect_view()}
                            
                            <A href="/synapses" attr:class="flex items-center gap-2 px-2 py-1 text-[10px] text-foreground/40 hover:text-brand transition-colors">
                                <svg class="w-3 h-3" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
                                </svg>
                                "Browse synapses"
                            </A>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }
            }}
        </div>
    }
}
