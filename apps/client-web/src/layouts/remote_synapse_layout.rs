// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

//! # Remote Synapse Layout
//!
//! Layout for rendering a remote Synapse. This dispatches a `synapse:get_manifest`
//! event to the target Synapse, receives the manifest, and renders the Synapse accordingly.

use crate::components::synapse::RemoteSynapseRenderer;
use crate::layouts::main_layout::MainLayout;
use leptos::prelude::*;
use leptos_router::{hooks::use_params, params::Params};

#[derive(Clone, PartialEq, Params)]
struct SynapseParams {
    synapse_public_key: Option<String>,
}

#[component]
pub fn RemoteSynapseLayout() -> impl IntoView {
    let synapse_params = use_params::<SynapseParams>();
    
    // Create a signal from the URL parameter
    let synapse_public_key = Signal::derive(move || {
        synapse_params.with(|res| match res {
            Ok(p) => p.synapse_public_key.clone().unwrap_or_default(),
            Err(_) => String::new(),
        })
    });

    view! {
        <MainLayout>
            <div class="h-full w-full">
                <RemoteSynapseRenderer synapse_public_key=synapse_public_key/>
            </div>
        </MainLayout>
    }
}
