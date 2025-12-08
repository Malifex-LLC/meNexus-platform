// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use super::types::{
    Credential, ExternalLink, FavoriteSynapse, FavoriteUser, ProfileData, ShowcasedPost,
};
use leptos::prelude::*;
use leptos_router::components::A;

/// Format large numbers with K/M suffixes
fn format_count(count: u32) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.1}K", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

/// Showcase tab content - user's highlighted content
#[component]
pub fn ProfileShowcase(#[prop(into)] profile: ProfileData) -> impl IntoView {
    let posts = ShowcasedPost::mock_posts();
    let synapses = FavoriteSynapse::mock_synapses();
    let users = FavoriteUser::mock_users();
    let links = ExternalLink::mock_links();
    let credentials = Credential::mock_credentials();

    view! {
        <div class="p-4 sm:p-6 space-y-6">
            // Showcased Posts Section
            <ShowcaseSection 
                title="Pinned Posts" 
                icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M7.5 8.25h9m-9 3H12m-9.75 1.51c0 1.6 1.123 2.994 2.707 3.227 1.129.166 2.27.293 3.423.379.35.026.67.21.865.501L12 21l2.755-4.133a1.14 1.14 0 01.865-.501 48.172 48.172 0 003.423-.379c1.584-.233 2.707-1.626 2.707-3.228V6.741c0-1.602-1.123-2.995-2.707-3.228A48.394 48.394 0 0012 3c-2.392 0-4.744.175-7.043.513C3.373 3.746 2.25 5.14 2.25 6.741v6.018z"/></svg>"#
            >
                <div class="space-y-4">
                    {posts.into_iter().map(|post| {
                        view! { <ShowcasedPostCard post=post /> }
                    }).collect_view()}
                </div>
            </ShowcaseSection>

            // Two column grid for synapses and users
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                // Favorite Synapses
                <ShowcaseSection 
                    title="Favorite Synapses" 
                    icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13 10V3L4 14h7v7l9-11h-7z"/></svg>"#
                >
                    <div class="space-y-2">
                        {synapses.into_iter().map(|synapse| {
                            view! { <FavoriteSynapseCard synapse=synapse /> }
                        }).collect_view()}
                    </div>
                </ShowcaseSection>

                // Favorite Users
                <ShowcaseSection 
                    title="Favorite People" 
                    icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M15 19.128a9.38 9.38 0 002.625.372 9.337 9.337 0 004.121-.952 4.125 4.125 0 00-7.533-2.493M15 19.128v-.003c0-1.113-.285-2.16-.786-3.07M15 19.128v.106A12.318 12.318 0 018.624 21c-2.331 0-4.512-.645-6.374-1.766l-.001-.109a6.375 6.375 0 0111.964-3.07M12 6.375a3.375 3.375 0 11-6.75 0 3.375 3.375 0 016.75 0zm8.25 2.25a2.625 2.625 0 11-5.25 0 2.625 2.625 0 015.25 0z"/></svg>"#
                >
                    <div class="space-y-2">
                        {users.into_iter().map(|user| {
                            view! { <FavoriteUserCard user=user /> }
                        }).collect_view()}
                    </div>
                </ShowcaseSection>
            </div>

            // External Links
            <ShowcaseSection 
                title="Links" 
                icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"/></svg>"#
            >
                <div class="flex flex-wrap gap-3">
                    {links.into_iter().map(|link| {
                        view! { <ExternalLinkButton link=link /> }
                    }).collect_view()}
                </div>
            </ShowcaseSection>

            // Favorite Badges/Credentials
            <ShowcaseSection 
                title="Featured Badges" 
                icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09zM18.259 8.715L18 9.75l-.259-1.035a3.375 3.375 0 00-2.455-2.456L14.25 6l1.036-.259a3.375 3.375 0 002.455-2.456L18 2.25l.259 1.035a3.375 3.375 0 002.456 2.456L21.75 6l-1.035.259a3.375 3.375 0 00-2.456 2.456zM16.894 20.567L16.5 21.75l-.394-1.183a2.25 2.25 0 00-1.423-1.423L13.5 18.75l1.183-.394a2.25 2.25 0 001.423-1.423l.394-1.183.394 1.183a2.25 2.25 0 001.423 1.423l1.183.394-1.183.394a2.25 2.25 0 00-1.423 1.423z"/></svg>"#
            >
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                    {credentials.into_iter().take(4).map(|cred| {
                        view! { <FeaturedBadgeCard credential=cred /> }
                    }).collect_view()}
                </div>
            </ShowcaseSection>

            // Photo Gallery placeholder
            <ShowcaseSection 
                title="Photos" 
                icon=r#"<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2"><path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"/></svg>"#
            >
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
                    // Placeholder photos
                    {(0..8).map(|i| {
                        view! {
                            <div class="aspect-square bg-gradient-to-br from-foreground/10 to-foreground/5 rounded-xl border border-border/30 flex items-center justify-center group hover:border-brand/30 transition-colors cursor-pointer">
                                <svg class="w-8 h-8 text-foreground/20 group-hover:text-brand/40 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1.5">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M2.25 15.75l5.159-5.159a2.25 2.25 0 013.182 0l5.159 5.159m-1.5-1.5l1.409-1.409a2.25 2.25 0 013.182 0l2.909 2.909m-18 3.75h16.5a1.5 1.5 0 001.5-1.5V6a1.5 1.5 0 00-1.5-1.5H3.75A1.5 1.5 0 002.25 6v12a1.5 1.5 0 001.5 1.5zm10.5-11.25h.008v.008h-.008V8.25zm.375 0a.375.375 0 11-.75 0 .375.375 0 01.75 0z"/>
                                </svg>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </ShowcaseSection>
        </div>
    }
}

/// Reusable showcase section wrapper
#[component]
fn ShowcaseSection(
    #[prop(into)] title: String,
    #[prop(into)] icon: String,
    children: Children,
) -> impl IntoView {
    view! {
        <section class="bg-card border border-border/50 rounded-2xl overflow-hidden">
            <header class="px-4 py-3 border-b border-border/30 flex items-center justify-between">
                <div class="flex items-center gap-2">
                    <span class="w-4 h-4 text-brand" inner_html=icon></span>
                    <h3 class="font-semibold text-foreground">{title}</h3>
                </div>
                <button class="text-xs text-foreground/40 hover:text-brand transition-colors">
                    "Edit"
                </button>
            </header>
            <div class="p-4">
                {children()}
            </div>
        </section>
    }
}

/// Showcased post card
#[component]
fn ShowcasedPostCard(#[prop(into)] post: ShowcasedPost) -> impl IntoView {
    view! {
        <article class="p-4 bg-background/50 border border-border/30 rounded-xl hover:border-border/50 transition-colors">
            <div class="flex items-start gap-3">
                // Pin indicator
                {if post.is_pinned {
                    view! {
                        <div class="flex-shrink-0 w-8 h-8 rounded-lg bg-amber-500/15 flex items-center justify-center">
                            <svg class="w-4 h-4 text-amber-400" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M16 12V4h1V2H7v2h1v8l-2 2v2h5.2v6h1.6v-6H18v-2l-2-2z"/>
                            </svg>
                        </div>
                    }.into_any()
                } else {
                    view! { <span></span> }.into_any()
                }}
                
                <div class="flex-1 min-w-0">
                    <p class="text-foreground/90 leading-relaxed mb-2">{post.content}</p>
                    
                    <div class="flex items-center gap-4 text-xs text-foreground/40">
                        <span>{post.timestamp}</span>
                        <span class="flex items-center gap-1">
                            <svg class="w-3.5 h-3.5" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                            </svg>
                            {format_count(post.likes)}
                        </span>
                        <span class="flex items-center gap-1">
                            <svg class="w-3.5 h-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                                <path stroke-linecap="round" stroke-linejoin="round" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"/>
                            </svg>
                            {post.comments}
                        </span>
                    </div>
                </div>
            </div>
        </article>
    }
}

/// Favorite synapse card
#[component]
fn FavoriteSynapseCard(#[prop(into)] synapse: FavoriteSynapse) -> impl IntoView {
    let initials: String = synapse
        .name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    view! {
        <A 
            href=format!("/synapses/{}", synapse.id)
            attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-foreground/5 transition-colors group"
        >
            // Icon
            {if let Some(url) = synapse.icon_url {
                view! {
                    <img src=url alt="" class="w-10 h-10 rounded-xl object-cover"/>
                }.into_any()
            } else {
                view! {
                    <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center">
                        <span class="text-brand font-bold text-sm">{initials}</span>
                    </div>
                }.into_any()
            }}
            
            <div class="flex-1 min-w-0">
                <p class="font-medium text-foreground group-hover:text-brand transition-colors truncate">
                    {synapse.name}
                </p>
                <p class="text-xs text-foreground/40 truncate">{synapse.description}</p>
            </div>
            
            // Member stats
            <div class="text-right">
                <p class="text-sm font-medium text-foreground">{format_count(synapse.member_count)}</p>
                <p class="text-xs text-emerald-400 flex items-center gap-1 justify-end">
                    <span class="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
                    {format_count(synapse.online_count)}" online"
                </p>
            </div>
        </A>
    }
}

/// Favorite user card
#[component]
fn FavoriteUserCard(#[prop(into)] user: FavoriteUser) -> impl IntoView {
    let initials: String = user
        .display_name
        .split_whitespace()
        .take(2)
        .filter_map(|s| s.chars().next())
        .collect::<String>()
        .to_uppercase();

    view! {
        <A 
            href=format!("/profiles/{}", user.handle)
            attr:class="flex items-center gap-3 p-3 rounded-xl hover:bg-foreground/5 transition-colors group"
        >
            // Avatar
            <div class="relative">
                {if let Some(url) = user.avatar_url {
                    view! {
                        <img src=url alt="" class="w-10 h-10 rounded-full object-cover"/>
                    }.into_any()
                } else {
                    view! {
                        <div class="w-10 h-10 rounded-full bg-gradient-to-br from-brand/30 to-brand/10 flex items-center justify-center">
                            <span class="text-brand font-bold text-sm">{initials}</span>
                        </div>
                    }.into_any()
                }}
            </div>
            
            <div class="flex-1 min-w-0">
                <div class="flex items-center gap-1.5">
                    <p class="font-medium text-foreground group-hover:text-brand transition-colors truncate">
                        {user.display_name}
                    </p>
                    {if user.is_verified {
                        view! {
                            <svg class="w-4 h-4 text-brand flex-shrink-0" viewBox="0 0 24 24" fill="currentColor">
                                <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                            </svg>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
                    }}
                </div>
                <p class="text-xs text-brand/60 font-mono">{"@"}{user.handle}</p>
            </div>
            
            // Mutual indicator
            {if user.mutual_follows {
                view! {
                    <span class="px-2 py-0.5 rounded-full bg-brand/15 text-brand text-xs font-medium">
                        "Mutual"
                    </span>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </A>
    }
}

/// External link button
#[component]
fn ExternalLinkButton(#[prop(into)] link: ExternalLink) -> impl IntoView {
    view! {
        <a 
            href=link.url
            target="_blank"
            rel="noopener noreferrer"
            class="flex items-center gap-2 px-4 py-2 rounded-xl bg-foreground/5 border border-border/30 hover:bg-foreground/10 hover:border-brand/30 transition-all group"
        >
            <span class="w-5 h-5 text-foreground/60 group-hover:text-brand transition-colors" inner_html=link.icon.svg()></span>
            <span class="font-medium text-foreground/80 group-hover:text-foreground transition-colors">{link.title}</span>
            <svg class="w-4 h-4 text-foreground/30 group-hover:text-brand/50 transition-colors" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 19.5l15-15m0 0H8.25m11.25 0v11.25"/>
            </svg>
        </a>
    }
}

/// Featured badge card (larger display for showcase)
#[component]
fn FeaturedBadgeCard(#[prop(into)] credential: Credential) -> impl IntoView {
    let rarity_gradient = credential.rarity.color_class();
    let border_class = credential.rarity.border_class();

    view! {
        <div class=format!(
            "p-4 rounded-xl border bg-background/50 text-center hover:scale-105 transition-transform cursor-pointer {}",
            border_class
        )>
            // Badge icon (larger)
            <div class=format!(
                "w-16 h-16 mx-auto mb-3 rounded-2xl bg-gradient-to-br flex items-center justify-center shadow-lg {}",
                rarity_gradient
            )>
                {if credential.is_verified {
                    view! {
                        <svg class="w-8 h-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5"/>
                        </svg>
                    }.into_any()
                } else {
                    view! {
                        <svg class="w-8 h-8 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M9.813 15.904L9 18.75l-.813-2.846a4.5 4.5 0 00-3.09-3.09L2.25 12l2.846-.813a4.5 4.5 0 003.09-3.09L9 5.25l.813 2.846a4.5 4.5 0 003.09 3.09L15.75 12l-2.846.813a4.5 4.5 0 00-3.09 3.09z"/>
                        </svg>
                    }.into_any()
                }}
            </div>
            
            <h4 class="font-semibold text-foreground text-sm mb-1">{credential.name}</h4>
            <p class="text-xs text-foreground/40">{credential.issuer_name}</p>
            
            // Verified indicator
            {if credential.is_verified {
                view! {
                    <div class="mt-2 flex items-center justify-center gap-1 text-emerald-400 text-xs">
                        <svg class="w-3.5 h-3.5" viewBox="0 0 24 24" fill="currentColor">
                            <path fill-rule="evenodd" d="M8.603 3.799A4.49 4.49 0 0112 2.25c1.357 0 2.573.6 3.397 1.549a4.49 4.49 0 013.498 1.307 4.491 4.491 0 011.307 3.497A4.49 4.49 0 0121.75 12a4.49 4.49 0 01-1.549 3.397 4.491 4.491 0 01-1.307 3.497 4.491 4.491 0 01-3.497 1.307A4.49 4.49 0 0112 21.75a4.49 4.49 0 01-3.397-1.549 4.49 4.49 0 01-3.498-1.306 4.491 4.491 0 01-1.307-3.498A4.49 4.49 0 012.25 12c0-1.357.6-2.573 1.549-3.397a4.49 4.49 0 011.307-3.497 4.49 4.49 0 013.497-1.307zm7.007 6.387a.75.75 0 10-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 00-1.06 1.06l2.25 2.25a.75.75 0 001.14-.094l3.75-5.25z" clip-rule="evenodd"/>
                        </svg>
                        "Verified"
                    </div>
                }.into_any()
            } else {
                view! { <span></span> }.into_any()
            }}
        </div>
    }
}
