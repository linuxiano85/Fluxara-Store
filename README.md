# Fluxara Store

**Universal Linux Package Manager with GTK4/libadwaita UI**

Fluxara Store aims to be a modern, comprehensive software center for Linux that unifies multiple package sources (Flatpak, APT, Pacman, AUR, and more) into a single, beautiful interface.

## Vision

Fluxara Store provides a unified interface for managing software across different Linux distributions, supporting multiple package formats and repositories while maintaining security and user control.

## Features

### Current (v0.1.0)
- ✅ **Multiple Package Providers**: Flatpak, APT, Pacman support
- ✅ **GTK4/libadwaita UI**: Modern, native Linux interface
- ✅ **Update Daemon**: Background update checking with optional tray icon
- ✅ **Configuration System**: Feature toggles for repositories and UI preferences
- ✅ **CLI Interface**: Command-line tool for package management
- ✅ **Driver Detection**: Hardware detection and driver suggestions (stub)
- ✅ **System Maintenance**: Cache cleanup, orphan removal, mirror speed testing (stub)
- ✅ **AppStream/ODRS Integration**: Ratings and reviews support (stub)
- ✅ **Package Conversion**: Safe package format conversion via alien
- ✅ **Source Builds**: Sandboxed builds using Podman (stub)

### Planned for 1.0 (Target: 2026-10-31)
- 🔄 **Flatpak DBus Integration**: Native Flatpak support via DBus
- 🔄 **AUR Build Support**: Build AUR packages in containers
- 🔄 **ODRS Write Support**: Submit ratings and reviews
- 🔄 **Driver Installation**: Automated driver installation
- 🔄 **Mirror Auto-Switch**: Automatic mirror selection based on speed
- 🔄 **Complete Packaging**: .deb, .rpm, Flatpak, AppImage distribution

## Architecture

Fluxara Store is built as a modular Rust workspace with the following components:

### Core Components
- **fluxara-core**: Common types, traits, and configuration management
- **fluxara-ui-gtk**: GTK4/libadwaita desktop application
- **fluxara-cli**: Command-line interface
- **fluxara-daemon**: Background update daemon with tray icon

### Package Providers
- **fluxara-provider-flatpak**: Flatpak package management
- **fluxara-provider-apt**: Debian/Ubuntu APT support
- **fluxara-provider-pacman**: Arch/Manjaro Pacman support

### Services
- **fluxara-appstream**: AppStream metadata and ODRS integration
- **fluxara-converter**: Package format conversion (via alien)
- **fluxara-builder**: Source code building in Podman sandbox
- **fluxara-drivers**: Hardware detection and driver management
- **fluxara-maintenance**: System maintenance utilities
- **fluxara-polkit-agent**: Privileged operations helper

## Installation

### From Source

#### Prerequisites
```bash
# Ubuntu/Debian
sudo apt install build-essential libgtk-4-dev libadwaita-1-dev

# Fedora
sudo dnf install gtk4-devel libadwaita-devel

# Arch/Manjaro
sudo pacman -S gtk4 libadwaita
```

#### Build
```bash
git clone https://github.com/linuxiano85/Fluxara-Store.git
cd Fluxara-Store
cargo build --release
```

#### Install
```bash
sudo cp target/release/fluxara-store /usr/local/bin/
sudo cp target/release/fluxara /usr/local/bin/
sudo cp target/release/fluxara-daemon /usr/local/bin/
```

## Configuration

Configuration is stored in `~/.config/fluxara/config.toml`:

```toml
[ui]
tray_enabled = true

[repos.flathub]
beta_enabled = true

[repos.aur]
enabled = true  # Auto-detected on Arch/Manjaro

[security]
conversion_policy = "safe"

[telemetry]
enabled = false  # Opt-in only
```

### Configuration Options

#### UI Settings
- **tray_enabled**: Show system tray icon for background updates (default: true)

#### Repository Settings
- **repos.flathub.beta_enabled**: Enable Flathub beta repository (default: true, can be disabled)
- **repos.aur.enabled**: Enable AUR support (default: auto-detected for Arch/Manjaro)

