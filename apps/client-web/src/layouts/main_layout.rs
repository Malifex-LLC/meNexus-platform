// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::components::control_panel::ControlPanel;
use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="grid grid-cols-12 min-h-screen min-w-screen">
            <div class="col-span-2 w-full h-full">
                <ControlPanel/>
            </div>
            <main class="flex-1 col-span-10 w-full h-full">
                {children()}
            </main>
        </div>
    }
}
