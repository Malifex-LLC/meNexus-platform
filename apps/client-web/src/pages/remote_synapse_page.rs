// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use crate::layouts::remote_synapse_layout::RemoteSynapseLayout;
use leptos::prelude::*;

#[component]
pub fn RemoteSynapsePage() -> impl IntoView {
    view! {
        <RemoteSynapseLayout/>
    }
}
