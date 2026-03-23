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

# Todo

- SSL-Certificate on/off env variable
- Import of proxmox ssl certificate

## Proxmox Testing using cURL 
 
 Environment Variables Required (from .env):
-   APP_PROXMOX_URL        - Proxmox server URL (e.g., 10.142.203.230:8006)
-   APP_PROXMOX_TOKEN_ID   - Token ID (e.g., testuser@pam!test_token)
-   APP_PROXMOX_TOKEN_SECRET - Token secret

**Usage:**
   ./proxmox_test.sh [command]

**Commands:**
-   auth                  - Test authentication/version endpoint
-   nodes                 - List all nodes
-   vms                   - List all VMs
-   storage               - List storage resources
-   diagnose              - Run full diagnostics
-   help                  - Show this help message

**Examples:**
-   ./proxmox_test.sh auth
-   ./proxmox_test.sh diagnose
-   ./proxmox_test.sh all
