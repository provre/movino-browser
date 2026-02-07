# Quick Start Guide - Minimal Browser

## One-Command Start (Linux/macOS)

```bash
cd /path/to/minimal-browser && ./minimal-browser.sh
```

## Step-by-Step Setup

### 1. Install Rust (if not already installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Install System Dependencies

**Debian/Ubuntu:**
```bash
sudo apt-get update && sudo apt-get install -y \
    libgtk-3-dev libwebkit2gtk-4.1-dev libjavascriptcoregtk-4.1-dev \
    libsoup2.4-dev build-essential pkg-config
```

**Fedora:**
```bash
sudo dnf install -y gtk3-devel webkit2gtk3-devel webkit2gtk3-jsc-devel \
    libsoup-devel gcc automake
```

**Arch Linux:**
```bash
sudo pacman -S --noconfirm gtk3 webkit2gtk libsoup
```

**macOS:**
```bash
brew install webkit2gtk libsoup
```

### 3. Build and Run

```bash
cd minimal-browser
./minimal-browser.sh
```

Or manually:
```bash
cargo build --release
./target/release/minimal-browser
```

## What You'll See

- Clean, minimalist interface with black text on white background
- Search box at the top
- Four service buttons: Proton Mail, Proton Pass, Proton Drive, Mullvad VPN
- Footer showing "Privacy First • No History • No Tracking"

## How to Use

1. **Search the Web**: Type in the search box and press Enter. Queries search DuckDuckGo privately.
2. **Visit Websites**: Enter a URL (e.g., `example.com`) and press Enter.
3. **Quick Access**: Click the service buttons for instant access to privacy tools.
4. **Close Safely**: Close the window normally. No data is saved.

## File Structure

```
minimal-browser/
├── Cargo.toml              ← Rust dependencies & build config
├── README.md               ← Full documentation
├── QUICKSTART.md           ← This file
├── minimal-browser.sh      ← Launch script
├── src/
│   └── main.rs             ← Browser application code
└── assets/
    └── index.html          ← Homepage design & search functionality
```

## Customization

### Change the default search engine:
Edit `assets/index.html`, find this line:
```javascript
const searchUrl = 'https://duckduckgo.com/?q=' + encodeURIComponent(query);
```
Replace `https://duckduckgo.com` with your preferred search engine.

### Add/remove service buttons:
Edit `assets/index.html`, find the `<div class="services">` section and modify the links.

### Change the homepage appearance:
Edit the CSS in `assets/index.html` under `<style>`.

## Keyboard Shortcuts

- **Ctrl+Q**: Quit (standard window manager)
- **Enter**: Search/Navigate
- **Tab**: Focus search box (usually)
- **Back/Forward**: Browser back/forward (depends on your window manager)

## Performance Tips

- First build takes 2-5 minutes - subsequent builds are faster
- Binary size is ~50-80MB (normal for WebKit)
- Memory usage ~80-120MB (normal for Chromium/WebKit)
- Startup time 1-2 seconds

## Troubleshooting

| Problem | Solution |
|---------|----------|
| "command not found: cargo" | Install Rust: https://rustup.rs/ |
| "WebKit not found" | Install WebKit dev files (see System Dependencies above) |
| Slow build | First builds are slow; incremental builds are faster |
| Won't start | Verify dependencies with `pkg-config --list-all \| grep webkit` |

## Next Steps

1. Run `./minimal-browser.sh`
2. Enjoy private, fast, distraction-free browsing
3. Read [README.md](README.md) for detailed documentation
4. Customize as needed

---

**Privacy is a right. Simplicity is a feature. Speed is essential.**
