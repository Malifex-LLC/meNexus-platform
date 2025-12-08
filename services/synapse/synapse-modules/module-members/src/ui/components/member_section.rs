// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::types::{Member, MemberRole};
use crate::ui::components::member_card::MemberCard;
use leptos::prelude::*;

/// A collapsible section for grouping members by role
#[component]
pub fn MemberSection(
    #[prop(into)] role: MemberRole,
    #[prop(into)] members: Vec<Member>,
    /// Whether to show in compact mode
    #[prop(into, optional)] compact: Option<bool>,
    /// Whether the section starts collapsed
    #[prop(into, optional)] default_collapsed: Option<bool>,
) -> impl IntoView {
    let compact = compact.unwrap_or(false);
    let default_collapsed = default_collapsed.unwrap_or(false);
    let (is_collapsed, set_is_collapsed) = signal(default_collapsed);
    let member_count = members.len();
    let role_label = role.label();
    let role_classes = role.badge_classes();
    let role_icon = role.icon_svg();

    view! {
        <div class="mb-4">
            // Section header
            <button
                class="w-full flex items-center gap-2 px-2 py-1.5 rounded-lg hover:bg-foreground/5 transition-colors group"
                on:click=move |_| set_is_collapsed.update(|v| *v = !*v)
            >
                // Collapse indicator
                <svg 
                    class=move || format!(
                        "w-3 h-3 text-foreground/40 transition-transform {}",
                        if is_collapsed.get() { "-rotate-90" } else { "" }
                    )
                    fill="none" 
                    viewBox="0 0 24 24" 
                    stroke="currentColor" 
                    stroke-width="2.5"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                </svg>

                // Role icon and label
                <div class=format!(
                    "flex items-center gap-1.5 px-1.5 py-0.5 rounded border {}",
                    role_classes
                )>
                    <span inner_html=role_icon></span>
                    <span class="text-xs font-medium">{role_label}</span>
                </div>

                // Member count
                <span class="text-xs text-foreground/40 font-mono">
                    "("{ member_count }")"
                </span>

                // Decorative line
                <div class="flex-1 h-px bg-border/30 group-hover:bg-border/50 transition-colors"></div>
            </button>

            // Member list
            <div class=move || format!(
                "overflow-hidden transition-all duration-200 {}",
                if is_collapsed.get() { "max-h-0 opacity-0" } else { "max-h-[2000px] opacity-100" }
            )>
                <div class=if compact { "mt-1 space-y-0.5" } else { "mt-2 space-y-2 pl-3" }>
                    {members.into_iter().map(|member| {
                        view! {
                            <MemberCard member=member compact=compact />
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}

/// A section that shows all online members or all offline members
#[component]
pub fn OnlineStatusSection(
    #[prop(into)] title: String,
    #[prop(into)] members_by_role: Vec<(MemberRole, Vec<Member>)>,
    #[prop(into)] is_online: bool,
    /// Whether to show in compact mode
    #[prop(into, optional)] compact: Option<bool>,
    /// Whether the section starts collapsed
    #[prop(into, optional)] default_collapsed: Option<bool>,
) -> impl IntoView {
    let compact = compact.unwrap_or(false);
    let default_collapsed = default_collapsed.unwrap_or(false);
    let (is_collapsed, set_is_collapsed) = signal(default_collapsed);
    
    let total_count: usize = members_by_role.iter().map(|(_, m)| m.len()).sum();

    view! {
        <div class="mb-2">
            // Section header
            <button
                class="w-full flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-foreground/5 transition-colors group"
                on:click=move |_| set_is_collapsed.update(|v| *v = !*v)
            >
                // Collapse indicator
                <svg 
                    class=move || format!(
                        "w-4 h-4 text-foreground/40 transition-transform {}",
                        if is_collapsed.get() { "-rotate-90" } else { "" }
                    )
                    fill="none" 
                    viewBox="0 0 24 24" 
                    stroke="currentColor" 
                    stroke-width="2"
                >
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19 9l-7 7-7-7"/>
                </svg>

                // Online/Offline indicator
                <span class=format!(
                    "w-2 h-2 rounded-full {}",
                    if is_online { "bg-emerald-400" } else { "bg-foreground/30" }
                )></span>

                // Title
                <span class=format!(
                    "font-semibold text-sm {}",
                    if is_online { "text-foreground" } else { "text-foreground/60" }
                )>
                    {title}
                </span>

                // Count badge
                <span class=format!(
                    "px-1.5 py-0.5 rounded-full text-xs font-mono {}",
                    if is_online { 
                        "bg-emerald-500/15 text-emerald-400" 
                    } else { 
                        "bg-foreground/10 text-foreground/40" 
                    }
                )>
                    {total_count}
                </span>

                // Decorative line
                <div class="flex-1 h-px bg-border/30 group-hover:bg-border/50 transition-colors"></div>
            </button>

            // Role sections
            <div class=move || format!(
                "overflow-hidden transition-all duration-300 {}",
                if is_collapsed.get() { "max-h-0 opacity-0" } else { "max-h-[5000px] opacity-100" }
            )>
                <div class="pl-4 pt-2">
                    {members_by_role.into_iter().map(|(role, members)| {
                        view! {
                            <MemberSection 
                                role=role 
                                members=members 
                                compact=compact 
                            />
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}
