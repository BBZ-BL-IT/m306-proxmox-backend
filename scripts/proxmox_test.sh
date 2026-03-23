#!/bin/bash

################################################################################
# Proxmox API Testing Script - All-in-One
# 
# Purpose: Test Proxmox API connectivity using curl with environment config
# 
# Environment Variables Required (from .env):
#   APP_PROXMOX_URL        - Proxmox server URL (e.g., 10.142.203.230:8006)
#   APP_PROXMOX_TOKEN_ID   - Token ID (e.g., testuser@pam!test_token)
#   APP_PROXMOX_TOKEN_SECRET - Token secret
#
# Usage:
#   ./proxmox_test.sh [command]
#
# Commands:
#   auth                  - Test authentication/version endpoint
#   nodes                 - List all nodes
#   vms                   - List all VMs
#   storage               - List storage resources
#   diagnose              - Run full diagnostics
#   help                  - Show this help message
#
# Examples:
#   ./proxmox_test.sh auth
#   ./proxmox_test.sh diagnose
#   ./proxmox_test.sh all
#
# Author: OpenCode
# Date: March 2026
################################################################################

set -euo pipefail

# ============================================================================
# CONFIGURATION & SETUP
# ============================================================================

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Script paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ENV_FILE="${PROJECT_ROOT}/.env"

# Default values
TIMEOUT=10
CURL_OPTS="-s -k"

# ============================================================================
# HELPER FUNCTIONS
# ============================================================================

# Load environment variables from .env
load_env() {
    if [[ -f "$ENV_FILE" ]]; then
        echo -e "${BLUE}ℹ Loading configuration from: $ENV_FILE${NC}"
        set -a
        source "$ENV_FILE"
        set +a
    else
        echo -e "${YELLOW}⚠ .env file not found at: $ENV_FILE${NC}"
        return 1
    fi
}

# Validate required environment variables
validate_env() {
    local missing=()
    
    [[ -z "${APP_PROXMOX_URL:-}" ]] && missing+=("APP_PROXMOX_URL")
    [[ -z "${APP_PROXMOX_TOKEN_ID:-}" ]] && missing+=("APP_PROXMOX_TOKEN_ID")
    [[ -z "${APP_PROXMOX_TOKEN_SECRET:-}" ]] && missing+=("APP_PROXMOX_TOKEN_SECRET")
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        echo -e "${RED}✗ Missing environment variables: ${missing[*]}${NC}"
        return 1
    fi
    return 0
}

# Print header with current configuration
print_header() {
    echo
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}Proxmox API Test${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "URL:       ${BLUE}${APP_PROXMOX_URL}${NC}"
    echo -e "Token:     ${BLUE}${APP_PROXMOX_TOKEN_ID}${NC}"
    echo -e "Status:    ${BLUE}Ready${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo
}

# Make API call with curl
api_call() {
    local method="${1:-GET}"
    local endpoint="$2"
    local description="${3:-}"
    
    local url="https://${APP_PROXMOX_URL}/api2/json${endpoint}"
    local auth="Authorization: PVEAPIToken=${APP_PROXMOX_TOKEN_ID}:${APP_PROXMOX_TOKEN_SECRET}"
    
    echo -e "${YELLOW}Testing: ${description}${NC}"
    echo -e "${BLUE}Request: ${method} ${endpoint}${NC}"
    echo
    
    local response=$(timeout $TIMEOUT curl $CURL_OPTS -X "$method" -H "$auth" "$url" 2>&1 || echo "ERROR")
    
    echo -e "${BLUE}Response:${NC}"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
    echo
}

# ============================================================================
# DIAGNOSTIC FUNCTIONS
# ============================================================================

