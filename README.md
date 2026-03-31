# Project EggShell - Proxmox Backend API (Loomox)

A lightweight Rust backend service for managing Proxmox virtualization environments.

## Features

### Core Infrastructure
- **Proxmox API Integration** - Seamless proxy and management of Proxmox API calls with token-based authentication
- **Basic Authentication Middleware** - Optional HTTP Basic Auth with Base64 encoding for protected endpoints (disabled by default if credentials not provided)
- **Health Checks** - Endpoint monitoring and diagnostics for application status verification
- **Async Request Handling** - High-performance concurrent request processing using Tokio and Axum
- **Comprehensive Logging** - Structured tracing with configurable log levels (trace, debug, info, warn, error)
- **SSL Certificate Configuration** - Flexible SSL verification with support for custom certificates and self-signed certificates
- **Custom Certificate Support** - Load PEM-formatted certificates from filesystem for secure Proxmox communication

### Virtual Environment Management (Environment)
- **VM Cloning & Provisioning** - Clone virtual machines from templates with automatic naming
- **Bulk Environment Creation** - Create multiple groups with firewall VM cloning capability
- **Module-Based Configuration** - Support for module numbers, class definitions, and user assignments
- **Firewall VM Support** - Optional firewall VM deployment per group with network profile assignment
- **Group Management** - Dynamic group creation with sequential numbering and user list management

### Security & Configuration
- **Proxmox API Token Authentication** - Secure token-based authentication with ID and Secret pairs
- **Admin User Management** - Configurable admin credentials for API access control
- **Environment Variable Configuration** - Core configuration via `APP_` prefixed environment variables
- **Runtime Settings** - Application settings (prefixes, storage, etc.) persisted in SQLite database and configurable via REST API
- **SSL Verification Control** - Enable/disable SSL verification and custom certificate paths 

## Tech Stack

- **Language**: Rust 1.93.0 (Edition 2024)
- **Web Framework**: Axum 0.8 (async HTTP)
- **Async Runtime**: Tokio 1.0
- **HTTP Client**: Reqwest 0.13
- **Database**: Rusqlite 0.32 (SQLite)
- **Serialization**: Serde + JSON
- **Configuration**: Dotenvy + Config
- **CORS**: Tower-HTTP 0.6
- **Observability**: Tracing + Tracing Subscriber
- **Containerization**: Docker support included out of the box

## Dev Environment Setup

### Environment Variables

All app configs must start with `APP_`:

#### Startup Configuration (Environment Variables Only)

These settings are loaded from environment variables at startup and cannot change without restarting:

| Variable                   | Description                                                 | Required?                                     |
|----------------------------|-------------------------------------------------------------|-----------------------------------------------|
| `APP_PROXMOX_URL`          | Proxmox server URL (e.g., `10.142.203.230:8006`)            | **Yes** - Required for Proxmox API connection |
| `APP_PROXMOX_TOKEN_ID`     | Token ID (format: `user@realm!token_name`)                  | **Yes** - Required for Proxmox authentication |
| `APP_PROXMOX_TOKEN_SECRET` | Token secret                                                | **Yes** - Required for Proxmox authentication |
| `APP_USERNAME_ADMIN`       | Admin username for basic auth                               | No - Basic auth disabled when empty           |
| `APP_PASSWORD_ADMIN`       | Admin password for basic auth                               | No - Basic auth disabled when empty           |
| `APP_SERVER_PORT`          | Server port (default: 3000)                                 | No - Defaults to 3000 if not set              |
| `APP_CORS_ORIGIN`          | CORS origin URL (e.g., `http://localhost:3001`)             | No - CORS disabled if not set                 |
| `APP_SSL_VERIFY`           | Enable/disable SSL verification (default: true)             | No - Defaults to true if not set              |
| `APP_SSL_CERT_PATH`        | Path to custom SSL certificate (PEM format)                 | No - Optional custom certificate              |
| `RUST_LOG`                 | Log level: `trace`, `debug`, `info`, `warn`, `error`, `off` | No - Logging disabled by default              |

#### Runtime Settings (Database Persisted, Env Var Defaults Only)

These settings are persisted in SQLite and can be modified at runtime via API. Environment variables are **only used as defaults** if the database is empty:

