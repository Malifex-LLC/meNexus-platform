// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;

#[component]
pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
    let (value, set_value) = signal(initial_value);

    let clear = move |_| set_value.set(0);
    let decrement = move |_| set_value.update(|n| *n -= 1);
    let increment = move |_| set_value.update(|n| *n += 1);

    view! {
        <div style="display:flex;gap:0.75rem;align-items:center;">
            <button on:click=clear>"Reset"</button>
            <button on:click=decrement>"-1"</button>
            <span>{move || format!("Count: {}", value.get())}</span>
            <button on:click=increment>"+1"</button>
        </div>
    }
}
