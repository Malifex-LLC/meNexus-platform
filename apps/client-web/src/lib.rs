use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};

/// Hydration entry point for WASM - called when the page loads in the browser
#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}

/// Shell component for SSR - renders the full HTML document
#[component]
pub fn Shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
                <HydrationScripts options/>
            </body>
        </html>
    }
}

/// Main app component - this is what gets hydrated on the client
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="meNexus"/>
        <Stylesheet id="leptos" href="/pkg/client-web.css"/>
        <Meta name="description" content="meNexus Platform"/>

        <main style="min-height:100vh;display:flex;flex-direction:column;justify-content:center;align-items:center;gap:1rem;background:#000;color:#fff;">
            <h1>"Welcome to meNexus"</h1>
            <SimpleCounter initial_value=0/>
        </main>
    }
}

#[component]
fn SimpleCounter(initial_value: i32) -> impl IntoView {
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

pub fn leptos_options() -> LeptosOptions {
    LeptosOptions::builder()
        .site_root("target/site")
        .site_pkg_dir("pkg")
        .output_name("client-web")
        .build()
}
