#!/bin/bash

# Minimal Browser Launcher
# A lightweight, privacy-focused browser in Rust
# 
# Usage:
#   ./monivo.sh          - Launch the browser (builds only if source changed)
#   ./monivo.sh --clean  - Clean build artifacts and rebuild

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$SCRIPT_DIR"

echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo "  MINIMAL BROWSER - Privacy First"
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌  Rust toolchain not found."
    echo ""
    echo "Install Rust from: https://rustup.rs/"
    echo ""
    exit 1
fi

echo "✓  Rust toolchain found"
echo ""

# Check for required system dependencies on Linux
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Checking system dependencies..."
    
    MISSING_DEPS=()
    
    # Try to update PKG_CONFIG_PATH for better detection
    export PKG_CONFIG_PATH="/usr/lib/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
    
    # Check for WebKit2GTK (most critical - if present, GTK is likely present)
    if ! pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
        MISSING_DEPS+=("webkit2gtk-4.1")
    else
        echo "✓  WebKit2GTK found"
    fi
    
    # Check for libsoup-2.4 (required by wry)
    if ! pkg-config --exists libsoup-2.4 2>/dev/null; then
        MISSING_DEPS+=("libsoup-2.4")
    else
        echo "✓  libsoup2 found"
    fi
    
    # Check for GTK3 (if webkit2gtk is present, GTK is likely present too)
    if ! pkg-config --exists gtk-3.0 2>/dev/null; then
        # If webkit is found, GTK is probably installed but pkg-config can't find gtk-3.0.pc
        # This is a known issue, so we'll just warn instead of fail
        if pkg-config --exists webkit2gtk-4.1 2>/dev/null; then
            echo "✓  GTK3 (detected via webkit2gtk)"
        else
            MISSING_DEPS+=("gtk-3.0")
        fi
    else
        echo "✓  GTK3 found"
    fi
    
    if [ ${#MISSING_DEPS[@]} -gt 0 ]; then
        echo ""
        echo "❌ Missing system dependencies: ${MISSING_DEPS[@]}"
        echo ""
        echo "Install with the appropriate command for your distribution:"
        echo ""
        echo "Debian/Ubuntu:"
        echo "  sudo apt-get update && sudo apt-get install -y \\"
        echo "    libgtk-3-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \\"
        echo "    libsoup2.4-dev build-essential pkg-config"
        echo ""
        echo "Fedora:"
        echo "  sudo dnf install -y gtk3-devel webkit2gtk3-devel webkit2gtk3-jsc-devel \\"
        echo "    libsoup-devel gcc automake"
        echo ""
        echo "Arch Linux:"
        echo "  sudo pacman -S --noconfirm gtk3 webkit2gtk libsoup"
        echo ""
        exit 1
    fi
    
    echo "✓  All required dependencies found"
fi

echo ""

# Handle --clean flag
if [ "$1" = "--clean" ]; then
    echo "🧹 Cleaning build artifacts..."
    cd "$PROJECT_DIR"
    cargo clean
    echo "✓  Clean complete"
    echo ""
fi

# Check if binary exists and is up-to-date
BINARY="$PROJECT_DIR/target/release/minimal-browser"
NEEDS_BUILD=true

if [ -f "$BINARY" ]; then
    # Check if any source files are newer than the binary
    BINARY_TIME=$(stat -f%m "$BINARY" 2>/dev/null || stat -c%Y "$BINARY" 2>/dev/null)
    
    # Check modification times of key files
    SRC_TIME=$(stat -f%m "$PROJECT_DIR/src/main.rs" 2>/dev/null || stat -c%Y "$PROJECT_DIR/src/main.rs" 2>/dev/null)
    CARGO_TIME=$(stat -f%m "$PROJECT_DIR/Cargo.toml" 2>/dev/null || stat -c%Y "$PROJECT_DIR/Cargo.toml" 2>/dev/null)
    HTML_TIME=$(stat -f%m "$PROJECT_DIR/assets/index.html" 2>/dev/null || stat -c%Y "$PROJECT_DIR/assets/index.html" 2>/dev/null)
    
    # If binary is newer than all source files, skip rebuild
    if [ "$BINARY_TIME" -gt "$SRC_TIME" ] && [ "$BINARY_TIME" -gt "$CARGO_TIME" ] && [ "$BINARY_TIME" -gt "$HTML_TIME" ]; then
        NEEDS_BUILD=false
        echo "✓  Binary is up-to-date, launching..."
    fi
fi

if [ "$NEEDS_BUILD" = true ]; then
    echo "Building Minimal Browser (release build)..."
    echo ""

    # Set environment variables to help locate system libraries
    export PKG_CONFIG_PATH="/usr/lib/x86_64-linux-gnu/pkgconfig:/usr/lib/pkgconfig:/usr/local/lib/pkgconfig:$PKG_CONFIG_PATH"
    export LD_LIBRARY_PATH="/usr/lib/x86_64-linux-gnu:/usr/lib:$LD_LIBRARY_PATH"

    # Build the project
    cd "$PROJECT_DIR"
    cargo build --release

    echo ""
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "  Build Complete!"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo ""
fi

echo "🚀  Launching Minimal Browser..."
echo ""

# Run the browser
exec "$PROJECT_DIR/target/release/minimal-browser"
