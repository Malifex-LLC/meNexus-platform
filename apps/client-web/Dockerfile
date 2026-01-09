FROM rust:1-bookworm

WORKDIR /workspace/apps/client-web

# Install tools needed for Leptos builds
RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos

EXPOSE 5173

CMD ["cargo", "leptos", "watch"]
