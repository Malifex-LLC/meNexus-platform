// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod identity_panel;
pub mod nav_bar;

use crate::components::control_panel::identity_panel::IdentityPanel;
use crate::components::control_panel::nav_bar::NavBar;
use leptos::prelude::*;

#[component]
pub fn ControlPanel() -> impl IntoView {
    view! {
        <div class="flex-col bg-panel border border-border w-full h-full p-2">
            <div class=""><IdentityPanel/></div>
            <div class="my-2"><NavBar/></div>
        </div>
    }
}
