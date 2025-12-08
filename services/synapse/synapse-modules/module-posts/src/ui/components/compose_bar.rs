// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::server_fns::create_post_server;
use crate::types::CreatePostRequest;
use leptos::prelude::*;
use leptos::task::spawn_local;
use std::rc::Rc;

#[cfg(feature = "hydrate")]
#[component]
pub fn ComposeBar(
    /// The channel name to post to
    #[prop(into)]
    channel: Signal<String>,
    /// Optional user avatar URL
    #[prop(into, optional)]
    user_avatar: Option<String>,
    /// Optional user initials for avatar fallback
    #[prop(into, optional)]
    user_initials: Option<String>,
    #[prop(into, optional)] on_post_created: Option<Callback<()>>,
) -> impl IntoView {
    use synapse_core::domain::profiles::Profile;

    let session_user_profile =
        use_context::<Profile>().expect("Session_user_profile context not found");

    let (content, set_content) = signal(String::new());
    let (is_posting, set_is_posting) = signal(false);
    let textarea_ref = NodeRef::<leptos::html::Textarea>::new();

    let initials = user_initials.unwrap_or_else(|| "?".to_string());

    let resize_textarea = move || {
        if let Some(textarea) = textarea_ref.get() {
            let _ = textarea.style(("height", "auto"));
            let scroll_height = textarea.scroll_height();
            let max_height = 120; // Smaller max for compact mode
            let new_height = scroll_height.min(max_height);
            let _ = textarea.style(("height", format!("{}px", new_height)));
        }
    };

    let on_submit: Rc<dyn Fn()> = {
        let channel = channel.clone();
        let textarea_ref = textarea_ref.clone();
        let session_user_profile = session_user_profile.clone();

        Rc::new(move || {
            let post_content = content.get();
            if post_content.trim().is_empty() {
                return;
            }

            set_is_posting.set(true);

            let agent = session_user_profile.public_key.clone();
            let channel_slug = channel.get();
            let textarea_ref = textarea_ref.clone();

            // Spawn async server call
            spawn_local(async move {
                let _ = create_post_server(CreatePostRequest {
                    event_type: "posts:create_post".to_string(),
                    agent,
                    module_kind: Some("posts".to_string()),
                    module_slug: Some(channel_slug),
                    target: None,
                    previous: None,
                    content: Some(post_content),
                    artifacts: None,
                    metadata: None,
                    links: None,
                    data: None,
                    expiration: None,
                })
                .await;

                if let Some(callback) = &on_post_created {
                    callback.run(());
                }

                set_content.set(String::new());
                set_is_posting.set(false);

                if let Some(textarea) = textarea_ref.get() {
                    let _ = textarea.style(("height", "auto"));
                }
            });
        })
    };

    // Clone Rc into each handler
    let on_submit_keydown = on_submit.clone();
    let on_submit_click = on_submit.clone();

    view! {
        <div class="flex-shrink-0 border-t border-border/50 bg-panel/30">
            <div class="p-2">
                <div class="flex items-end gap-2">
                    // Avatar (hidden on very small screens)
                    <div class="hidden sm:block flex-shrink-0 mb-0.5">
                        {if let Some(avatar_url) = user_avatar.clone() {
                            view! {
                                <img
                                    src=avatar_url
                                    alt="Your avatar"
                                    class="w-7 h-7 rounded-full object-cover ring-1 ring-border/30"
                                />
                            }.into_any()
                        } else {
                            view! {
                                <div class="w-7 h-7 rounded-full bg-gradient-to-br from-brand/20 to-brand/5 flex items-center justify-center ring-1 ring-border/30">
                                    <span class="text-brand font-bold text-[10px]">{initials.clone()}</span>
                                </div>
                            }.into_any()
                        }}
                    </div>

                    // Input container
                    <div class="flex-1 relative min-w-0">
                        <div class="relative bg-background border border-border/50 rounded-lg focus-within:border-brand/50 transition-all">
                            <textarea
                                node_ref=textarea_ref
                                placeholder=move || format!("Post to #{}", channel.get())
                                class="w-full px-3 py-2 pr-20 bg-transparent text-sm text-foreground placeholder-foreground/30 focus:outline-none resize-none overflow-y-auto leading-relaxed scrollbar-thin"
                                style="min-height: 36px; max-height: 120px;"
                                prop:value=move || content.get()
                                prop:disabled=move || is_posting.get()
                                on:input=move |ev| {
                                    set_content.set(event_target_value(&ev));
                                    resize_textarea();
                                }
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" && (ev.meta_key() || ev.ctrl_key()) {
                                        ev.prevent_default();
                                        on_submit_keydown();
                                    }
                                }
                            ></textarea>

                            // Action buttons inside
                            <div class="absolute bottom-1.5 right-1.5 flex items-center gap-0.5">
                                <button
                                    class="p-1.5 rounded text-foreground/30 hover:text-foreground/60 hover:bg-foreground/5 transition-colors"
                                    title="Attach file"
                                >
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.172 7l-6.586 6.586a2 2 0 102.828 2.828l6.414-6.586a4 4 0 00-5.656-5.656l-6.415 6.585a6 6 0 108.486 8.486L20.5 13"></path>
                                    </svg>
                                </button>
                                <button
                                    class="p-1.5 rounded text-foreground/30 hover:text-foreground/60 hover:bg-foreground/5 transition-colors"
                                    title="Add emoji"
                                >
                                    <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                    </svg>
                                </button>
                            </div>
                        </div>
                    </div>

                    // Post button
                    <button
                        class=move || format!(
                            "flex-shrink-0 mb-0.5 px-3 py-2 rounded-lg font-medium text-sm transition-all active:scale-95 {}",
                            if content.get().trim().is_empty() || is_posting.get() {
                                "bg-brand/30 text-white/50 cursor-not-allowed"
                            } else {
                                "bg-brand hover:bg-brand/90 text-white"
                            }
                        )
                        prop:disabled=move || content.get().trim().is_empty() || is_posting.get()
                        on:click=move |_| on_submit_click()
                    >
                        {move || if is_posting.get() {
                            view! {
                                <svg class="w-4 h-4 animate-spin" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4z"></path>
                                </svg>
                            }.into_any()
                        } else {
                            view! { "Post" }.into_any()
                        }}
                    </button>
                </div>

                // Hints (hidden on small screens)
                <div class="hidden sm:flex items-center justify-end mt-1 px-9 text-[10px] text-foreground/25">
                    <span>
                        <kbd class="px-1 py-0.5 bg-foreground/5 rounded font-mono">"⌘"</kbd>
                        "+"
                        <kbd class="px-1 py-0.5 bg-foreground/5 rounded font-mono">"Enter"</kbd>
                        " to post"
                    </span>
                </div>
            </div>
        </div>
    }
}
