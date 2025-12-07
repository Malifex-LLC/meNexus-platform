// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::main_layout::MainLayout;
use crate::layouts::three_column_module_layout::ThreeColumnModuleLayout;
use leptos::prelude::*;

#[component]
pub fn SearchLayout() -> impl IntoView {
    view! {
        <MainLayout>
           Search Page !
        </MainLayout>
    }
}
