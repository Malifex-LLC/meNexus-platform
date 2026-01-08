// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # module-creator
//!
//! A creator/content distribution module for meNexus, similar to Patreon or OnlyFans.
//!
//! ## Features
//!
//! - **Subscription Tiers**: Multiple tier levels with different pricing and benefits
//! - **Content Distribution**: Support for images, galleries, videos, audio, text, files, and polls
//! - **Access Control**: Free, subscriber-only, and tier-locked content
//! - **Creator Profiles**: Rich profiles with stats, social links, and supporter lists
//! - **Responsive UI**: Mobile-first design with slide-in sidebars and compact views
//!
//! ## Main Components
//!
//! - [`CreatorFeed`](ui::CreatorFeed) - Main page component combining all elements
//! - [`CreatorHeader`](ui::CreatorHeader) - Profile banner, avatar, bio, and stats
//! - [`ContentCard`](ui::ContentCard) - Individual content post display
//! - [`TierCard`](ui::TierCard) - Subscription tier card
//! - [`SubscriptionSidebar`](ui::SubscriptionSidebar) - Sidebar with tiers and supporters
//! - [`ContentFilterBar`](ui::ContentFilterBar) - Search, filter, and sort controls
//! - [`ContentGrid`](ui::ContentGrid) - Grid view for content
//! - [`SupportersList`](ui::SupportersList) - Recent supporters display

pub mod types;
pub mod ui;

// Re-export main components for convenience
pub use ui::{
    ContentCard, ContentFilterBar, ContentGrid, CreatorFeed, CreatorHeader, SubscriptionSidebar,
    SupportersList, TierCard,
};
