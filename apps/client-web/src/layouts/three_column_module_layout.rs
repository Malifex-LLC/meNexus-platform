// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use module_posts::ui::components::post_feed::PostsFeed;

#[component]
pub fn ThreeColumnModuleLayout() -> impl IntoView {
    view! {
        <div class="grid grid-cols-12 h-full w-full">
            <div class="flex flex-col col-span-4">Column 1</div>
            <div class="flex flex-col col-span-5">
                Column 2
                <PostsFeed/>
            </div>
            <div class="flex flex-col col-span-3">
                Column 3
            </div>
        </div>
    }
}
