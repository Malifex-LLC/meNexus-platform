// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

/// Notification settings panel
#[component]
pub fn NotificationSettings() -> impl IntoView {
    // Push notifications
    let (push_enabled, set_push_enabled) = signal(true);
    let (push_mentions, set_push_mentions) = signal(true);
    let (push_dms, set_push_dms) = signal(true);
    let (push_followers, set_push_followers) = signal(true);
    let (push_likes, set_push_likes) = signal(false);
    let (push_reposts, set_push_reposts) = signal(false);
    let (push_synapse_activity, set_push_synapse_activity) = signal(true);

    // Email notifications
    let (email_enabled, set_email_enabled) = signal(true);
    let (email_digest, set_email_digest) = signal("weekly".to_string());
    let (email_mentions, set_email_mentions) = signal(true);
    let (email_dms, set_email_dms) = signal(false);
    let (email_announcements, set_email_announcements) = signal(true);

    // Sound & vibration
    let (sound_enabled, set_sound_enabled) = signal(true);
    let (vibration_enabled, set_vibration_enabled) = signal(true);

    view! {
        <div class="space-y-6">
            // Header
            <div>
                <h2 class="text-xl font-semibold text-foreground">"Notification Settings"</h2>
                <p class="text-sm text-foreground/50">"Configure how and when you receive notifications"</p>
            </div>

            // Push Notifications
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="flex items-center justify-between p-6 border-b border-border/30">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl bg-brand/15 flex items-center justify-center">
                            <svg class="w-5 h-5 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
                            </svg>
                        </div>
                        <div>
                            <h3 class="font-semibold text-foreground">"Push Notifications"</h3>
                            <p class="text-sm text-foreground/50">"Receive notifications on your device"</p>
                        </div>
                    </div>
                    <button
                        class=move || format!(
                            "relative w-11 h-6 rounded-full transition-colors {}",
                            if push_enabled.get() { "bg-brand" } else { "bg-foreground/20" }
                        )
                        on:click=move |_| set_push_enabled.update(|v| *v = !*v)
                    >
                        <span class=move || format!(
                            "absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform {}",
                            if push_enabled.get() { "translate-x-5" } else { "translate-x-0" }
                        )></span>
                    </button>
                </div>

                {move || {
                    if push_enabled.get() {
                        view! {
                            <div class="p-6 space-y-4">
                                <NotificationToggle
                                    icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/>"#
                                    label="Mentions"
                                    description="When someone @mentions you"
                                    checked=push_mentions
                                    on_change=move |v| set_push_mentions.set(v)
                                />
                                <NotificationToggle
                                    icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>"#
                                    label="Direct Messages"
                                    description="New messages from other users"
                                    checked=push_dms
                                    on_change=move |v| set_push_dms.set(v)
                                />
                                <NotificationToggle
                                    icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"/>"#
                                    label="New Followers"
                                    description="When someone follows you"
                                    checked=push_followers
                                    on_change=move |v| set_push_followers.set(v)
                                />
                                <NotificationToggle
                                    icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>"#
                                    label="Likes"
                                    description="When someone likes your post"
                                    checked=push_likes
                                    on_change=move |v| set_push_likes.set(v)
                                />
                                <NotificationToggle
                                    icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z"/>"#
                                    label="Reposts"
                                    description="When someone shares your post"
                                    checked=push_reposts
                                    on_change=move |v| set_push_reposts.set(v)
                                />
                                <NotificationToggle
                                    icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/>"#
                                    label="Synapse Activity"
                                    description="Important updates from your synapses"
                                    checked=push_synapse_activity
                                    on_change=move |v| set_push_synapse_activity.set(v)
                                />
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="p-6 text-center text-foreground/40 text-sm">
                                "Push notifications are disabled"
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Email Notifications
            <div class="bg-card border border-border/50 rounded-2xl overflow-hidden">
                <div class="flex items-center justify-between p-6 border-b border-border/30">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 rounded-xl bg-violet-500/15 flex items-center justify-center">
                            <svg class="w-5 h-5 text-violet-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                            </svg>
                        </div>
                        <div>
                            <h3 class="font-semibold text-foreground">"Email Notifications"</h3>
                            <p class="text-sm text-foreground/50">"Receive updates via email"</p>
                        </div>
                    </div>
                    <button
                        class=move || format!(
                            "relative w-11 h-6 rounded-full transition-colors {}",
                            if email_enabled.get() { "bg-brand" } else { "bg-foreground/20" }
                        )
                        on:click=move |_| set_email_enabled.update(|v| *v = !*v)
                    >
                        <span class=move || format!(
                            "absolute top-0.5 left-0.5 w-5 h-5 bg-white rounded-full shadow transition-transform {}",
                            if email_enabled.get() { "translate-x-5" } else { "translate-x-0" }
                        )></span>
                    </button>
                </div>

                {move || {
                    if email_enabled.get() {
                        view! {
                            <div class="p-6 space-y-5">
                                // Digest frequency
                                <div class="space-y-2">
                                    <label class="text-sm font-medium text-foreground">"Email Digest Frequency"</label>
                                    <div class="flex gap-2">
                                        {["daily", "weekly", "never"].into_iter().map(|freq| {
                                            let label = match freq {
                                                "daily" => "Daily",
                                                "weekly" => "Weekly",
                                                _ => "Never",
                                            };
                                            view! {
                                                <button
                                                    class=move || format!(
                                                        "flex-1 px-4 py-2 rounded-xl text-sm font-medium transition-all {}",
                                                        if email_digest.get() == freq {
                                                            "bg-brand text-white"
                                                        } else {
                                                            "bg-foreground/5 text-foreground/60 hover:bg-foreground/10"
                                                        }
                                                    )
                                                    on:click=move |_| set_email_digest.set(freq.to_string())
                                                >
                                                    {label}
                                                </button>
                                            }
                                        }).collect_view()}
                                    </div>
                                </div>

                                <div class="space-y-3 pt-2">
                                    <NotificationToggle
                                        icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/>"#
                                        label="Mentions"
                                        description="Email when someone mentions you"
                                        checked=email_mentions
                                        on_change=move |v| set_email_mentions.set(v)
                                    />
                                    <NotificationToggle
                                        icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>"#
                                        label="Direct Messages"
                                        description="Email for unread messages"
                                        checked=email_dms
                                        on_change=move |v| set_email_dms.set(v)
                                    />
                                    <NotificationToggle
                                        icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M11 5.882V19.24a1.76 1.76 0 01-3.417.592l-2.147-6.15M18 13a3 3 0 100-6M5.436 13.683A4.001 4.001 0 017 6h1.832c4.1 0 7.625-1.234 9.168-3v14c-1.543-1.766-5.067-3-9.168-3H7a3.988 3.988 0 01-1.564-.317z"/>"#
                                        label="Product Updates"
                                        description="News and feature announcements"
                                        checked=email_announcements
                                        on_change=move |v| set_email_announcements.set(v)
                                    />
                                </div>
                            </div>
                        }.into_any()
                    } else {
                        view! {
                            <div class="p-6 text-center text-foreground/40 text-sm">
                                "Email notifications are disabled"
                            </div>
                        }.into_any()
                    }
                }}
            </div>

            // Sound & Vibration
            <div class="bg-card border border-border/50 rounded-2xl p-6 space-y-4">
                <h3 class="text-sm font-semibold text-foreground flex items-center gap-2">
                    <svg class="w-4 h-4 text-brand" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"/>
                    </svg>
                    "Sound & Haptics"
                </h3>

                <div class="space-y-3">
                    <NotificationToggle
                        icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z"/>"#
                        label="Sound"
                        description="Play a sound for notifications"
                        checked=sound_enabled
                        on_change=move |v| set_sound_enabled.set(v)
                    />
                    <NotificationToggle
                        icon=r#"<path stroke-linecap="round" stroke-linejoin="round" d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"/>"#
                        label="Vibration"
                        description="Vibrate for notifications (mobile)"
                        checked=vibration_enabled
                        on_change=move |v| set_vibration_enabled.set(v)
                    />
                </div>
            </div>

            // Quiet Hours
            <div class="bg-card border border-border/50 rounded-2xl p-6">
                <div class="flex items-start gap-4">
                    <div class="w-10 h-10 rounded-xl bg-amber-500/15 flex items-center justify-center flex-shrink-0">
                        <svg class="w-5 h-5 text-amber-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <div class="flex items-center justify-between">
                            <h3 class="font-semibold text-foreground">"Quiet Hours"</h3>
                            <span class="text-xs text-foreground/40 bg-foreground/5 px-2 py-0.5 rounded-lg">"Coming soon"</span>
                        </div>
                        <p class="text-sm text-foreground/50 mt-1">"Mute notifications during specific hours. Perfect for maintaining focus or getting uninterrupted sleep."</p>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Notification toggle with icon
#[component]
fn NotificationToggle(
    icon: &'static str,
    label: &'static str,
    description: &'static str,
    #[prop(into)] checked: ReadSignal<bool>,
    on_change: impl Fn(bool) + 'static + Copy,
) -> impl IntoView {
    view! {
        <div class="flex items-center justify-between p-3 bg-foreground/5 rounded-xl">
            <div class="flex items-center gap-3">
                <svg class="w-5 h-5 text-foreground/40" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2" inner_html=icon></svg>
                <div>
                    <p class="text-sm font-medium text-foreground">{label}</p>
                    <p class="text-xs text-foreground/40">{description}</p>
                </div>
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
