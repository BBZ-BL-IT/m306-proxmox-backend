#!/usr/bin/env bash
# -------------------------------------------------------------------
# generate-api.sh
#
# 1. Runs the OpenAPI Generator to produce the raw rust-axum crate.
# 2. Applies automated cleanups so the output stays lean.
#
# Prerequisites:
#   - npx (Node.js) OR a global install of @openapitools/openapi-generator-cli
#   - The openapi.yaml spec at the project root
# -------------------------------------------------------------------
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

SPEC="$PROJECT_ROOT/openapi.yaml"
OUTPUT="$PROJECT_ROOT/generated-api"
GENERATOR="rust-axum"
PACKAGE_NAME="loomox-api"

# ── Step 1: Generate ─────────────────────────────────────────────────
echo ">>> Generating API from $SPEC ..."
npx @openapitools/openapi-generator-cli generate \
  -i "$SPEC" \
  -g "$GENERATOR" \
  -o "$OUTPUT" \
  --package-name "$PACKAGE_NAME" \
  --additional-properties=packageVersion=0.1.0

echo ">>> Generation complete. Applying cleanups ..."

SRC="$OUTPUT/src"

# ── Step 2: Delete files we don't need ───────────────────────────────
rm -f "$SRC/header.rs"
rm -f "$SRC/types.rs"

# ── Step 3: Rewrite lib.rs ───────────────────────────────────────────
cat > "$SRC/lib.rs" << 'RUST'
#![allow(
    missing_docs,
    unused_variables,
    unused_imports,
)]

pub const BASE_PATH: &str = "";
pub const API_VERSION: &str = "0.1.0";

#[cfg(feature = "server")]
pub mod server;

pub mod models;
pub mod apis;
RUST

# ── Step 4: Rewrite Cargo.toml with minimal deps ────────────────────
cat > "$OUTPUT/Cargo.toml" << 'TOML'
[package]
name = "loomox-api"
version = "0.1.0"
authors = ["OpenAPI Generator team and contributors"]
description = "Proxmox management backend API"
edition = "2024"

[features]
default = ["server"]
server = []

[dependencies]
async-trait = "0.1"
axum = "0.8"
http = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = { version = "0.1", features = ["attributes"] }

[dev-dependencies]
tracing-subscriber = "0.3"
TOML

# ── Step 5: Clean up models.rs ───────────────────────────────────────
# Replace Nullable<T> with Option<T>, strip Display/FromStr/Header impls,
# remove XSS checks, ammonia references, validator derives.
MODELS="$SRC/models.rs"
if [ -f "$MODELS" ]; then
  # Remove everything and write clean models.
  # This is the nuclear option — you maintain models.rs by hand after this,
  # OR re-run this script which regenerates then overwrites with the cleaned
  # version from the section below.
  echo "    NOTE: models.rs requires manual review after generation."
  echo "    The generator output includes Display/FromStr/Header/XSS code"
  echo "    that should be stripped. See the current models.rs for the"
  echo "    expected clean format."
fi

# ── Step 6: Clean up API trait files ─────────────────────────────────
# Remove Method/Host/CookieJar params from trait signatures.
# This is a pattern-based replacement.
for f in "$SRC/apis/health.rs" "$SRC/apis/proxmox.rs"; do
  if [ -f "$f" ]; then
    # Remove unused imports added by the generator
    sed -i '/^use axum::extract::\*/d' "$f"
    sed -i '/^use axum_extra::extract::CookieJar;/d' "$f"
    sed -i '/^use bytes::Bytes;/d' "$f"
    sed -i '/^use headers::Host;/d' "$f"
    sed -i '/^use http::Method;/d' "$f"
    # Remove use of crate::types
    sed -i 's/use crate::{models, types::\*};/use crate::models;/' "$f"

    # Remove method/host/cookies params from trait method signatures
    # These appear as lines like: "    method: &Method,"
    sed -i '/^\s*method: \&Method,$/d' "$f"
    sed -i '/^\s*host: \&Host,$/d' "$f"
    sed -i '/^\s*cookies: \&CookieJar,$/d' "$f"

    # Add #[allow(non_camel_case_types)] before enum definitions
    sed -i 's/^#\[must_use\]$/#[must_use]\n#[allow(non_camel_case_types)]/' "$f"
  fi
done

# ── Step 7: Clean up apis/mod.rs ─────────────────────────────────────
cat > "$SRC/apis/mod.rs" << 'RUST'
pub mod health;
pub mod proxmox;

/// Error handler for unhandled errors.
#[async_trait::async_trait]
pub trait ErrorHandler<E: std::fmt::Debug + Send + Sync + 'static = ()> {
    #[allow(unused_variables)]
    async fn handle_error(&self, error: E) -> Result<axum::response::Response, http::StatusCode> {
        tracing::error!("Unhandled error: {:?}", error);
        axum::response::Response::builder()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body(axum::body::Body::empty())
            .map_err(|_| http::StatusCode::INTERNAL_SERVER_ERROR)
    }
}
RUST

# ── Step 8: Clean up server/mod.rs ──────────────────────────────────
# The server file needs the most rewriting — the generated version extracts
# Host/CookieJar on every handler. We leave a NOTE for manual review since
# this file's structure changes with every new endpoint.
echo "    NOTE: server/mod.rs requires manual review after generation."
echo "    Remove Host/CookieJar extraction and update trait calls."

echo ">>> Cleanup complete. Run 'cargo check' to verify."