#### Security Settings
- **conversion_policy**: Package conversion safety level
  - `safe`: Block kernel, drivers, bootloader, and system packages (default)
  - `permissive`: Allow more conversions with warnings
  - `strict`: Block all conversions

#### Privacy Settings
- **telemetry.enabled**: Anonymous usage statistics (default: false, opt-in)

## Usage

### Graphical Interface
```bash
fluxara-store
```

### Command Line

Search for packages:
```bash
fluxara search firefox
```

Install a package:
```bash
fluxara install org.mozilla.firefox
```

Remove a package:
```bash
fluxara remove org.mozilla.firefox
```

Update all packages:
```bash
fluxara update
```

Update specific package:
```bash
fluxara update org.mozilla.firefox
```

List installed packages:
```bash
fluxara list
```

### Background Daemon
```bash
fluxara-daemon
```

## Distribution-Specific Features

### Manjaro vs Arch
- **On Manjaro**: Uses Manjaro's official repositories (safer, tested packages)
- **On Arch**: Uses Arch Linux repositories
- **Policy**: Does NOT add Manjaro repos on Arch for compatibility and safety

### AUR Support
- **Arch/Manjaro**: AUR enabled by default (can be disabled)
- **Other distros**: Planned via Podman containers with Arch rootfs (future milestone)

### Conversion Safety
The package converter (`alien`) is restricted to safe operations:
- ✅ **Allowed**: User applications, libraries
- ❌ **Blocked**: Kernel packages, drivers, bootloader, glibc, systemd

## Development

### Project Structure
```
Fluxara-Store/
├── crates/
│   ├── fluxara-core/          # Core types and config
│   ├── fluxara-ui-gtk/        # GTK4 UI
│   ├── fluxara-cli/           # CLI tool
│   ├── fluxara-daemon/        # Update daemon
│   ├── fluxara-provider-*/    # Package providers
│   ├── fluxara-appstream/     # AppStream/ODRS
│   ├── fluxara-converter/     # Package conversion
│   ├── fluxara-builder/       # Source building
│   ├── fluxara-drivers/       # Driver management
│   ├── fluxara-maintenance/   # System maintenance
│   └── fluxara-polkit-agent/  # Privilege helper
├── Cargo.toml                  # Workspace config
└── rust-toolchain.toml         # Rust version

```

### Building
```bash
cargo build
```

### Running Tests
```bash
cargo test
```

### Code Quality
```bash
cargo clippy
cargo fmt
```

## Roadmap

### Milestone 1: Foundation (Current - v0.1.0)
- ✅ Complete workspace scaffolding
- ✅ Basic provider implementations
- ✅ GTK4/libadwaita UI
- ✅ Configuration system
- ✅ CLI tool
- ✅ Update daemon with tray

### Milestone 2: Enhanced Providers (Q2 2026)
- 🔄 Flatpak DBus integration
- 🔄 AUR container builds
- 🔄 Snap provider
- 🔄 Enhanced AppStream parsing

### Milestone 3: Advanced Features (Q3 2026)
- 🔄 ODRS write support
- 🔄 Driver installation
- 🔄 Automatic mirror selection
- 🔄 Full telemetry implementation

### Milestone 4: Release Preparation (Q4 2026)
- 🔄 Complete packaging (.deb, .rpm, Flatpak, AppImage)
- 🔄 Documentation and localization
- 🔄 Performance optimization
- 🔄 Security audit
- 🔄 1.0 Release (Target: 2026-10-31)

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

### Development Guidelines
- Follow Rust best practices and idioms
- Maintain code coverage with tests
- Use `cargo fmt` and `cargo clippy`
- Document public APIs
- Keep changes focused and atomic

## License

Dual-licensed under MIT or Apache-2.0, at your option.

- MIT License: [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
- Apache License 2.0: [LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0

## Acknowledgments

- GTK and libadwaita teams for the beautiful UI toolkit
- Flatpak, APT, and Pacman maintainers
- GNOME Software and KDE Discover for inspiration
- Open Desktop Rating Service (ODRS) for ratings infrastructure

## Security

For security issues, please email the maintainers directly rather than using the public issue tracker.

## Support

- GitHub Issues: https://github.com/linuxiano85/Fluxara-Store/issues
- Discussions: https://github.com/linuxiano85/Fluxara-Store/discussions