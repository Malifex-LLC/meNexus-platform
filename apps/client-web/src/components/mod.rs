// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod control_panel;
pub mod dashboard;
pub mod messenger;
pub mod module_container;
pub mod notifications;
pub mod profile;
pub mod search;
pub mod settings;
pub mod simple_counter;
pub mod synapse;

pub use dashboard::{
    FeedFilters, GlobalCompose, GlobalFeed, GlobalPostCard, NetworkStatsWidget, TrendingSidebar,
};
pub use module_container::{ActiveTabSignal, ModulePanel, ModuleTab, TabbedModules, tabs};
pub use profile::{
    ProfileData, ProfileHeader, ProfileOverview, ProfileShowcase, ProfileTab, ProfileTabs,
    ReputationCard,
};
pub use search::{
    BrowseByCategory, CompactSearchBar, EmptySearchState, FeaturedSynapses, NoResultsState,
    PostResultCard, SearchBar, SearchCategory, SearchFilters, SearchResults, SearchScope,
    SuggestedUsers, SynapseResultCard, TagResultCard, TrendingPosts, TrendingTags, UserResultCard,
};
pub use settings::{
    AccountSettings, AppearanceSettings, DataSettings, NetworkSettings, NotificationSettings,
    PrivacySettings, ProfileSettings, SecuritySettings, SettingsSidebar, SettingsTab,
};
pub use notifications::{
    AllNotificationsView, AnalyticsPanel, BroadcastAlertCard, BroadcastsPanel, BroadcastsSection,
    ConnectionNotificationCard, ConnectionsPanel, ConnectionsSection, ContentNotificationCard,
    ContentPanel, ContentSection, NotificationCategory, NotificationsSidebar, QuickActionsPanel,
    StatusOverview, SystemNotificationCard, SystemPanel, SystemSection,
};
pub use messenger::{
    ChatHeader, Conversation, ConversationFilter, ConversationInfo, ConversationItem,
    ConversationList, ConversationType, Message, MessageBubble, MessageComposer, MessageContent,
    MessageReaction, MessageStatus, MessageThread, Participant,
};
pub use synapse::{SynapseRenderer, RemoteSynapseRenderer, SynapseHeader, DynamicLayout, ModuleSlot};