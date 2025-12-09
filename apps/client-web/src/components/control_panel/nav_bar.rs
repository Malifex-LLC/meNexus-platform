// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

#[derive(Clone)]
struct NavItem {
    href: &'static str,
    label: &'static str,
    icon: &'static str,
}

fn get_nav_items() -> Vec<NavItem> {
    vec![
        NavItem {
            href: "/",
            label: "Dashboard",
            icon: r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/></svg>"#,
        },
        NavItem {
            href: "/synapse",
            label: "Synapse",
            icon: r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>"#,
        },
        NavItem {
            href: "/messages",
            label: "Messages",
            icon: r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/></svg>"#,
        },
        NavItem {
            href: "/search",
            label: "Search",
            icon: r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/></svg>"#,
        },
        NavItem {
            href: "/profile",
            label: "Profile",
            icon: r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/></svg>"#,
        },
        NavItem {
            href: "/settings",
            label: "Settings",
            icon: r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/><path stroke-linecap="round" stroke-linejoin="round" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/></svg>"#,
        },
    ]
}

#[component]
pub fn NavBar() -> impl IntoView {
    let location = use_location();
    let nav_items = get_nav_items();

    view! {
        <nav class="space-y-0.5">
            {nav_items.into_iter().map(|item| {
                let href = item.href;
                let current_path = location.pathname.get();
                let is_active = if href == "/" {
                    current_path == "/"
                } else {
                    current_path.starts_with(href)
                };

                view! {
                    <A 
                        href=href
                        attr:class=format!(
                            "group flex items-center gap-2 px-2 py-1.5 rounded-lg transition-all text-sm {}",
                            if is_active {
                                "bg-brand/15 text-brand"
                            } else {
                                "text-foreground/60 hover:text-foreground hover:bg-foreground/5"
                            }
                        )
                    >
                        <div 
                            class=format!(
                                "w-4 h-4 transition-colors {}",
                                if is_active { "text-brand" } else { "text-foreground/50 group-hover:text-foreground/70" }
                            )
                            inner_html=item.icon
                        ></div>
                        <span class="font-medium">{item.label}</span>
                        {if is_active {
                            view! {
                                <span class="ml-auto w-1 h-1 rounded-full bg-brand"></span>
                            }.into_any()
                        } else {
                            view! { <span></span> }.into_any()
                        }}
                    </A>
                }
            }).collect_view()}
        </nav>
    }
}
