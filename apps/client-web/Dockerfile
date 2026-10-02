FROM rust:1.98.1-trixie

WORKDIR /workspace/apps/client-web

# Install tools needed for Leptos builds
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --version 0.3.10 --locked
ENV LEPTOS_WASM_BINDGEN_VERSION=0.2.108
ENV LEPTOS_TAILWIND_VERSION=v4.3.3
ENV LEPTOS_WASM_OPT_VERSION=version_133

EXPOSE 5173

CMD ["cargo", "leptos", "watch"]
