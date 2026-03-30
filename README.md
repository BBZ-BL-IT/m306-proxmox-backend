# Project EggShell - Proxmox Backend API

A lightweight Rust backend service for managing Proxmox virtualization environments.

## Features

### Core Infrastructure
- **Proxmox API Integration** - Seamless proxy and management of Proxmox API calls with token-based authentication
- **Basic Authentication Middleware** - HTTP Basic Auth with Base64 encoding for protected endpoints
- **Health Checks** - Endpoint monitoring and diagnostics for application status verification
- **Async Request Handling** - High-performance concurrent request processing using Tokio and Axum
- **Comprehensive Logging** - Structured tracing with configurable log levels (trace, debug, info, warn, error)
- **SSL Certificate Configuration** - Flexible SSL verification with support for custom certificates and self-signed certificates
- **Custom Certificate Support** - Load PEM-formatted certificates from filesystem for secure Proxmox communication

### API Endpoints
- **Public Endpoints**
  - `GET /health` - Health check with application status and version
  
- **Protected Endpoints** (requires Basic Auth)
  - `GET /auth/verify` - Verify valid credentials
  - `GET /nodes` - List all Proxmox cluster nodes
  - `POST /umgebung/create` - Create and provision virtual environments with bulk group support

### Virtual Environment Management (Umgebung)
- **VM Cloning & Provisioning** - Clone virtual machines from templates with automatic naming
- **Bulk Environment Creation** - Create multiple groups with firewall VM cloning capability
- **Module-Based Configuration** - Support for module numbers, class definitions, and user assignments
- **Firewall VM Support** - Optional firewall VM deployment per group with network profile assignment
- **Group Management** - Dynamic group creation with sequential numbering and user list management

### Security & Configuration
- **Proxmox API Token Authentication** - Secure token-based authentication with ID and Secret pairs
- **Admin User Management** - Configurable admin credentials for API access control
- **Environment Variable Configuration** - All configuration via `APP_` prefixed environment variables
- **SSL Verification Control** - Enable/disable SSL verification and custom certificate paths 

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8 (async HTTP)
- **Async Runtime**: Tokio 1.0
- **HTTP Client**: Reqwest 0.13
- **Serialization**: Serde + JSON
- **Configuration**: Dotenvy + Config
- **Observability**: Tracing + Tracing Subscriber
- **Containerization**: Docker support included

## Dev Environment Setup

All Application properties in the **.env** file needs its entries to start with "APP\_"

example:

> APP_PROXMOX_URL=test

Tools from dependencies use their default configuration in **.env**

example:

> RUST_LOG=info

### Environment Variables

All app configs must start with `APP_`:

| Variable                   | Description                                                 |
|----------------------------|-------------------------------------------------------------|
| `APP_PROXMOX_URL`          | Proxmox server URL (e.g., `10.142.203.230:8006`)            |
| `APP_SERVER_PORT`          | Server port (default: 3000)                                 |
| `RUST_LOG`                 | Log level: `trace`, `debug`, `info`, `warn`, `error`, `off` |
| `APP_PROXMOX_TOKEN_ID`     | Token ID (format: `user@realm!token_name`)                  |
| `APP_PROXMOX_TOKEN_SECRET` | Token secret                                                |
| `APP_USERNAME_ADMIN`       | Admin username for basic auth                               |
| `APP_PASSWORD_ADMIN`       | Admin password for basic auth                               |

## Build and Run

### Native Build (Linux/macOS/Windows)

#### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (included in Rust)

#### Build
```bash
cargo build --release
```

#### Run
```bash
# Set up environment variables in .env file
export APP_PROXMOX_URL="10.142.203.230:8006"
export APP_PROXMOX_TOKEN_ID="user@realm!token_name"
export APP_PROXMOX_TOKEN_SECRET="your_token_secret"
export APP_SERVER_PORT="3000"
export RUST_LOG="info"

# Run the application
cargo run --release
```

The application will start on `http://localhost:3000`

### Docker Build and Run

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

Or with environment file:
```bash
docker run -p 3000:3000 --env-file .env m306-proxmox-backend:latest
```
Or using `docker-compose.yml`:
```bash
docker-compose up
```

### Health Check

After starting the application, verify it's running:
```bash
# Public endpoint (no auth needed)
curl http://localhost:3000/health

# Response:
# {"status":"ok","version":"0.1.0"}
```

### Testing Authentication

```bash
# Test basic auth (replace credentials with your admin credentials)
curl -u admin:password http://localhost:3000/auth/verify

# Test with Authorization header
curl -H "Authorization: Basic $(echo -n 'admin:password' | base64)" \
  http://localhost:3000/auth/verify
```
