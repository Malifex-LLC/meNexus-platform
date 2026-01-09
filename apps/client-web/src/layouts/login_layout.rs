// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;
use module_auth::ui::components::login_form::LoginForm;

#[component]
pub fn LoginLayout() -> impl IntoView {
    view! {
        <div>
            <LoginForm/>
        </div>
    }
}
