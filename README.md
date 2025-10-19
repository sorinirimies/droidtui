# DroidTUI 🤖

[![Crates.io](https://img.shields.io/crates/v/droidtui.svg)](https://crates.io/crates/droidtui)
![Version](https://img.shields.io/badge/version-0.3.2-blue.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Release](https://github.com/sorinirimies/droidtui/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/droidtui/actions/workflows/release.yml)
[![CI](https://github.com/sorinirimies/droidtui/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/droidtui/actions/workflows/ci.yml)

A beautiful Terminal User Interface (TUI) for Android development, providing an intuitive interface for ADB (Android Debug Bridge) commands with stunning visual effects and animations powered by TachyonFX.

![DroidTUI Demo](examples/vhs/output/main_menu.gif)

## What's New in v0.3.0 🎉

- **🔧 Type-Safe ADB Commands**: Complete refactoring using `adb_cli` (adb_client) for robust, typed ADB operations
- **⚡ Performance Boost**: 5-10% improvement from direct ADB server communication
- **📚 Rich Examples**: New examples for streaming, device info, package manager, and more
- **🎬 VHS Demos**: 7+ animated terminal demos showcasing all features
- **✅ Comprehensive Testing**: 31 tests with 82% coverage
- **📖 Enhanced Documentation**: New refactoring guide, implementation summary, and quick reference
- **🏗️ Better Architecture**: New `AdbManager` abstraction layer for maintainable, future-proof code

See [CHANGELOG.md](CHANGELOG.md) for full release notes.

## Features ✨

- **🎨 Beautiful UI**: Clean, modern terminal interface with consistent Android green theming
- **🌟 TachyonFX Animations**: Dramatic reveal animations and gradient effects
- **📱 Type-Safe ADB Commands**: 14 functional ADB command categories with typed, error-safe execution
- **📺 Screen Streaming**: Stream your Android device screen in a separate window with real video (like scrcpy!)
- **📜 Scrollable Results**: Navigate through long command output with visual scroll indicators
- **🎭 Clean Design**: Static Android green selections and professional layout borders
- **⌨️ Keyboard Navigation**: Intuitive vim-like navigation (j/k) and arrow keys
- **🚀 Fast & Responsive**: Built with Rust and Ratatui for optimal performance
- **🔧 Real Command Execution**: Direct ADB server communication with comprehensive error handling
- **📚 Extensive Examples**: 5+ runnable examples demonstrating all features
- **🎬 VHS Demos**: Animated terminal recordings showcasing the TUI in action

## Installation 🔧

### Prerequisites

- Android SDK with ADB in your PATH
- FFmpeg (for video decoding during screen streaming)

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
- **📺 Screen Stream** ▶ - Stream device screen in separate window (like scrcpy)
  - Start screen stream, high quality stream, fast stream
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

**Note**: Screen streaming opens in a separate window. Close the window or press Q/Esc in it to stop streaming.

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

## Examples & Demos 📚

### Runnable Examples

DroidTUI includes several standalone examples demonstrating different features:

```bash
# Main menu with all features
cargo run --example main_menu

# Screen streaming
cargo run --example streaming

# Device information
cargo run --example device_info

# Package manager
cargo run --example package_manager

# All examples (run sequentially)
cargo run --example all_examples
```

See [examples/README.md](examples/README.md) for detailed information.

### VHS Terminal Demos

Animated terminal recordings showcasing DroidTUI in action:

- `main_menu.tape` - Complete main menu navigation
- `streaming.tape` - Screen streaming feature
- `device_info.tape` - Device information commands
- `package_manager.tape` - Package management
- `navigation.tape` - Keyboard navigation and scrolling
- `features_highlight.tape` - All major features
- `all_examples.tape` - Running all examples

Generate demos with [VHS](https://github.com/charmbracelet/vhs):

```bash
# Install VHS
go install github.com/charmbracelet/vhs@latest

# Generate a demo
vhs examples/vhs/main_menu.tape

# Generate all demos
./examples/vhs/generate_all.sh
```

See [examples/vhs/README.md](examples/vhs/README.md) for more information.

## Dependencies 📦

### Core Dependencies

- **ratatui**: Terminal user interface library
- **crossterm**: Cross-platform terminal manipulation
- **tokio**: Async runtime
- **tachyonfx**: Visual effects and animations
- **color-eyre**: Beautiful error handling
- **adb_client** (adb_cli): Type-safe ADB command execution
- **futures**: Async utilities
- **minifb**: Window creation for screen streaming

### Why adb_client?

The v0.3.0 refactoring replaced string-based ADB commands with the typed `adb_client` crate, providing:

- **Type Safety**: Compile-time guarantees for ADB operations
- **Error Handling**: Rich error types with detailed information
- **Performance**: Direct ADB server communication (5-10% faster)
- **Maintainability**: Clear API, easier to extend and debug
- **Future-Proof**: Support for new ADB features as they're added

See [REFACTORING.md](REFACTORING.md) for the complete migration story.

## Technical Details 🔧

### Architecture

DroidTUI follows an Elm-like architecture with clear separation of concerns:

- **Model**: Application state (`src/model.rs`)
- **View**: UI rendering (`src/view.rs`)
- **Update**: State transitions (`src/update.rs`)
- **Message**: State change events (`src/message.rs`)
- **Event System**: Async event handling (`src/event.rs`)
- **ADB Layer**: Type-safe command execution (`src/adb.rs`)
- **Effects**: Visual animations (`src/effects.rs`)
- **Streaming**: Screen streaming (`src/stream.rs`)

### ADB Command Abstraction

The new `AdbManager` provides a clean abstraction over `adb_client`:

```rust
pub enum AdbCommand {
    ListDevices,
    Shell(String),
    GetProp(String),
    GetState,
    // ... 20+ typed commands
}

pub struct AdbManager {
    // Manages ADB server connection
}

impl AdbManager {
    pub async fn execute(&mut self, command: AdbCommand) -> Result<String, AdbError>
}
```

This design:
- Encapsulates ADB complexity
- Provides type-safe commands
- Enables comprehensive error handling
- Facilitates testing and mocking
- Makes adding new commands easy

See [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for usage examples.

### Project Structure

```
droidtui/
├── src/
│   ├── main.rs          # Application entry point
│   ├── app.rs           # Main application logic
│   ├── model.rs         # Application state
│   ├── view.rs          # UI rendering
│   ├── update.rs        # State update logic
│   ├── message.rs       # Message types
│   ├── event.rs         # Event handling
│   ├── menu.rs          # Menu system
│   ├── adb.rs           # ADB command abstraction (NEW in v0.3.0)
│   ├── effects.rs       # Visual effects
│   └── stream.rs        # Screen streaming
├── examples/
│   ├── main_menu.rs     # Full menu example
│   ├── streaming.rs     # Streaming example
│   ├── device_info.rs   # Device info example
│   ├── package_manager.rs  # Package manager example
│   ├── all_examples.rs  # Run all examples
│   ├── vhs/             # VHS demo tapes
│   └── README.md        # Examples documentation
├── tests/
│   ├── integration_tests.rs  # Integration tests
│   └── adb_tests.rs     # ADB layer tests
├── docs/
│   ├── REFACTORING.md           # Refactoring story
│   ├── IMPLEMENTATION_SUMMARY.md # Implementation details
│   ├── QUICK_REFERENCE.md       # Quick API reference
│   └── CHANGELOG.md             # Version history
└── scripts/
    └── bump_version.sh  # Version management
```

## Development 🛠️

### Adding New Commands

To add a new ADB command:

1. **Define the command variant** in `src/adb.rs`:

```rust
pub enum AdbCommand {
    // ... existing commands
    YourNewCommand { param: String },
}
```

2. **Implement execution** in `AdbManager::execute`:

```rust
AdbCommand::YourNewCommand { param } => {
    // Use adb_client API
    let result = self.client.shell_command(device_serial, &format!("your command {}", param))?;
    Ok(result)
}
```

3. **Add to menu** in `src/menu.rs`:

```rust
MenuItem {
    label: "🔧 Your Command".to_string(),
    description: "What it does".to_string(),
    command: AdbCommand::YourNewCommand { param: "value".to_string() },
    children: vec![],
}
```

4. **Add tests** in `tests/adb_tests.rs`

See [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for detailed examples.

### Customizing Effects

Visual effects can be modified in `src/effects.rs`:
- Adjust startup reveal animation timing and intensity
- Modify gradient wave effects and background colors
- Change static Android green selection color
- Change startup progress phases and content timing
- Modify layout border colors (green for active, dark gray for inactive)

### Testing

Run the comprehensive test suite:

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_adb_manager

# Run integration tests
cargo test --test integration_tests

# Check code coverage (requires tarpaulin)
cargo tarpaulin --out Html
```

Current test coverage: **82%** with 31 passing tests.

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
just release 0.3.0
```

### Available Commands

```bash
just              # Show all available commands
just version      # Show current version
just bump 0.3.0   # Bump version to 0.3.0
just release 0.3.0  # Full release workflow
just check-all    # Run all checks (fmt, clippy, test)
```

### Manual Version Bump

Use the provided script:
```bash
./scripts/bump_version.sh 0.3.0
```

### Automated Workflows

- **CI Workflow**: Runs tests on every push and PR
- **Release Workflow**: Builds and publishes on tag push
- **Update README**: Automatically updates version badge on release

📚 **Documentation:**
- **Quick Start**: [QUICK_RELEASE.md](QUICK_RELEASE.md) - TL;DR one-command release
- **Full Guide**: [RELEASE.md](RELEASE.md) - Detailed release process documentation

## Documentation 📖

DroidTUI includes comprehensive documentation:

- **[README.md](README.md)** - This file, main overview
- **[CHANGELOG.md](CHANGELOG.md)** - Version history and release notes
- **[REFACTORING.md](REFACTORING.md)** - Complete refactoring story and rationale
- **[IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md)** - Implementation details and outcomes
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Quick API reference and examples
- **[examples/README.md](examples/README.md)** - Examples documentation
- **[examples/vhs/README.md](examples/vhs/README.md)** - VHS demos documentation
- **[QUICK_RELEASE.md](QUICK_RELEASE.md)** - Quick release guide
- **[RELEASE.md](RELEASE.md)** - Detailed release process

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Contribution Guidelines

- Follow the existing code style (run `cargo fmt`)
- Add tests for new features
- Update documentation as needed
- Use the `AdbCommand` enum for ADB operations
- Ensure all tests pass (`cargo test`)
- Check for clippy warnings (`cargo clippy`)

See [QUICK_REFERENCE.md](QUICK_REFERENCE.md) for API usage examples.

## Roadmap 🗺️

Future enhancements planned:

- [ ] APK installation via file transfer
- [ ] Multiple device selection
- [ ] Wireless ADB support
- [ ] Command batching and async execution
- [ ] Custom command presets
- [ ] Configuration file support
- [ ] Plugin system for custom commands
- [ ] Log filtering and search
- [ ] File browser for device storage
- [ ] App manager with install/uninstall

## Performance 📈

DroidTUI v0.3.0 performance improvements:

- **Startup Time**: < 100ms to main menu
- **Command Execution**: 5-10% faster than string-based commands
- **Memory Usage**: < 20MB typical
- **Screen Streaming**: 30-60 FPS depending on device
- **Responsiveness**: < 16ms UI update latency

Benchmarks run on: Ubuntu 22.04, Ryzen 7 5800X, 32GB RAM

## Troubleshooting 🔍

### Common Issues

**"No devices found"**
- Ensure ADB is in your PATH: `which adb`
- Check device connection: `adb devices`
- Enable USB debugging on device

**"FFmpeg not found" (for streaming)**
- Install FFmpeg: `sudo apt install ffmpeg` (Linux) or `brew install ffmpeg` (macOS)
- Ensure it's in your PATH: `which ffmpeg`

**Screen streaming doesn't work**
- Check FFmpeg installation
- Ensure device screen is unlocked
- Try different quality settings in menu

**Commands fail with "Connection refused"**
- Restart ADB server: `adb kill-server && adb start-server`
- Check device authorization
- Verify ADB version compatibility

For more issues, check [GitHub Issues](https://github.com/sorinirimies/droidtui/issues).

## License 📄

Copyright (c) Sorin Albu-Irimies <mihaiirimies@gmail.com>

This project is licensed under the MIT license ([LICENSE](./LICENSE) or <http://opensource.org/licenses/MIT>)

## Acknowledgments 🙏

- [Ratatui](https://ratatui.rs) - Amazing TUI library for Rust
- [TachyonFX](https://github.com/junkdog/tachyonfx) - Visual effects library
- [adb_client](https://github.com/adb-client/adb_client) - Type-safe ADB Rust client
- Android team for the awesome platform and tooling
- All contributors and users of DroidTUI

## Links 🔗

- **Crates.io**: [droidtui](https://crates.io/crates/droidtui)
- **GitHub**: [sorinirimies/droidtui](https://github.com/sorinirimies/droidtui)
- **Issues**: [Report a bug](https://github.com/sorinirimies/droidtui/issues)
- **Discussions**: [GitHub Discussions](https://github.com/sorinirimies/droidtui/discussions)

---

**Made with ❤️ and ☕ for Android developers**

*Powered by Rust 🦀 | Built with Ratatui 🐭 | Enhanced by TachyonFX ⚡*