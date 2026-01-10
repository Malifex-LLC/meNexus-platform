// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

#![recursion_limit = "512"]

pub mod app;
pub mod components;
pub mod layouts;
pub mod pages;

use crate::app::App;
use leptos::config::LeptosOptions;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

pub fn leptos_options() -> LeptosOptions {
    // Use LEPTOS_SITE_ROOT env var for production, fallback to target/site for dev
    let site_root = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "target/site".to_string());
    let site_pkg_dir =
        std::env::var("LEPTOS_SITE_PKG_DIR").unwrap_or_else(|_| "pkg".to_string());

    LeptosOptions::builder()
        .site_root(site_root)
        .site_pkg_dir(site_pkg_dir)
        .output_name("client-web")
        .build()
}
