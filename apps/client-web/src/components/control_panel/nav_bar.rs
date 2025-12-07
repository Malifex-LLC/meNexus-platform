// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use icondata as icons;
use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_router::components::A;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <nav class="flex p-4 bg-panel text-foreground gap-4 w-full text-3xl justify-center border border-border rounded-xl">
            <A href="/" >
                <Icon icon={icons::RiHomeWifiBuildingsLine}/>
            </A>
            <A href="/synapse" >
                <Icon icon={icons::IoGitNetworkSharp}/>
            </A>
            <A href="/search" >
                <Icon icon={icons::MdiCardSearchOutline}/>
            </A>
            <A href="/profile" >
                <Icon icon={icons::OcPersonSm}/>
            </A>
            <A href="/settings" >
                <Icon icon={icons::VsSettings}/>
            </A>
            <A href="/notifications" >
                <Icon icon={icons::RiNotification2MediaLine}/>
            </A>
        </nav>
    }
}
