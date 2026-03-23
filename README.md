# Project EggShell - Proxmox Backend API

A lightweight Rust backend service for managing Proxmox virtualization environments.

## Features

- **Proxmox API Integration** - Proxy and manage Proxmox API calls
- **Basic Authentication** - Token-based auth for secure API access
- **Health Checks** - Endpoint monitoring and diagnostics
- **Async Request Handling** - High-performance concurrent request processing
- **Comprehensive Logging** - Structured tracing for debugging and monitoring

## Tech Stack

- **Language**: Rust (Edition 2024)
- **Web Framework**: Axum 0.8 (async HTTP)
- **Async Runtime**: Tokio 1.0
- **HTTP Client**: Reqwest 0.13
- **Serialization**: Serde + JSON
- **Configuration**: Dotenvy + Config
- **Observability**: Tracing + Tracing Subscriber
- **Containerization**: Docker support included

## TODO

- [ ] SSL certificate configuration
- [ ] Proxmox certificate auto-import

## Dev Environment Setup

All Application properties in the **.env** file needs its entries to start with "APP\_"

example:

> APP_PROXMOX_URL=test

Tools from dependencies use their default configuration in **.env**

example:

> RUST_LOG=info

### Environment Variables

All app configs must start with `APP_`:

| Variable | Description |
|----------|-------------|
|`APP_PROXMOX_URL` | Proxmox server URL (e.g., `10.142.203.230:8006`) |
|`APP_SERVER_PORT` | Server port (default: 3000) |
|`RUST_LOG` | Log level: `trace`, `debug`, `info`, `warn`, `error`, `off` |
|`APP_PROXMOX_TOKEN_ID` | Token ID (format: `user@realm!token_name`)
|`APP_PROXMOX_TOKEN_SECRET` | Token secret
