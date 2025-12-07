// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::dashboard_layout::DashboardLayout;
use leptos::prelude::*;

#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <DashboardLayout/>
    }
}