# Check environment variables
check_env() {
    echo -e "${YELLOW}Checking Environment Variables${NC}"
    echo -e "${BLUE}────────────────────────────────────────────────────────────${NC}"
    
    if [[ -z "${APP_PROXMOX_URL:-}" ]]; then
        echo -e "${RED}✗ APP_PROXMOX_URL not set${NC}"
    else
        echo -e "${GREEN}✓ APP_PROXMOX_URL: ${BLUE}${APP_PROXMOX_URL}${NC}"
    fi
    
    if [[ -z "${APP_PROXMOX_TOKEN_ID:-}" ]]; then
        echo -e "${RED}✗ APP_PROXMOX_TOKEN_ID not set${NC}"
    else
        echo -e "${GREEN}✓ APP_PROXMOX_TOKEN_ID: ${BLUE}${APP_PROXMOX_TOKEN_ID}${NC}"
    fi
    
    if [[ -z "${APP_PROXMOX_TOKEN_SECRET:-}" ]]; then
        echo -e "${RED}✗ APP_PROXMOX_TOKEN_SECRET not set${NC}"
    else
        echo -e "${GREEN}✓ APP_PROXMOX_TOKEN_SECRET: ${BLUE}[REDACTED]${NC}"
    fi
    echo
}

# Check required tools
check_tools() {
    echo -e "${YELLOW}Checking Required Tools${NC}"
    echo -e "${BLUE}────────────────────────────────────────────────────────────${NC}"
    
    if command -v curl &> /dev/null; then
        echo -e "${GREEN}✓ curl: $(curl --version | head -n1)${NC}"
    else
        echo -e "${RED}✗ curl not found${NC}"
        return 1
    fi
    
    if command -v jq &> /dev/null; then
        echo -e "${GREEN}✓ jq: $(jq --version)${NC}"
    else
        echo -e "${YELLOW}⚠ jq not found (optional for JSON formatting)${NC}"
    fi
    echo
}

# Check network connectivity
check_network() {
    echo -e "${YELLOW}Checking Network Connectivity${NC}"
    echo -e "${BLUE}────────────────────────────────────────────────────────────${NC}"
    
    local host="${APP_PROXMOX_URL%:*}"
    local port="${APP_PROXMOX_URL##*:}"
    
    echo "Host: $host"
    echo "Port: $port"
    echo
    
    if ping -c 1 -W 2 "$host" &> /dev/null; then
        echo -e "${GREEN}✓ Host is reachable (ping)${NC}"
    else
        echo -e "${YELLOW}⚠ Host may not be reachable (firewall may block ping)${NC}"
    fi
    
    if timeout 3 bash -c "cat < /dev/null > /dev/tcp/$host/$port" 2>/dev/null; then
        echo -e "${GREEN}✓ Port $port is open${NC}"
    else
        echo -e "${RED}✗ Cannot connect to port $port${NC}"
        return 1
    fi
    echo
}

# Test API connectivity
check_api() {
    echo -e "${YELLOW}Testing API Connectivity${NC}"
    echo -e "${BLUE}────────────────────────────────────────────────────────────${NC}"
    
    local url="https://${APP_PROXMOX_URL}/api2/json/version"
    local auth="Authorization: PVEAPIToken=${APP_PROXMOX_TOKEN_ID}:${APP_PROXMOX_TOKEN_SECRET}"
    
    if timeout 5 curl $CURL_OPTS -H "$auth" "$url" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ API is reachable and responding${NC}"
    else
        echo -e "${RED}✗ Cannot reach API${NC}"
        return 1
    fi
    echo
}

# ============================================================================
# TEST COMMANDS
# ============================================================================

# Test authentication and get version
test_auth() {
    api_call "GET" "/api2/json/version" "Proxmox Version"
}

# List all nodes in cluster
test_nodes() {
    api_call "GET" "/api2/json/nodes" "List Nodes"
}

# List all VMs
test_vms() {
    api_call "GET" "/api2/json/cluster/resources?type=vm" "List VMs"
}

# List storage resources
test_storage() {
    api_call "GET" "/api2/json/storage" "List Storage"
}

# Run full diagnostics
run_diagnostics() {
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo -e "${BLUE}Proxmox Connectivity Diagnostics${NC}"
    echo -e "${BLUE}═══════════════════════════════════════════════════════════${NC}"
    echo
    
    check_env
    check_tools
    check_network || {
        echo -e "${RED}Network check failed${NC}"
        return 1
    }
    check_api || {
        echo -e "${RED}API check failed${NC}"
        return 1
    }
    
    echo -e "${GREEN}✓ All diagnostics passed${NC}"
    echo
}