| Variable                   | Description                                                 | Default Value                    |
|----------------------------|-------------------------------------------------------------|----------------------------------|
| `APP_ROLE`                 | Default role for resource pool assignment                   | `uro_bbzblit_PVELabUsers`        |
| `APP_USER_GROUP_TEMPLATES` | User group templates for bulk operations                    | `ugr_bbzblit_Lernende`           |
| `APP_PREFIX_USER_GROUP`    | Prefix for user group naming                                | `ug`                             |
| `APP_PREFIX_RESOURCEPOOL`  | Prefix for resource pool naming                             | `rp`                             |
| `APP_PREFIX_SIMPLE_ZONE`   | Prefix for simple zone naming                               | `sz`                             |
| `APP_PREFIX_VNETS`         | Prefix for virtual networks                                 | `vn`                             |
| `APP_POSTFIX_VNET_DMZ`     | Postfix for DMZ virtual network                             | `DMZ`                            |
| `APP_POSTFIX_VNET_LAN`     | Postfix for LAN virtual network                             | `LAN`                            |
| `APP_PREFIX_FIREWALL`      | Prefix for firewall VM naming                               | `fw`                             |
| `APP_VM_STORAGE`           | VM storage location (ceph pool or local storage)            | `pvecephpool01`                  |
| `APP_TEMPLATE_STORAGE`     | Template storage location                                   | `templates`                      |
| `APP_WAN_INTERFACE`        | WAN interface bridge name                                   | `vmbr1`                          |

## Build and Run

### Native Build (Linux/macOS/Windows)

#### Prerequisites
- Rust 1.93.0+ ([Install Rust](https://rustup.rs/))
- Cargo (included in Rust)

#### Build
```bash
cargo build --release
```

#### Run
```bash
# Set up environment variables in .env file
# (e.g:)
APP_PROXMOX_URL="10.142.203.230:3000"
APP_PROXMOX_TOKEN_ID="user@realm!token_name"
APP_PROXMOX_TOKEN_SECRET="your_token_secret"
APP_SERVER_PORT="3000"
RUST_LOG="info"
# etc...
```

```bash
# Run the application
cargo run --release
```

The application will start on `http://localhost:3000`

### Using Docker

#### 1. Build Docker Image
```bash
docker build -t m306-proxmox-backend:latest .
```

#### 2. Run Docker Container
```bash
docker run -p 3000:3000 \
  -e APP_PROXMOX_URL="URL:PORT" \
  -e APP_PROXMOX_TOKEN_ID="user@realm!token_name" \
  -e APP_PROXMOX_TOKEN_SECRET="your_token_secret" \
  -e RUST_LOG="info" \
  m306-proxmox-backend:latest
```

#### 3. Alternative: Using Environment File
```bash
docker run -p 3000:3000 --env-file .env m306-proxmox-backend:latest
```

#### 4. Alternative: Using Docker Compose (recommended)
```bash
docker-compose up
```

## API Usage

### API Endpoints

- **Public Endpoints**
  - `GET /health` - Health check with application status and version

- **Protected Endpoints** (requires Basic Auth - *Note: middleware is currently commented out, all routes are open)*
  - `GET /api/auth/verify` - Verify valid credentials
  - `GET /api/node/list` - List all Proxmox cluster nodes
  - `GET /api/environment/list` - List all environments
  - `GET /api/user/list` - List all users
  - `GET /api/group/list` - List all groups
  - `GET /api/vm/list` - List all virtual machines
  - `GET /api/config/storage` - List storage configuration
  - `GET /api/role/list` - List all roles
  - `GET /api/infrastructure/{vm_id}` - Get infrastructure details for a specific VM
  - `GET /api/settings` - Get application settings
  - `PUT /api/settings` - Update application settings
  - `POST /api/config/create` - Create and provision virtual environments with bulk group support
  - `DELETE /api/environment/delete` - Delete environments by group IDs

### Quick Test Examples

Try these endpoints to verify the API is working:

**1. Check Application Health** (No authentication required)
```bash
curl http://localhost:3000/health
# Response: {"status":"ok","version":"0.1.0"}
```

**2. Verify Admin Credentials** (Requires Basic Auth)
```bash
curl -u USER:PASS http://localhost:3000/api/auth/verify
# Response: HTTP 200 OK (empty body indicates valid credentials)
```

Replace `USER` and `PASS` with the values from your `APP_USERNAME_ADMIN` and `APP_PASSWORD_ADMIN` environment variables.

### Testing Authentication

You can also test authentication with the Authorization header:
```bash
curl -H "Authorization: Basic $(echo -n 'admin:password' | base64)" \
  http://localhost:3000/api/auth/verify
```
