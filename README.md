# Monivo - Minimal Browser

A lightweight, privacy-focused web browser written in Rust. Emphasizes stability, speed, and user privacy without unnecessary features or data collection.

## ✨ Features

- **Minimalist Design**: Clean, black and white interface with minimal distractions
- **Privacy First**: No history, cookies, cache, or user data storage
- **Dark Theme**: Persistent dark mode toggle that works on any website
- **Persistent Navigation**: Always-visible topbar with home button, search box, and dark mode toggle
- **Fast Performance**: Compiled to native machine code with aggressive optimizations (~841 KB binary)
- **Default Search**: Integrated with DuckDuckGo for private, unrestricted searches
- **Homepage Shortcut**: Press `Ctrl+H` anywhere to return to homepage instantly
- **URL Bar**: Search box displays current page URL (e.g., `monivo:about` on homepage)
- **Quick Access**: One-click buttons for:
  - Proton Mail (secure email)
  - Proton Pass (password manager)
  - Proton Drive (secure storage)
  - Mullvad VPN (anonymous VPN)
- **No Bloat**: Removed bookmarks, extensions, and other unnecessary features
- **Memory Safe**: Leverages Rust's memory safety guarantees

## ⚙️ System Requirements

| Item | Requirement |
|------|-------------|
| **OS** | Linux ✅ (macOS untested, Windows not supported yet) |
| **RAM** | 512MB minimum (1GB recommended) |
| **Disk** | ~1GB for build (binary alone is only 841 KB) |
| **Rust** | 1.70+ from [rustup.rs](https://rustup.rs/) |

### Linux Dependencies

#### Debian/Ubuntu
```bash
sudo apt-get update && sudo apt-get install -y \
    libgtk-3-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
    libsoup2.4-dev build-essential pkg-config
```

#### Fedora
```bash
sudo dnf install -y gtk3-devel webkit2gtk3-devel webkit2gtk3-jsc-devel \
    libsoup-devel gcc automake
```

#### Arch Linux
```bash
sudo pacman -S --noconfirm gtk3 webkit2gtk libsoup
```

## 🚀 Quick Start

### Using the Launcher Script (Recommended)

```bash
cd /path/to/minimal-browser
./monivo.sh
```

The script automatically:
- Checks for dependencies
- Builds only if source code changed (smart incremental builds)
- Launches the browser

### Other Ways to Run

**Manual build & run:**
```bash
cargo build --release
./target/release/minimal-browser
```

**Copy binary standalone:**
```bash
./copy-binary.sh ~
~/minimal-browser
```

**Clean build (remove all artifacts):**
```bash
./monivo.sh --clean
```

## 💻 Usage

| Action | How |
|--------|-----|
| **Search** | Type in topbar search box, press Enter |
| **Navigate to URL** | Paste or type URL in topbar search box, press Enter |
| **Return Home** | Click **H** button or press **Ctrl+H** |
| **Dark Mode** | Click **Dark** button in topbar (persists across sessions) |
| **Access Services** | Click Proton Mail/Pass/Drive or Mullvad VPN buttons |

### Navigation Examples
- Type `github.com` → Opens https://github.com
- Type `rust` → Searches DuckDuckGo for "rust"
- Press `Ctrl+H` from any page → Returns to homepage instantly
- Click `H` button → Same as Ctrl+H

## 📁 Project Structure

```
minimal-browser/
├── src/
│   └── main.rs              # Rust browser engine + topbar injection
├── assets/
│   └── index.html           # Homepage with services + dark theme
├── Cargo.toml               # Dependencies & build config
├── monivo.sh                # Smart launcher (checks timestamps)
├── copy-binary.sh           # Extract standalone binary
├── .cargo/config.toml       # Cargo optimization settings
└── README.md                # This file
```

## 🔧 How It Works

### Architecture

1. **Rust Core** (`src/main.rs`)
   - WebView wrapper using `wry` framework
   - Injects topbar + navigation into all pages via JavaScript
   - Custom `minimal://home` protocol for homepage restoration
   - Keyboard listener for Ctrl+H shortcut

2. **Homepage** (`assets/index.html`)
   - Embedded in binary via `include_str!()`
   - Dark theme CSS rules for `body.dark` class
   - DuckDuckGo integration
   - Service buttons and footer

3. **Topbar Injection**
   - Injected on every page load
   - Shows `monivo:about` on homepage, full URL on other pages
   - Features: Home button (H), Search box, Dark mode toggle
   - Fixed positioning (z-index: 999999)
   - Persists dark theme preference via localStorage

4. **Build System**
   - Smart launcher checks file modification times
   - Only rebuilds if `src/main.rs`, `Cargo.toml`, or `assets/index.html` changed
   - Release build: aggressive optimizations (LTO, codegen-units=1, stripping)

## 📊 Performance

| Metric | Value |
|--------|-------|
| Binary Size | ~841 KB (highly optimized) |
| Build Artifacts | ~1 GB (deps, cache, etc.) |
| Startup Time | 1-2 seconds |
| Memory Usage | 80-120 MB baseline |
| First Build | 2-5 minutes |
| Incremental Build | 10-30 seconds |

## 🔐 Security & Privacy

- ✅ **No Data Storage**: Zero history, cookies, or cache persistence
- ✅ **No Telemetry**: Zero tracking or analytics
- ✅ **Private Search**: DuckDuckGo integration (all queries anonymous)
- ✅ **Memory Safe**: Rust eliminates buffer overflows, use-after-free bugs
- ✅ **Minimal Surface**: Only 3 dependencies (wry, tokio, serde)
- ✅ **Open Source**: Full transparency, no hidden code

## 🛠️ Development

### Edit the Homepage
```bash
# Modify assets/index.html to change:
# - Search engine URLs
# - Service buttons
# - Colors and dark theme CSS
# - JavaScript handlers
```

### Rebuild After Changes
```bash
./monivo.sh
# Will auto-detect changes and rebuild
```

### Enable Developer Tools (for debugging)
Edit `src/main.rs` and change `.with_devtools(false)` to `.with_devtools(true)`, then rebuild.

## ⚠️ Limitations & Known Issues

| Issue | Status | Notes |
|-------|--------|-------|
| Windows Support | ❌ Not supported | WebKit2GTK unavailable on Windows |
| macOS | ⚠️ Untested | Should work, but not officially tested |
| Browser Extensions | ❌ Not available | By design (privacy + minimalism) |
| Bookmarks | ❌ Not available | By design |
| History | ❌ Not stored | By design (privacy feature) |
| Multiple Tabs | ❌ Not available | Use multiple windows instead |

## 🐛 Troubleshooting

### Build fails: "webkit2gtk not found"
**Solution**: Install WebKit dev packages (see Linux Dependencies above)

### Binary won't run on different Linux distro
**Solution**: Rebuild on target system using `./monivo.sh`

### Dark theme doesn't persist after restart
**Solution**: Dark mode preference is saved in localStorage; it persists only within the session

### High startup time
**Solution**: This is normal for Rust WebView apps. macOS is typically slower than Linux.

## 📦 Minimal Dependencies

```toml
wry = "0.24"           # WebView framework
tokio = "1.35"         # Async runtime
serde = "1.0"          # JSON serialization
serde_json = "1.0"     # JSON support
```

That's it! No bloat, no unnecessary crates.

## 💡 Philosophy

Monivo is built on five core principles:

1. **Privacy**: Zero data collection, period.
2. **Simplicity**: Only essential browser features.
3. **Speed**: Optimized Rust code, minimal overhead.
4. **Security**: Memory-safe code by design.
5. **Minimalism**: Clean, distraction-free interface.

## 📝 License

Free to use, modify, and distribute for personal use.

---

**Built with Rust 🦀 for Privacy, Speed, and Security**
