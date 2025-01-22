#!/bin/bash

# Utils
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m'

error() {
    echo -e "${RED}Error: $1${NC}" >&2
    exit 1
}

success() {
    echo -e "${GREEN}$1${NC}"
}

# 1. Verify requirements more efficiently
check_requirements() {
    local missing=()
    for cmd in jq curl; do
        if ! command -v "$cmd" >/dev/null; then
            missing+=("$cmd")
        fi
    done

    if [ ${#missing[@]} -eq 0 ]; then
        success "\u2705 jq and curl exist"
    else
        error "Missing required tools: ${missing[*]}"
    fi
}

# 2. JSON download and parsing
ZIX_JSON_URL="https://raw.githubusercontent.com/zix-rs/zix/refs/heads/main/zix.json"
download_and_parse() {
    ZIX_JSON=$(curl -s --connect-timeout 10 --max-time 30 "$ZIX_JSON_URL")

    if [ -z "$ZIX_JSON" ]; then
        error "Could not download JSON file from $ZIX_JSON_URL"
    fi

    read -r LTS_VERSION INSTALLER_URL INSTALLER_NAME << EOF
    $(echo "$ZIX_JSON" | jq -r '[.dist.latest, .versions[.dist.latest].url, .bin] | @tsv')
EOF

    if [ -z "$LTS_VERSION" ] || [ -z "$INSTALLER_URL" ] || [ -z "$INSTALLER_NAME" ]; then
        error "Error parsing JSON. Please verify the structure at $ZIX_JSON_URL"
    fi
}

# 3. installation
ZIX_DIR="$HOME/.zix"
install_binary() {
    local ZIX_BIN="$ZIX_DIR/$INSTALLER_NAME"

    mkdir -p "$ZIX_DIR" || error "Could not create directory $ZIX_DIR"

    echo "Downloading $INSTALLER_NAME version $LTS_VERSION..."
    if ! curl -L --connect-timeout 10 --max-time 300 --progress-bar -o "$ZIX_BIN" "$INSTALLER_URL"; then
        error "Error downloading $INSTALLER_NAME from $INSTALLER_URL"
    fi

    chmod +x "$ZIX_BIN" || error "Could not make $ZIX_BIN executable"
}

# 4. PATH configuration
configure_path() {
    if ! echo "$PATH" | grep -q "$ZIX_DIR"; then
        echo "Adding $ZIX_DIR to PATH..."
        export PATH="$ZIX_DIR:$PATH"

        # Use a single loop for RC files
        for rc in "$HOME/.bashrc" "$HOME/.zshrc"; do
            if [ -f "$rc" ] && ! grep -q "$ZIX_DIR" "$rc"; then
                echo "export PATH=\"$ZIX_DIR:\$PATH\"" >> "$rc"
            fi
        done
    fi
}

# 5. verification
verify_installation() {
    if command -v "$INSTALLER_NAME" >/dev/null; then
        # Try to execute the command with --version or -v to verify it works
        if "$INSTALLER_NAME" --version >/dev/null 2>&1 || "$INSTALLER_NAME" -v >/dev/null 2>&1; then
            success "$INSTALLER_NAME is installed and working correctly"
        else
            error "$INSTALLER_NAME is installed but might have issues"
        fi
    else
        error "$INSTALLER_NAME could not be verified"
    fi
}

# main
main() {
    check_requirements
    download_and_parse
    install_binary
    configure_path
    verify_installation
}

main
