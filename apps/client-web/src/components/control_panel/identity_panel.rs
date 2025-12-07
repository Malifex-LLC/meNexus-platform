// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use icondata as icons;
use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn IdentityPanel() -> impl IntoView {
    view! {
        <div class="flex-col p-4 bg-panel text-foreground gap-4 w-full text-3xl justify-center border border-border rounded-xl">
            <div class="flex-col text-center">
                <div>Jacob - Malifex</div>
                <div class="text-brand">@jacobwileyross</div>
            </div>
        </div>
    }
}
