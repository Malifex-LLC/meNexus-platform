// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod discovery_sections;
pub mod result_cards;
pub mod search_bar;
pub mod search_filters;
pub mod search_results;
pub mod types;

pub use discovery_sections::{
    BrowseByCategory, FeaturedSynapses, SuggestedUsers, TrendingPosts, TrendingTags,
};
pub use result_cards::{PostResultCard, SynapseResultCard, TagResultCard, UserResultCard};
pub use search_bar::{CompactSearchBar, SearchBar};
pub use search_filters::SearchFilters;
pub use search_results::{EmptySearchState, NoResultsState, SearchResults};
pub use types::{
    DiscoveryPost, DiscoverySynapse, DiscoveryUser, PostResult, SearchCategory, SearchScope,
    SearchSortBy, SearchTimeRange, SynapseResult, TagResult, UserResult,
};
