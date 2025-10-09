# DroidTUI 🤖

[![Crates.io](https://img.shields.io/crates/v/droidtui.svg)](https://crates.io/crates/droidtui)
![Version](https://img.shields.io/badge/version-0.2.8-blue.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Release](https://github.com/sorinirimies/droidtui/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/droidtui/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/droidtui/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/droidtui/actions/workflows/ci.yml)

A beautiful Terminal User Interface (TUI) for Android development, providing an intuitive interface for ADB (Android Debug Bridge) commands with stunning visual effects and animations powered by TachyonFX.

## Features ✨

- **🎨 Beautiful UI**: Clean, modern terminal interface with consistent Android green theming
- **🌟 TachyonFX Animations**: Dramatic reveal animations and gradient effects
- **📱 ADB Command Center**: 13 functional ADB command categories with real-time execution
- **📜 Scrollable Results**: Navigate through long command output with visual scroll indicators
- **🎭 Clean Design**: Static Android green selections and professional layout borders
- **⌨️ Keyboard Navigation**: Intuitive vim-like navigation (j/k) and arrow keys
- **🚀 Fast & Responsive**: Built with Rust and Ratatui for optimal performance
- **🔧 Real Command Execution**: Actual ADB command execution with formatted output

## Installation 🔧

### Prerequisites

- Android SDK with ADB in your PATH

### Install from crates.io

```bash
cargo install droidtui
```

### Install from Source

```bash
git clone https://github.com/sorinirimies/droidtui.git
cd droidtui
cargo install --path .
```

### Run

```bash
droidtui
```

Or with cargo:

```bash
cargo run
```

## Usage 🎮

### Startup Screen

When you launch DroidTUI, you'll see a stunning startup screen with:
- TachyonFX-powered reveal animation with gradient sweep effects
- Dynamic wave animations and center-out reveal patterns
- Progressive content loading with smooth fade-in transitions
- Press any key to continue to the main menu

### Main Menu

The main interface provides access to functional ADB commands with expandable sub-options:

- **📱 List Devices** ▶ - Show all connected Android devices with detailed info
  - Basic device list, detailed info, serial numbers only
- **📋 List Packages** ▶ - List all installed packages with file paths
  - All packages, with paths, user packages only, system packages only  
- **🔋 Battery Info** ▶ - Display detailed battery information and status
  - Full battery status, battery level only, charging status
- **💾 Memory Usage** ▶ - Show comprehensive memory usage statistics
  - System memory, available memory, top memory apps
- **📊 CPU Info** ▶ - Display CPU information and specifications
  - CPU details, current usage, load average
- **🔗 Network Info** ▶ - Show network connectivity and configuration
  - Connectivity status, WiFi info, IP configuration
- **📱 Device Properties** ▶ - Get all device system properties
  - All properties, device model, Android version
- **🎯 Running Processes** ▶ - List all currently running processes
  - All processes, top processes, user processes only
- **📊 System Services** ▶ - List all system services and their status
  - All services, running services, app services
- **📷 Screenshot** ▶ - Take and save device screenshots
  - Take & save locally, save to device, view screenshot paths
- **🔄 Reboot Device** ▶ - Reboot the connected device
  - Normal reboot, fast reboot (bootloader), recovery mode
- **📜 System Log** ▶ - View recent system logs (last 100 lines)
  - Recent logs, errors only, warnings & errors, clear logs
- **🔍 ADB Version** ▶ - Display ADB version information
  - ADB version, ADB help, ADB installation path

### Navigation

| Key | Action |
|-----|--------|
| `↑` / `k` | Move up in menu / Scroll up in results |
| `↓` / `j` | Move down in menu / Scroll down in results |
| `Enter` / `→` | Enter sub-options (main menu) |
| `Enter` | Execute selected command (child menu) |
| `←` / `Backspace` | Return to main menu (from child menu) |
| `Page Up` | Fast scroll up in results (10 lines) |
| `Page Down` | Fast scroll down in results (10 lines) |
| `Home` | Jump to beginning of results |
| `End` | Jump to end of results |
| `q` / `Esc` | Quit application / Return from results |
| `Ctrl+C` | Force quit |

### Interface Layout

The interface adapts based on navigation mode:

**Main Menu Mode:**
1. **Left Panel (60%)**: Main ADB commands with ▶ indicators for expandable items (Android green border)
2. **Right Panel (40%)**: Description of selected command with usage hints (dark gray border)
3. **Footer**: Navigation help and keyboard shortcuts

**Child Menu Mode:**
1. **Left Panel (40%)**: Category overview with current selection highlighted (dark gray border) 
2. **Right Panel (60%)**: Expanded sub-options for the selected category (Android green border)
3. **Footer**: Updated navigation help for child menu mode

## Dependencies 📦

- **ratatui**: Terminal user interface library
- **crossterm**: Cross-platform terminal manipulation
- **tokio**: Async runtime
- **tachyonfx**: Visual effects and animations
- **color-eyre**: Beautiful error handling
- **futures**: Async utilities

## Technical Details 🔧

### Architecture

- **Event-driven**: Async event system for responsive UI
- **State Management**: Clean separation of startup, menu, execution, and result states
- **Effects System**: Startup animations with consistent Android green theming
- **Command Execution**: Real ADB command execution with comprehensive error handling  
- **Menu System**: Dual-level navigation with expandable command categories

### Project Structure

```
src/
├── main.rs          # Application entry point
├── app.rs           # Main application logic and state management
├── event.rs         # Event handling system
├── ui.rs            # User interface rendering
├── menu.rs          # Menu system and ADB commands
└── effects.rs       # Visual effects and animations
```

## Development 🛠️

### Adding New Commands

To add a new ADB command to the menu:

1. Edit `src/menu.rs`
2. Add a new `MenuItem` to the `items` vector in `Menu::default()`
3. Include label, description, command, and optional children

Example:
```rust
MenuItem {
    label: "🔧 Custom Command".to_string(),
    description: "Description of what this command does".to_string(),
    command: "adb shell your-complete-command-here".to_string(),
    children: vec![
        MenuChild {
            label: "🎯 Specific Option".to_string(),
            description: "Specific variation of the command".to_string(),
            command: "adb shell specific-command".to_string(),
        },
        // Add more children as needed
    ],
},
```

**For simple commands**: Leave `children` as an empty vector `vec![]`
**For expandable commands**: Add `MenuChild` items with specific variations

All commands are executed as-is, so ensure they are complete and functional.

### Customizing Effects

Visual effects can be modified in `src/effects.rs`:
- Adjust startup reveal animation timing and intensity
- Modify gradient wave effects and background colors
- Change static Android green selection color
- Change startup progress phases and content timing
- Modify layout border colors (green for active, dark gray for inactive)

## Release Automation 🚀

**📚 See [QUICK_RELEASE.md](QUICK_RELEASE.md) for a quick start guide!**

DroidTUI includes automated tools for version management and releases.

### Quick Release

Install `just` command runner:
```bash
cargo install just
```

Bump version and release in one command:
```bash
just release 0.2.5
```

### Available Commands

```bash
just              # Show all available commands
just version      # Show current version
just bump 0.2.5   # Bump version to 0.2.5
just release 0.2.5  # Full release workflow
just check-all    # Run all checks (fmt, clippy, test)
```

### Manual Version Bump

Use the provided script:
```bash
./scripts/bump_version.sh 0.2.5
```

### Automated Workflows

- **CI Workflow**: Runs tests on every push and PR
- **Release Workflow**: Builds and publishes on tag push
- **Update README**: Automatically updates version badge on release

📚 **Documentation:**
- **Quick Start**: [QUICK_RELEASE.md](QUICK_RELEASE.md) - TL;DR one-command release
- **Full Guide**: [RELEASE.md](RELEASE.md) - Detailed release process documentation

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License 📄

Copyright (c) Sorin Albu-Irimies <mihaiirimies@gmail.com>

This project is licensed under the MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)

## Acknowledgments 🙏

- [Ratatui](https://ratatui.rs) - Amazing TUI library for Rust
- [TachyonFX](https://github.com/junkdog/tachyonfx) - Visual effects library
- Android team for the awesome platform and tooling

---

**Made with ❤️ and ☕ for Android developers**