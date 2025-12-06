// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

pub mod app;
pub mod components;
pub mod layouts;

use crate::app::App;
use leptos::config::LeptosOptions;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

pub fn leptos_options() -> LeptosOptions {
    LeptosOptions::builder()
        .site_root("target/site")
        .site_pkg_dir("pkg")
        .output_name("client-web")
        .build()
}
