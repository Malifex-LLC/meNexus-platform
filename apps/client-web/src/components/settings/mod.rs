// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod account_settings;
pub mod appearance_settings;
pub mod data_settings;
pub mod network_settings;
pub mod notification_settings;
pub mod privacy_settings;
pub mod profile_settings;
pub mod security_settings;
pub mod settings_sidebar;
pub mod types;

pub use account_settings::AccountSettings;
pub use appearance_settings::AppearanceSettings;
pub use data_settings::DataSettings;
pub use network_settings::NetworkSettings;
pub use notification_settings::NotificationSettings;
pub use privacy_settings::PrivacySettings;
pub use profile_settings::ProfileSettings;
pub use security_settings::SecuritySettings;
pub use settings_sidebar::SettingsSidebar;
pub use types::SettingsTab;
