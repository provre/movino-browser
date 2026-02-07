# Minimal Browser

A lightweight, privacy-focused web browser written in Rust. Emphasizes stability, speed, and user privacy without unnecessary features or data collection.

## Features

- **Minimalist Design**: Clean, black and white interface with no visual distractions
- **Privacy First**: No history, cookies, cache, or user data storage
- **Fast Performance**: Compiled to native machine code with aggressive optimizations
- **Default Search**: Integrated with DuckDuckGo for private, unrestricted searches
- **Quick Access**: One-click buttons for:
  - Proton Mail
  - Proton Pass
  - Proton Drive
  - Mullvad VPN
- **No Bloat**: Removed bookmarks, extensions, and other unnecessary features
- **Memory Safe**: Leverages Rust's memory safety guarantees

## Requirements

### System Requirements
- **OS**: Linux, macOS, or Windows
- **RAM**: Minimum 512MB (1GB recommended)
- **Disk Space**: ~500MB for build artifacts

### Build Dependencies

#### Rust Toolchain
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get update && sudo apt-get install -y \
    libgtk-3-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
    libsoup2.4-dev build-essential pkg-config
```

#### Linux (Fedora)
```bash
sudo dnf install -y gtk3-devel webkit2gtk3-devel webkit2gtk3-jsc-devel \
    libsoup-devel gcc automake
```

#### Linux (Arch)
```bash
sudo pacman -S --noconfirm gtk3 webkit2gtk libsoup
```

#### macOS
```bash
brew install webkit2gtk
```

#### Windows
- Install Visual Studio Build Tools or MinGW
- Install WebKit2 runtime (included with wry on Windows)

## Installation & Compilation

### Method 1: Using the Launcher Script (Recommended)

```bash
cd /path/to/minimal-browser
./minimal-browser.sh
```

The script will:
1. Check for required dependencies
2. Build the project in release mode
3. Automatically launch the browser

### Method 2: Manual Build

```bash
cd /path/to/minimal-browser
cargo build --release
./target/release/minimal-browser
```

### Method 3: Development Mode

For development with debug symbols and faster compilation:

```bash
cargo run
```

## Usage

1. **Search**: Type your query in the search box and press Enter
   - Plain text searches use Searx Belgium
   - URLs are detected automatically (add `https://` if needed)

2. **Access Services**: Click the service buttons to access:
   - Proton Mail: Encrypted email service
   - Proton Pass: Password manager
   - Proton Drive: Secure file storage
   - Mullvad VPN: Anonymous VPN service

3. **Exit**: Close the window to exit safely
   - No data is retained after closing

## Architecture

### Core Components

- **Frontend**: Single HTML/CSS homepage with vanilla JavaScript
- **Backend**: Rust application using `wry` for WebView rendering
- **Rendering**: Native system WebKit/WebEngine (GTK on Linux, Cocoa on macOS, MSHTML on Windows)

### Minimal Dependencies

```
wry       - Cross-platform WebView library
tokio     - Async runtime
serde     - Serialization framework
```

### Build Profile

The release build is optimized for:
- Maximum performance (`opt-level = 3`)
- Link-time optimization (LTO enabled)
- Binary stripping (remove debug symbols)
- Single codegen unit (better optimization)

## Security Features

1. **No Data Persistence**: Browser history, cookies, and cache are not stored
2. **Default Search Privacy**: Searx Belgium provides anonymous search
3. **No Telemetry**: Zero tracking or data collection
4. **Memory Safety**: Rust prevents buffer overflows and use-after-free vulnerabilities
5. **Simplified Attack Surface**: Minimal code and dependencies reduce potential vulnerabilities

## Performance Characteristics

- **Startup Time**: ~1-2 seconds
- **Memory Usage**: ~80-120MB baseline
- **Binary Size**: ~50-80MB (optimized release build)
- **Rendering**: Hardware-accelerated through native WebKit

## File Structure

```
minimal-browser/
├── src/
│   └── main.rs              # Main browser application
├── assets/
│   └── index.html           # Homepage UI
├── Cargo.toml               # Rust package configuration
├── minimal-browser.sh       # Launch script
└── README.md                # This file
```

## Compilation times

First build: ~2-5 minutes (depending on system)
Incremental builds: ~10-30 seconds

## Troubleshooting

### Build fails with "WebKit not found"

**Linux**: Install the appropriate WebKit development files for your distribution (see Requirements section).

**macOS**: Run `brew install webkit2gtk`

### Application won't start

- Verify Rust is installed: `rustc --version`
- Check system dependencies are installed
- Try building manually: `cargo build --release`

### High memory usage

This is normal for web browser applications due to WebKit engine initialization. Minimal Browser uses approximately 80-120MB baseline memory.

### Slow build times

First builds are slow because of dependency compilation. Subsequent builds are faster. For development, use `cargo run` instead of building in release mode.

## Development

### Building from source (development mode)

```bash
cargo build
./target/debug/minimal-browser
```

### Enabling developer tools (for debugging)

Edit `src/main.rs` and change:
```rust
.with_dev_tools(false)
```
to:
```rust
.with_dev_tools(true)
```

Then rebuild and run.

### Modifying the homepage

Edit `assets/index.html` to customize:
- Search engine URLs
- Service buttons and links
- Visual design and styling
- JavaScript functionality

Changes to HTML don't require recompilation (if bundled resources are modified).

## Contributing & Customization

This is a personal project focused on privacy and minimalism. You can easily customize:

1. **Search Engine**: Edit the `searchUrl` in `assets/index.html`
2. **Service Buttons**: Add or remove buttons in the `<div class="services">` section
3. **Visual Design**: Modify CSS in the `<style>` section
4. **Window Size**: Change dimensions in `src/main.rs` in the `WindowBuilder`

## License

This project prioritizes user privacy and security above all else. Feel free to use, modify, and distribute for personal use.

## Philosophy

Minimal Browser is built on these principles:

1. **Privacy**: No data collection, tracking, or persistence
2. **Simplicity**: Remove everything except core browser functionality
3. **Speed**: Optimized Rust code with minimal dependencies
4. **Security**: Memory-safe code and minimal attack surface
5. **Minimalism**: Clean, distraction-free interface

## Support

For issues or questions:
1. Check this README
2. Review the troubleshooting section
3. Check system dependencies are installed
4. Verify Rust toolchain is up to date: `rustup update`

---

**Built with Rust for Speed, Privacy, and Security**
