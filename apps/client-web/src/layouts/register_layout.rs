// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use module_auth::ui::components::register_form::RegisterForm;

#[component]
pub fn RegisterLayout() -> impl IntoView {
    view! {
        <div>
            <RegisterForm/>
        </div>
    }
}
