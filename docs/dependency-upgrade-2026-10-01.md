# Dependency upgrade — 2026-10-01

Versions were checked against crates.io, upstream GitHub releases, and Docker
Hub and npm. Direct dependencies target current stable releases, except for the browser
binding compatibility constraint below. `Cargo.lock` resolves transitive
dependencies to the newest versions allowed by the selected upstream packages.

## Rust dependencies

| Dependency | Selected version |
| --- | --- |
| anyhow | 1.0.104 |
| async-trait | 0.1.92 |
| automerge | 0.12.0 |
| axum | 0.8.9 |
| axum-extra | 0.12.6 |
| base64 | 0.23.1 |
| console_error_panic_hook | 0.1.7 |
| dashmap | 6.2.1 |
| dotenvy | 0.15.7 |
| futures | 0.3.34 |
| getrandom | 0.4.3 |
| hex | 0.4.3 |
| icondata | 0.7.0 |
| k256 | 0.14.0 |
| leptos | 0.8.21 |
| leptos_axum | 0.8.10 |
| leptos_icons | 0.7.1 |
| leptos_meta | 0.8.7 |
| leptos_router | 0.8.16 |
| libp2p | 0.57.0 |
| libp2p-identity | 0.3.0 |
| libp2p-kad | 0.49.0 |
| libp2p-mdns | 0.49.0 |
| libp2p-swarm-derive | 0.36.0 |
| log | 0.4.34 |
| rand | 0.10.3 |
| rand_core | 0.10.1 |
| reqwasm | 0.5.0 |
| serde | 1.0.229 |
| serde_json | 1.0.151 |
| sha2 | 0.11.0 |
| sqlx | 0.9.0 |
| thiserror | 2.0.21 |
| time | 0.3.55 |
| tokio | 1.53.1 |
| tower-http | 0.7.1 |
| tracing | 0.1.44 |
| tracing-subscriber | 0.3.23 |
| url | 2.5.8 |
| uuid | 1.26.1 |
| wasm-bindgen | 0.2.108 (exact pin) |
| web-sys | 0.3.85 |

`console_error_panic_hook`, `dotenvy`, `hex`, and `reqwasm` already used their
latest stable releases.

### Browser binding constraint

libp2p 0.57 uses libp2p-swarm 0.48, which pins wasm-bindgen-futures to 0.4.58.
That package requires js-sys/web-sys 0.3.85 and wasm-bindgen 0.2.108. Selecting
the latest wasm-bindgen 0.2.129 or web-sys 0.3.106 causes Cargo resolution to
fail. Keep this family synchronized until libp2p relaxes its upstream pin.

The binding version is synchronized in the frontend manifest, Compose build
environment, example environment, and production Dockerfile. cargo-leptos
detects the binding ABI from Cargo metadata when generating JavaScript/WASM.

## Build tools, images, and CI

| Component | Selected version |
| --- | --- |
| Rust Docker builders | 1.98.1-trixie |
| Production runtime | debian:trixie-slim |
| PostgreSQL (development and production) | 18.6-alpine |
| Nginx | 1.31.6-alpine |
| cargo-leptos | 0.3.10, installed with `--locked` |
| watchexec-cli | 2.7.3, installed with `--locked` |
| Tailwind CSS | v4.3.3 |
| Binaryen/wasm-opt | version_133 |
| ReDoc API documentation renderer | v2.5.4 |
| Redocly documentation generator | 2.57.0 |
| actions/checkout | v7.0.1 |
| docker/setup-buildx-action | v4.4.1 |
| docker/login-action | v4.6.0 |
| docker/build-push-action | v7.4.0 |
| softprops/action-gh-release | v3.0.3 |

OS packages installed by apt/apk use the selected distribution's package
repositories. Trixie names the OpenSSL runtime package `libssl3t64`.

## Compatibility changes

- Adapted ECDSA digest signing/verification to k256 0.14's closure-based API.
  SHA-256 hashing and compressed SEC1 public keys retain their existing formats.
- Replaced `to_encoded_point` with `to_sec1_point` in browser key derivation.
- Enabled URL's serde feature explicitly; the workspace previously relied on
  another dependency enabling it transitively.
- Mounted PostgreSQL volumes at `/var/lib/postgresql`, as required by 18+.
- Corrected a pre-existing stale test to describe the existing federation trust
  contract. Authentication policy was not changed.
