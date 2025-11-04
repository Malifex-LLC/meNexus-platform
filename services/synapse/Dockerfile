FROM rust:1-bookworm

WORKDIR /workspace/services/synapse

# Watcher for hot-reload
RUN cargo install watchexec-cli

EXPOSE 3000

# CMD ["watchexec","--poll","--clear","--restart","--watch","/workspace/services/synapse","--ignore","/workspace/services/synapse/target","--","cargo","run","-p","synapse-server"]
CMD ["cargo","run","-p","synapse-server"]
