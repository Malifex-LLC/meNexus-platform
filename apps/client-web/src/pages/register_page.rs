// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::register_layout::RegisterLayout;
use leptos::prelude::*;

#[component]
pub fn RegisterPage() -> impl IntoView {
    view! {
        <RegisterLayout/>
    }
}