- Added an externally generated signature fixture that verifies the canonical
  event payload and rejects altered content.
- Regenerated the static API documentation with Redocly CLI 2.57.0 and ReDoc
  2.5.4, preserving the existing theme and using the generator's script integrity
  hash. The prerendered React markup produced hydration mismatches even after
  regeneration, so the browser initializes ReDoc from the embedded spec/options
  instead of hydrating the static markup. The page remains standalone.

The existing SQLx offline query cache is accepted by SQLx 0.9 and the production
build. Query/schema changes were not needed for the upgrade.

## Verification

- `SQLX_OFFLINE=true cargo check --workspace --locked`
- `SQLX_OFFLINE=true cargo test --workspace --locked`: five unit tests pass,
  plus workspace documentation tests.
- Complete `cargo leptos build --release --project client-web`, covering the
  native server, WASM hydration, Tailwind, binding generation, and wasm-opt 133.
- Development Compose image and production Synapse/proxy image builds.
- Development and production Compose configuration validation.
- Isolated PostgreSQL 18 initialization and SQLx migration startup.
- HTTP health, rendered pages, manifest/config, and JavaScript/WASM assets.
- Challenge login using an independent OpenSSL-backed secp256k1 signer.
- Automerge profile persistence and post creation/channel listing.
- Chromium browser hydration and private-key login, including reactive input,
  client-side signing, session cookie, redirect, and dashboard rendering.
- Two-container libp2p connection and remote posts-configuration request.
- Nginx configuration validation and proxied health request.
- Chromium ReDoc initialization and logged-out Synapse/chat-tab interaction.

GitHub release/publishing actions require a real GitHub workflow run and were
not executed locally. Certificate issuance requires a public domain and was
not exercised. Existing compiler warnings remain, including an upstream future
compatibility warning for proc-macro-error2. cargo-leptos's published tool
lockfile also contains yanked packages; its locked installation succeeds.

### Pre-existing runtime issue

The authenticated `/synapse` page begins streaming HTML but does not finish,
preventing normal page load/hydration. This was reproduced with the original
checkout and its original locked dependencies using a profile created by that
checkout, as well as with the upgraded production image. The logged-out Synapse
page, upgraded authenticated dashboard, login, and direct post/profile APIs
work. The page-rendering issue is not resolved by this dependency upgrade and
needs a separate investigation of the nested resource/Suspense flow.

## Running the upgraded development stack

Existing `.env` files override Compose defaults. Match these build settings:

```dotenv
LEPTOS_WASM_BINDGEN_VERSION=0.2.108
LEPTOS_TAILWIND_VERSION=v4.3.3
LEPTOS_WASM_OPT_VERSION=version_133
```

Then run:

```sh
docker compose -f compose.dev.yaml up --build
```

For native builds, use Rust 1.98.1 with the `wasm32-unknown-unknown` target and
install `cargo-leptos --version 0.3.10 --locked`. Export the tooling settings
above when running the release build. The first build downloads new dependencies
and recompiles both the server and browser application.

Production PostgreSQL moves from 17 to 18 under the confirmed fresh-deployment
assumption. An old PostgreSQL 17 volume, if introduced later, must be migrated
before use with PostgreSQL 18; changing the mount alone does not migrate data.

The existing development named volume is preserved. Fresh volumes and existing
PostgreSQL 18 volumes with data under `18/docker` work with the new mount. If a
development volume contains database files directly at its root (for example,
an older PostgreSQL cluster), stop the stack and back up the volume before
changing its contents. Use the matching PostgreSQL major version to dump the
database, then restore into a fresh PostgreSQL 18 volume. The mount change does
not relocate or upgrade existing cluster files. Do not use `down -v` to solve
this if the volume contains data you need.

## Regenerating API documentation

From `docs/openapi`, run:

```sh
npx --yes @redocly/cli@2.57.0 build-docs openapi.yaml \
  --output index.html --disableGoogleFont --title 'meNexus Synapse API'
```

This uses `redocly.yaml` to preserve the custom theme. The generator reports an
existing incompatible `allOf` type in `PaginatedArtifactUris`; the schema is
unchanged by this dependency upgrade.

After generation, replace the final `Redoc.hydrate(__redoc_state, container)`
call with the initialization used in the checked-in HTML to avoid the generated
markup's browser hydration errors:

```js
container.innerHTML = '';
Redoc.init(__redoc_state.spec.data, __redoc_state.options, container);
```
