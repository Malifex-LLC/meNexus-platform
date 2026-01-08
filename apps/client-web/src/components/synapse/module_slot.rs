// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Module Slot
//!
//! Renders the appropriate module component based on the module ID.
//! This is the bridge between the manifest's module configuration and the actual UI.

use leptos::prelude::*;
use module_core::ClientManifest;

use crate::app::SessionUserProfile;

// Module UI imports
use module_activity::ui::components::activity_feed::ActivityFeed;
use module_chat::ui::components::chat_feed::ChatFeed;
use module_livestream::ui::components::LivestreamFeed;
use module_members::ui::components::members_list::MembersList;
use module_posts::ui::components::post_feed::PostsFeed;

/// Renders a module based on its ID
#[component]
pub fn ModuleSlot(module_id: String, manifest: ClientManifest) -> impl IntoView {
    // Check if module is installed
    let is_installed = manifest.installed_modules.contains(&module_id);

    if !is_installed {
        return view! {
            <ModuleDisabledState module_id=module_id/>
        }.into_any();
    }

    // Render the appropriate module
    match module_id.as_str() {
        "posts" => view! { <PostsModule/> }.into_any(),
        "chat" => view! { <ChatModule/> }.into_any(),
        "activity" => view! { <ActivityModule/> }.into_any(),
        "members" => view! { <MembersModule/> }.into_any(),
        "livestream" => view! { <LivestreamModule/> }.into_any(),
        "creator" => view! { <CreatorModule/> }.into_any(),
        "files" => view! { <PlaceholderModule name="Files" icon="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"/> }.into_any(),
        "media" => view! { <PlaceholderModule name="Media" icon="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"/> }.into_any(),
        "calendar" => view! { <PlaceholderModule name="Calendar" icon="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/> }.into_any(),
        _ => view! { <UnknownModule module_id=module_id/> }.into_any(),
    }
}

// =============================================================================
// MODULE WRAPPERS
// =============================================================================

/// Posts module wrapper
#[component]
fn PostsModule() -> impl IntoView {
    let session_user =
        use_context::<SessionUserProfile>().expect("SessionUserProfile context not found");

    view! {
        <Suspense fallback=move || view! { <ModuleLoadingState/> }>
            {move || {
                session_user.get().map(|result| {
                    match result {
                        Ok(Some(profile)) => {
                            view! { <PostsFeed session_user_profile=profile/> }.into_any()
                        }
                        Ok(None) => {
                            view! { <ModuleAuthRequired module_name="Posts"/> }.into_any()
                        }
                        Err(e) => {
                            view! { <ModuleErrorState error=e.to_string()/> }.into_any()
                        }
                    }
                })
            }}
        </Suspense>
    }
}

/// Chat module wrapper
#[component]
fn ChatModule() -> impl IntoView {
    view! { <ChatFeed/> }
}

/// Activity module wrapper
#[component]
fn ActivityModule() -> impl IntoView {
    view! { <ActivityFeed/> }
}

/// Members module wrapper
#[component]
fn MembersModule() -> impl IntoView {
    view! { <MembersList/> }
}

/// Livestream module wrapper
#[component]
fn LivestreamModule() -> impl IntoView {
    view! { <LivestreamFeed/> }
}

/// Creator module wrapper (placeholder - module removed)
#[component]
fn CreatorModule() -> impl IntoView {
    view! { <PlaceholderModule name="Creator" icon="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z"/> }
}

// =============================================================================
// PLACEHOLDER & STATE COMPONENTS
// =============================================================================

/// Placeholder for modules not yet implemented
#[component]
fn PlaceholderModule(name: &'static str, icon: &'static str) -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col items-center justify-center gap-4 bg-panel/30 p-8">
            <div class="w-16 h-16 rounded-2xl bg-foreground/5 flex items-center justify-center">
                <svg class="w-8 h-8 text-foreground/30" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d=icon/>
                </svg>
            </div>
            <div class="text-center">
                <h3 class="text-lg font-semibold text-foreground/70">{name}</h3>
                <p class="text-sm text-foreground/40">"Coming soon"</p>
            </div>
        </div>
    }
}

/// Unknown module state
#[component]
fn UnknownModule(module_id: String) -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col items-center justify-center gap-4 bg-panel/30 p-8">
            <div class="w-16 h-16 rounded-2xl bg-warning/10 flex items-center justify-center">
                <svg class="w-8 h-8 text-warning" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M9.879 7.519c1.171-1.025 3.071-1.025 4.242 0 1.172 1.025 1.172 2.687 0 3.712-.203.179-.43.326-.67.442-.745.361-1.45.999-1.45 1.827v.75M21 12a9 9 0 11-18 0 9 9 0 0118 0zm-9 5.25h.008v.008H12v-.008z"/>
                </svg>
            </div>
            <div class="text-center">
                <h3 class="text-lg font-semibold text-foreground/70">"Unknown Module"</h3>
                <p class="text-sm text-foreground/40">{format!("Module '{}' is not recognized", module_id)}</p>
            </div>
        </div>
    }
}

/// Module disabled state
#[component]
fn ModuleDisabledState(module_id: String) -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col items-center justify-center gap-4 bg-panel/30 p-8">
            <div class="w-16 h-16 rounded-2xl bg-foreground/5 flex items-center justify-center">
                <svg class="w-8 h-8 text-foreground/20" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636"/>
                </svg>
            </div>
            <div class="text-center">
                <h3 class="text-lg font-semibold text-foreground/50">"Module Disabled"</h3>
                <p class="text-sm text-foreground/30">{format!("'{}' is not enabled for this Synapse", module_id)}</p>
            </div>
        </div>
    }
}

/// Module loading state
#[component]
fn ModuleLoadingState() -> impl IntoView {
    view! {
        <div class="h-full w-full flex items-center justify-center bg-panel/30">
            <div class="w-8 h-8 border-3 border-foreground/20 border-t-brand rounded-full animate-spin"></div>
        </div>
    }
}

/// Module error state
#[component]
fn ModuleErrorState(error: String) -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col items-center justify-center gap-4 bg-panel/30 p-8">
            <div class="w-12 h-12 rounded-full bg-error/20 flex items-center justify-center">
                <svg class="w-6 h-6 text-error" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
                </svg>
            </div>
            <div class="text-center">
                <h3 class="text-lg font-semibold text-foreground/70">"Error"</h3>
                <p class="text-sm text-foreground/40">{error}</p>
            </div>
        </div>
    }
}

/// Auth required state
#[component]
fn ModuleAuthRequired(module_name: &'static str) -> impl IntoView {
    view! {
        <div class="h-full w-full flex flex-col items-center justify-center gap-4 bg-panel/30 p-8">
            <div class="w-16 h-16 rounded-2xl bg-brand/10 flex items-center justify-center">
                <svg class="w-8 h-8 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.5 10.5V6.75a4.5 4.5 0 10-9 0v3.75m-.75 11.25h10.5a2.25 2.25 0 002.25-2.25v-6.75a2.25 2.25 0 00-2.25-2.25H6.75a2.25 2.25 0 00-2.25 2.25v6.75a2.25 2.25 0 002.25 2.25z"/>
                </svg>
            </div>
            <div class="text-center">
                <h3 class="text-lg font-semibold text-foreground/70">"Sign In Required"</h3>
                <p class="text-sm text-foreground/40">{format!("Please sign in to view {}", module_name)}</p>
            </div>
            <a
                href="/login"
                class="px-6 py-2 bg-brand text-white rounded-lg hover:bg-brand/90 transition-colors font-medium"
            >
                "Sign In"
            </a>
        </div>
    }
}
