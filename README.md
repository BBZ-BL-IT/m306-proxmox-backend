# Loomox - Proxmox Management Backend

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, stable toolchain)
- [Node.js](https://nodejs.org/) (for `npx` / openapi-generator-cli)
- A Proxmox VE instance with API token access

## Project Structure

This is a Cargo workspace with two crates:

```
m306-proxmox-backend/
├── openapi.yaml              # OpenAPI spec (source of truth for the API contract)
├── Cargo.toml                # Workspace root + main application crate
├── src/                      # Application code (you edit this)
│   ├── main.rs
│   ├── app.rs
│   ├── config.rs
│   ├── state.rs              # Implements generated API traits (Health, Proxmox)
│   └── clients/mod.rs        # Proxmox HTTP client
├── generated-api/            # Generated + trimmed server crate
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── models.rs         # Request/response model structs
│       ├── apis/             # API traits (one per tag)
│       └── server/           # Axum router that dispatches to trait impls
└── scripts/
    └── generate-api.sh       # Regenerate + auto-cleanup script
```

## Build & Setup

### 1. Clone the repository

```sh
git clone <repo-url>
cd m306-proxmox-backend
```

### 2. Configure environment variables

Copy the example file and fill in your values:

```sh
cp .env.example .env
```

Edit `.env` with your Proxmox credentials:

```env
APP_PROXMOX_URL=https://your-proxmox-host:8006
APP_PROXMOX_TOKEN_ID=user@pam!token-name
APP_PROXMOX_TOKEN_SECRET=xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
APP_SERVER_PORT=3000
RUST_LOG=info
```

### 3. Build

```sh
cargo build
```

For an optimised release build:

```sh
cargo build --release
```

### 4. Run

```sh
cargo run
```

The server starts on `http://localhost:<APP_SERVER_PORT>` (default `3000`).

## OpenAPI Code Generation

The API contract is defined in `openapi.yaml`. Server models, traits, and the
Axum router are generated from this spec into the `generated-api/` crate using
[openapi-generator](https://openapi-generator.tech/) with the `rust-axum`
generator.

### Regenerating after spec changes

Whenever you modify `openapi.yaml`, regenerate the server crate using the
cleanup script:

```sh
./scripts/generate-api.sh
```

This runs the OpenAPI Generator and then applies automated cleanups (removes
unused files, trims dependencies, strips `Method`/`Host`/`CookieJar` params
from trait signatures, etc.).

> **Note:** `models.rs` and `server/mod.rs` may need manual review after
> regeneration if you add endpoints with new response codes or request bodies.

Then update the application code in `src/` to match any new or changed
endpoints:

1. **New models** are available as `loomox_api::models::*`.
2. **New API trait methods** appear in `loomox_api::apis::<tag>::<Trait>`.
   Implement them on `State` in `src/state.rs`.
3. **Removed endpoints** will cause compile errors in `src/state.rs` -- remove
   the corresponding trait method implementations.

### How it works

| Layer | Location | Editable? |
|---|---|---|
| OpenAPI spec | `openapi.yaml` | Yes -- this is the source of truth |
| Generated models & traits | `generated-api/` | Regenerate with `./scripts/generate-api.sh` |
| Trait implementations | `src/state.rs` | Yes -- your business logic goes here |
| Proxmox HTTP client | `src/clients/mod.rs` | Yes -- talks to the Proxmox API |

The generated crate provides:

- **Models** (`loomox_api::models`) -- Rust structs with serde derives for every
  schema in `openapi.yaml`.
- **API traits** (`loomox_api::apis::health::Health`,
  `loomox_api::apis::proxmox::Proxmox`) -- one async trait per tag, with a
  method per operation.
- **Router** (`loomox_api::server::new(state)`) -- a ready-made `axum::Router`
  that dispatches requests to your trait implementations.

## API Specification

The full API spec is in [`openapi.yaml`](openapi.yaml). Current endpoints:

| Method | Path | Description |
|---|---|---|
| GET | `/health` | Health check |
| GET | `/api/nodes` | List Proxmox nodes |
| GET | `/api/cluster/status` | Get cluster status |
| GET | `/api/cluster/resources` | List cluster resources |

## Dev Environment

All application properties in the **.env** file need their entries to start with `APP_`.

Example:

> APP_PROXMOX_URL=https://your-proxmox-host:8006

Tools from dependencies use their default configuration keys in **.env**.

Example:

> RUST_LOG=info

### .env Reference

| Variable | Required | Default | Description |
|---|---|---|---|
| `APP_PROXMOX_URL` | Yes | -- | Base URL of the Proxmox VE API |
| `APP_PROXMOX_TOKEN_ID` | Yes | -- | API token ID (`user@realm!token-name`) |
| `APP_PROXMOX_TOKEN_SECRET` | Yes | -- | API token secret |
| `APP_SERVER_PORT` | No | `3000` | Port the server listens on |
| `RUST_LOG` | No | -- | Log level filter |

### RUST_LOG levels

- `trace` -- most verbose
- `debug`
- `info`
- `warn`
- `error` -- least verbose
- `off` -- silence everything
