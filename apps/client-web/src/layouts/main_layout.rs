// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="min-h-screen min-w-screen">
            <main class="flex-1">
                {children()}
            </main>
        </div>
    }
}
