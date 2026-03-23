# Dev Environment

All Application properties in the **.env** file needs its entries to start with "APP\_"

example:

> APP_PROXMOX_URL=test

Tools from dependencies use their default configuration in **.env**

example:

> RUST_LOG=info

## .env List

- APP_PROXMOX_URL=
- APP_SERVER_PORT=
- RUST_LOG=

**APP_PROXMOX_URL** defines the URL on which to call the proxmox api.

**APP_SERVER_PORT** defines the PORT which the application will run on.

**RUST_LOG** defines the logging.
_Options_:

- RUST_LOG=trace # most verbose
- RUST_LOG=debug
- RUST_LOG=info
- RUST_LOG=warn
- RUST_LOG=error # least verbose
- RUST_LOG=off # silence everything
