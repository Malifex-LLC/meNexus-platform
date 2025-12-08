// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use icondata as icons;
use leptos::prelude::*;
use leptos_icons::Icon;

use crate::app::SessionUserProfile;

#[component]
pub fn IdentityPanel() -> impl IntoView {
    let session_user =
        use_context::<SessionUserProfile>().expect("SessionUserProfile context not found");

    view! {
        <div class="flex-col p-4 bg-panel text-foreground gap-4 w-full text-3xl justify-center border border-border rounded-xl">
            <Suspense fallback=|| view! { <div>"Loading..."</div> }>
                {move || {
                    session_user.get().map(|result| {
                        match result {
                            Ok(Some(profile)) => view! {
                                <div class="flex-col text-center">
                                    <div>{profile.display_name.unwrap_or_default()}</div>
                                    <div class="text-brand">
                                        {format!("@{}", profile.handle.unwrap_or_default())}
                                    </div>
                                </div>
                            }.into_any(),
                            Ok(None) => view! {
                                <div>"Not logged in"</div>
                            }.into_any(),
                            Err(_) => view! {
                                <div>"Error loading profile"</div>
                            }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
