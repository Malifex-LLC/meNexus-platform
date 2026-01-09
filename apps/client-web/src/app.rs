// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::pages::dashboard_page::DashboardPage;
use crate::pages::login_page::LoginPage;
use crate::pages::messenger_page::MessengerPage;
use crate::pages::notifications_page::NotificationsPage;
use crate::pages::profile_page::ProfilePage;
use crate::pages::register_page::RegisterPage;
use crate::pages::remote_synapse_page::RemoteSynapsePage;
use crate::pages::settings_page::SettingsPage;
use crate::pages::synapse_page::SynapsePage;
use crate::{layouts::main_layout::MainLayout, pages::search_page::SearchPage};
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use module_profiles::server_fns::get_session_user_profile;
use synapse_core::domain::profiles::Profile;

pub type SessionUserProfile = Resource<Result<Option<Profile>, ServerFnError>>;

#[component]
pub fn Shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>

                // These will be rendered into <head>
                <Title text="meNexus"/>
                <Meta name="description" content="meNexus Platform"/>
                <Stylesheet id="leptos" href="/pkg/client-web.css"/>

                <MetaTags/>
            </head>
            <body>
                <App/>
                <HydrationScripts options/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let session_user = Resource::new(
        || (), // No dependencies - fetch once
        |_| get_session_user_profile(),
    );

    provide_context(session_user);

    view! {
        <Router>
            <Routes fallback=SynapsePage>
                <Route path=path!("/register") view=RegisterPage/>
                <Route path=path!("/login") view=LoginPage/>
                <Route path=path!("/") view=DashboardPage/>
                <Route path=path!("/synapse") view=SynapsePage/>
                <Route path=path!("/synapses/:synapse_public_key") view=RemoteSynapsePage/>
                <Route path=path!("/profile") view=ProfilePage/>
                <Route path=path!("/search") view=SearchPage/>
                <Route path=path!("/settings") view=SettingsPage/>
                <Route path=path!("/notifications") view=NotificationsPage/>
                <Route path=path!("/messages") view=MessengerPage/>
            </Routes>
        </Router>
    }
}
