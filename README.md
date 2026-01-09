<h1 align="center">meNexus</h1>
<p align="center">
  <strong>A peer-to-peer social layer for the open internet</strong>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-AGPL--v3-blue.svg" alt="License: AGPL v3"/></a>
  <img src="https://img.shields.io/badge/status-early--alpha-orange.svg" alt="Early Alpha"/>
  <img src="https://img.shields.io/badge/rust-2024-orange.svg" alt="Rust 2024"/>
</p>

<p align="center">
  <a href="#quickstart">Quickstart</a> •
  <a href="#what-is-menexus">About</a> •
  <a href="#architecture">Architecture</a> •
  <a href="#modules">Modules</a> •
  <a href="#configuration">Configuration</a> •
  <a href="#contributing">Contributing</a>
</p>

---

## What is meNexus?

**meNexus** is a decentralized social networking platform that empowers individuals and communities through meaningful connections, autonomy, and collaboration.

Rather than forcing users into a one-size-fits-all experience, meNexus allows every community to run their own **Synapse**—an autonomous, configurable node that can communicate with any other node in the network.

Your identity is portable.  
Your community is sovereign.  
Your data is yours.

---

## 🚧 Project Status

meNexus is currently in **early alpha development**.

- Core architecture and federation are actively evolving
- APIs, schemas, and internal boundaries may change
- Deployment tooling is **development-focused**
- A production-ready release configuration is **planned but not yet finalized**

At this stage, the **development workflow is the recommended way to run meNexus**.  
A hardened production Docker/Compose setup will be introduced once the architecture stabilizes.

---

## Quickstart (Development)

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### 1. Clone & Configure

```bash
git clone https://github.com/Malifex-LLC/meNexus-platform.git
cd meNexus-platform

cp env.example .env
```

Open `.env` and customize your Synapse—name it, choose your modules, and set your theme.

### 2. Launch (Development Mode)

```bash
docker compose -f compose.dev.yaml up
```

This runs the full stack in development mode, including:

- Axum + Leptos SSR
- Hot-reloading via cargo-leptos
- Source-mounted containers for rapid iteration

Your Synapse will be available at:

**<http://localhost:3000>**

⚠️ **Note**: `compose.yaml` is reserved for a future production/release configuration and is not yet supported.

---

## Architecture

meNexus is built with a modular, hexagonal architecture designed for extensibility, federation, and long-term evolution.

```
┌─────────────────────────────────────────────────────────┐
│                      client-web                         │
│               Leptos + WebAssembly + Tailwind           │
├─────────────────────────────────────────────────────────┤
│                     synapse-server                      │
│                   Axum + Leptos SSR                     │
├─────────────────────────────────────────────────────────┤
│                   synapse-application                   │
│              Module orchestration layer                 │
├────────────────────┬────────────────────────────────────┤
│   synapse-core     │           modules                  │
│   Domain types     │   posts · chat · members · ...     │
├────────────────────┴────────────────────────────────────┤
│                       adapters                          │
│            PostgreSQL · libp2p · (more...)              │
└─────────────────────────────────────────────────────────┘
```

### Tech Stack

| Layer | Technology |
|-------|-----------|
| Language | Rust 2024 Edition |
| Web Framework | Axum |
| Frontend | Leptos (WASM + SSR hydration) |
| Styling | Tailwind CSS v4 |
| Database | PostgreSQL |
| P2P Networking | libp2p |
| Cryptography | secp256k1 (k256) |
| Runtime | Tokio |

---

## Modules

Synapses are composed of pluggable modules. Enable only what your community needs:

| Module | Description | Status |
|--------|-------------|--------|
| posts | Long-form content with channels | 🔶 Alpha |
| chat | Real-time messaging rooms | 🔹 Preview |
| members | Member directory and management | 🔹 Preview |
| activity | Live activity feed | 🔹 Preview |
| livestream | Live video streaming | 🔹 Preview |

**Status Key**

- 🔶 Alpha — Functional but incomplete; expect breaking changes
- 🔹 Preview — UI prototype with mock data; backend wiring in progress

Enable modules in your `.env`:

```bash
SYNAPSE_MODULES=posts,chat,members,activity
```

---

## Configuration

All configuration is done via environment variables.

Copy `env.example` to `.env` and customize as needed.

### Identity

```bash
SYNAPSE_NAME="My Community"
SYNAPSE_DESCRIPTION="A place for like-minded people"
SYNAPSE_PUBLIC_URL=http://localhost:3000
```

### Layout

```bash
SYNAPSE_LAYOUT=TwoColumn
SYNAPSE_LAYOUT_LEFT=posts,chat
SYNAPSE_LAYOUT_RIGHT=members,activity
```

### Theme

```bash
SYNAPSE_THEME_PRESET=default
SYNAPSE_DARK_MODE=true
SYNAPSE_ACCENT_COLOR=#your-hex
```

### Federation

```bash
SYNAPSE_FEDERATION=true
SYNAPSE_OPEN_MEMBERSHIP=true
SYNAPSE_GUEST_ACCESS=false
```

See `env.example` for the full reference.

---

## Development Notes

- `compose.dev.yaml` is the only supported Compose file
- Optimized production images and deployment hardening are planned
- Expect breaking changes while core systems stabilize

---

## Contributing

Contributions are welcome and encouraged.

1. Fork the repository
2. Create a feature branch: `git checkout -b my-feature`
3. Commit with sign-off: `git commit -s -m "Add feature"`
4. Open a pull request

All commits must include a Signed-off-by line (Developer Certificate of Origin – DCO).

See [CONTRIBUTING.md](CONTRIBUTING.md) for full guidelines.

---

## License

This project is licensed under the GNU Affero General Public License v3.0 or later.

- **Code**: AGPL-3.0-or-later
- **Documentation & Assets**: CC BY-SA 4.0

---

<p align="center"> Built with care by <a href="https://github.com/Malifex-LLC">Malifex</a> and the open-source community </p>

