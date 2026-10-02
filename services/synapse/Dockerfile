FROM rust:1.98.1-trixie

WORKDIR /workspace/services/synapse

# Tooling shared by backend runtime and frontend watcher
RUN rustup target add wasm32-unknown-unknown \
  && cargo install watchexec-cli --version 2.7.3 --locked \
  && cargo install cargo-leptos --version 0.3.10 --locked

EXPOSE 3000

# CMD ["watchexec","--poll","--clear","--restart","--watch","/workspace/services/synapse","--ignore","/workspace/services/synapse/target","--","cargo","run","-p","synapse-server"]
CMD ["cargo","run","-p","synapse-server"]
