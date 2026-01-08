// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::notifications_layout::NotificationsLayout;
use leptos::prelude::*;

#[component]
pub fn NotificationsPage() -> impl IntoView {
    view! {
        <NotificationsLayout />
    }
}
