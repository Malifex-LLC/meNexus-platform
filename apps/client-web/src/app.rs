// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::main_layout::MainLayout;
use crate::layouts::three_column_module_layout::ThreeColumnModuleLayout;
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};

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
    view! {
        <MainLayout>
            <ThreeColumnModuleLayout/>
        </MainLayout>
    }
}
