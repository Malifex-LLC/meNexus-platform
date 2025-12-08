// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

/// Represents a module that can be displayed in the ModuleContainer
#[derive(Clone)]
pub struct ModuleTab {
    /// Unique identifier for this module
    pub id: &'static str,
    /// Display name for the tab
    pub name: &'static str,
    /// SVG icon path (the d attribute for the path element)
    pub icon: &'static str,
    /// Optional badge count (e.g., unread messages)
    pub badge: Option<Signal<u32>>,
}

impl ModuleTab {
    pub fn new(id: &'static str, name: &'static str, icon: &'static str) -> Self {
        Self {
            id,
            name,
            icon,
            badge: None,
        }
    }

    pub fn with_badge(mut self, badge: Signal<u32>) -> Self {
        self.badge = Some(badge);
        self
    }
}

/// Pre-defined module tab configurations
pub mod tabs {
    use super::ModuleTab;

    pub fn posts() -> ModuleTab {
        ModuleTab::new(
            "posts",
            "Posts",
            // Hash/pound icon for channels
            "M7 20l4-16m2 16l4-16M6 9h14M4 15h14",
        )
    }

    pub fn chat() -> ModuleTab {
        ModuleTab::new(
            "chat",
            "Chat",
            // Chat bubble icon
            "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z",
        )
    }

    pub fn activity() -> ModuleTab {
        ModuleTab::new(
            "activity",
            "Activity",
            // Activity/pulse icon
            "M13 10V3L4 14h7v7l9-11h-7z",
        )
    }

    pub fn files() -> ModuleTab {
        ModuleTab::new(
            "files",
            "Files",
            // Folder icon
            "M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z",
        )
    }

    pub fn media() -> ModuleTab {
        ModuleTab::new(
            "media",
            "Media",
            // Image icon
            "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
        )
    }

    pub fn calendar() -> ModuleTab {
        ModuleTab::new(
            "calendar",
            "Calendar",
            // Calendar icon
            "M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z",
        )
    }

    pub fn livestream() -> ModuleTab {
        ModuleTab::new(
            "livestream",
            "Live",
            // Video camera icon
            "M15.75 10.5l4.72-4.72a.75.75 0 011.28.53v11.38a.75.75 0 01-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 002.25-2.25v-9a2.25 2.25 0 00-2.25-2.25h-9A2.25 2.25 0 002.25 7.5v9a2.25 2.25 0 002.25 2.25z",
        )
    }

    pub fn creator() -> ModuleTab {
        ModuleTab::new(
            "creator",
            "Creator",
            // Star/sparkle icon for creators
            "M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z",
        )
    }
}

/// Signal type for active tab - provided via context
pub type ActiveTabSignal = RwSignal<&'static str>;

/// Container component for tabbed modules
#[component]
pub fn TabbedModules(
    /// The module tabs to display
    #[prop(into)]
    tabs: Vec<ModuleTab>,
    /// Optional: starting active tab (defaults to first)
    #[prop(into, optional)]
    default_active: Option<&'static str>,
    /// The children - ModulePanel components
    children: Children,
) -> impl IntoView {
    let default = default_active.unwrap_or_else(|| tabs.first().map(|t| t.id).unwrap_or(""));
    let active_tab = RwSignal::new(default);

    // Provide context so ModulePanel can access active state
    provide_context(active_tab);

    view! {
        <div class="flex flex-col h-full w-full overflow-hidden bg-background">
            // Tab bar
            <div class="flex-shrink-0 flex items-center gap-1 px-3 py-2 border-b border-border/50 bg-panel/50 backdrop-blur-sm">
                // Module tabs
                <div class="flex items-center">
                    {tabs.iter().map(|tab| {
                        let tab_id = tab.id;
                        let tab_name = tab.name;
                        let tab_icon = tab.icon;
                        let tab_badge = tab.badge;

                        view! {
                            <button
                                class=move || format!(
                                    "group relative flex items-center gap-2 px-4 py-2 rounded-t-lg text-sm font-medium transition-all duration-200 -mb-px {}",
                                    if active_tab.get() == tab_id {
                                        "bg-background text-brand border border-border/50 border-b-background z-10"
                                    } else {
                                        "text-foreground/50 hover:text-foreground bg-transparent border border-transparent"
                                    }
                                )
                                on:click=move |_| active_tab.set(tab_id)
                                title=tab_name
                            >
                                // Icon
                                <svg
                                    class=move || format!(
                                        "w-5 h-5 transition-all {}",
                                        if active_tab.get() == tab_id {
                                            "text-brand scale-110"
                                        } else {
                                            "text-foreground/40 group-hover:text-foreground/60"
                                        }
                                    )
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    stroke="currentColor"
                                    stroke-width="2"
                                >
                                    <path stroke-linecap="round" stroke-linejoin="round" d=tab_icon></path>
                                </svg>

                                    // Name
                                    <span class=move || format!(
                                        "transition-colors {}",
                                        if active_tab.get() == tab_id { "text-brand" } else { "" }
                                    )>
                                        {tab_name}
                                    </span>

                                // Badge
                                {move || {
                                    if let Some(badge_signal) = tab_badge {
                                        let count = badge_signal.get();
                                        if count > 0 {
                                            return view! {
                                                <span class="min-w-5 h-5 px-1.5 flex items-center justify-center rounded-full bg-rose-500 text-white text-xs font-bold">
                                                    {if count > 99 { "99+".to_string() } else { count.to_string() }}
                                                </span>
                                            }.into_any();
                                        }
                                    }
                                    view! { <span></span> }.into_any()
                                }}
                            </button>
                        }
                    }).collect_view()}
                </div>

                // Spacer
                <div class="flex-1"></div>

                // Connection status indicator
                <div class="flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20">
                    <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span>
                    <span class="text-xs font-medium text-emerald-400">"Live"</span>
                </div>
            </div>

            // Module content area - children should be ModulePanel components
            <div class="flex-1 min-h-0 overflow-hidden relative">
                {children()}
            </div>
        </div>
    }
}

/// Use this inside TabbedModules to wrap each module
#[component]
pub fn ModulePanel(
    /// The module ID this panel is for (must match a tab id)
    #[prop(into)]
    id: &'static str,
    /// The module content
    children: Children,
) -> impl IntoView {
    let active_tab =
        use_context::<ActiveTabSignal>().expect("ModulePanel must be used inside TabbedModules");

    view! {
        <div
            class=move || format!(
                "absolute inset-0 h-full w-full transition-all duration-200 {}",
                if active_tab.get() == id {
                    "opacity-100 translate-x-0 z-10"
                } else {
                    "opacity-0 translate-x-4 z-0 pointer-events-none"
                }
            )
        >
            {children()}
        </div>
    }
}