# Show help
show_help() {
    cat << 'EOF'
Proxmox API Testing Script - All-in-One

USAGE:
  ./proxmox_test.sh [command]

COMMANDS:
  auth       Test authentication and get version
  nodes      List all nodes in cluster
  vms        List all VMs
  storage    List storage resources
  diagnose   Run full connectivity diagnostics
  help       Show this help message

EXAMPLES:
  ./proxmox_test.sh diagnose    # Check all connectivity
  ./proxmox_test.sh auth        # Test authentication
  ./proxmox_test.sh vms         # List VMs

ENVIRONMENT:
  APP_PROXMOX_URL        Proxmox server URL
  APP_PROXMOX_TOKEN_ID   API token identifier
  APP_PROXMOX_TOKEN_SECRET API token secret

REQUIREMENTS:
  - curl (required)
  - jq (optional, for JSON formatting)

FEATURES:
  ✓ Automatic .env loading
  ✓ Environment validation
  ✓ Colored output
  ✓ Error handling
  ✓ Connectivity diagnostics

EOF
}

# ============================================================================
# MANUAL CURL EXAMPLES (For reference - uncomment to use)
# ============================================================================

# To use these manually, copy and paste into terminal:

# Get API version (no auth needed)
# curl -s -k https://10.142.203.230:8006/api2/json/version | jq '.'

# List nodes with authentication
# curl -s -k -H "Authorization: PVEAPIToken=testuser@pam!test_token:SECRET" \
#   https://10.142.203.230:8006/api2/json/nodes | jq '.'

# List VMs
# curl -s -k -H "Authorization: PVEAPIToken=testuser@pam!test_token:SECRET" \
#   'https://10.142.203.230:8006/api2/json/cluster/resources?type=vm' | jq '.'

# Start a VM (POST request - VMID = 100)
# curl -s -k -X POST \
#   -H "Authorization: PVEAPIToken=testuser@pam!test_token:SECRET" \
#   https://10.142.203.230:8006/api2/json/nodes/pve/qemu/100/status/start | jq '.'

# Stop a VM gracefully
# curl -s -k -X POST \
#   -H "Authorization: PVEAPIToken=testuser@pam!test_token:SECRET" \
#   https://10.142.203.230:8006/api2/json/nodes/pve/qemu/100/status/shutdown | jq '.'

# List storage
# curl -s -k -H "Authorization: PVEAPIToken=testuser@pam!test_token:SECRET" \
#   https://10.142.203.230:8006/api2/json/storage | jq '.'

# ============================================================================
# MAIN EXECUTION
# ============================================================================

main() {
    local command="${1:-help}"
    
    # Load environment
    if ! load_env; then
        echo -e "${RED}Failed to load configuration${NC}"
        exit 1
    fi
    
    # Validate environment
    if ! validate_env; then
        echo -e "${RED}Configuration validation failed${NC}"
        echo "Please ensure .env file has these variables:"
        echo "  APP_PROXMOX_URL"
        echo "  APP_PROXMOX_TOKEN_ID"
        echo "  APP_PROXMOX_TOKEN_SECRET"
        exit 1
    fi
    
    # Execute command
    case "$command" in
        auth)
            print_header
            test_auth
            ;;
        nodes)
            print_header
            test_nodes
            ;;
        vms)
            print_header
            test_vms
            ;;
        storage)
            print_header
            test_storage
            ;;
        diagnose)
            run_diagnostics
            ;;
        help|-h|--help)
            show_help
            ;;
        *)
            echo -e "${RED}Unknown command: $command${NC}"
            echo
            show_help
            exit 1
            ;;
    esac
    
    echo -e "${GREEN}✓ Done${NC}"
}

# Error handling
trap 'echo -e "${RED}✗ Error on line $LINENO${NC}"; exit 1' ERR

# Run main with all arguments
main "$@"
