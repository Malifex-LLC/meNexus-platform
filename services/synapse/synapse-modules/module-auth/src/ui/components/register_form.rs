// SPDX-License-Identifier: AGPL-3.0-or-later
// Copyright © 2025 Malifex LLC and contributors

use leptos::prelude::*;
use leptos_meta::Title;
use leptos_router::components::A;

#[component]
pub fn RegisterForm() -> impl IntoView {
    let (handle, set_handle) = signal(String::new());
    let (display_name, set_display_name) = signal(String::new());
    let (is_loading, set_is_loading) = signal(false);

    let on_submit =
        move |ev: leptos::ev::SubmitEvent| leptos::logging::log!("Register button submitted!");

    view! {
        <Title text="Register | meNexus"/>

        <div class="max-w-md flex align-center items-center justify-center bg-panel p-4 border border-border rounded-xl">
            <div class="w-full max-w-md justify-center">
                // Main card
                <div class="card text-center">
                    // Header
                    <h1 class="text-3xl font-bold text-foreground mb-1">
                        "Welcome to"
                    </h1>
                    <h2 class="text-3xl font-bold text-brand mb-4">
                        "meNexus"
                    </h2>
                    <p class="text-foreground-muted mb-6">
                        "Create your account to get started!"
                    </p>

                    // Form card
                    <form
                        class="bg-background rounded-lg p-6 mb-6 text-left"
                        on:submit=on_submit
                    >
                        // Handle field
                        <label class="block text-foreground font-medium mb-2">
                            "Handle:"
                        </label>
                        <input
                            type="text"
                            class="w-full px-4 py-3 bg-surface border border-border rounded-lg
                                   text-foreground placeholder-foreground-muted
                                   focus:outline-none focus:border-brand transition-colors mb-4"
                            prop:value=move || handle.get()
                            on:input=move |ev| {
                                set_handle.set(event_target_value(&ev));
                            }
                        />

                        // Display Name field
                        <label class="block text-foreground font-medium mb-2">
                            "Display Name:"
                        </label>
                        <input
                            type="text"
                            class="w-full px-4 py-3 bg-surface border border-border rounded-lg
                                   text-foreground placeholder-foreground-muted
                                   focus:outline-none focus:border-brand transition-colors mb-6"
                            prop:value=move || display_name.get()
                            on:input=move |ev| {
                                set_display_name.set(event_target_value(&ev));
                            }
                        />

                        <button
                            type="submit"
                            class="w-full py-3 bg-brand text-foreground-inverse font-semibold
                                   rounded-lg hover:bg-secondary transition-colors
                                   disabled:opacity-50 disabled:cursor-not-allowed"
                            disabled=move || is_loading.get()
                        >
                            {move || if is_loading.get() { "Registering..." } else { "Register" }}
                        </button>
                    </form>

                    // Login link
                    <p class="text-foreground-muted">
                        "Already have an account? "
                        <A href="/login" attr:class="text-brand hover:text-secondary transition-colors">
                            "Login!"
                        </A>
                    </p>
                </div>
            </div>
        </div>
    }
}
