# Quick Start Guide

This guide helps you get started with Fluxara Store development.

## Prerequisites

### Required
- Rust (stable channel, automatically managed via rust-toolchain.toml)
- Git

### For GTK4 UI Development
```bash
# Ubuntu/Debian
sudo apt install build-essential libgtk-4-dev libadwaita-1-dev

# Fedora
sudo dnf install gtk4-devel libadwaita-devel

# Arch/Manjaro
sudo pacman -S gtk4 libadwaita
```

### Optional (for full functionality)
- Flatpak: For testing Flatpak provider
- Podman: For testing source builds
- Alien: For testing package conversion

## Quick Build

### Build Everything (requires GTK4)
```bash
cargo build --all
```

### Build CLI and Daemon Only (no GTK4 required)
```bash
cargo build -p fluxara-cli -p fluxara-daemon
```

## Running

### CLI
```bash
# Show help
cargo run -p fluxara-cli

# Search for packages (requires flatpak)
cargo run -p fluxara-cli -- search firefox

# List installed packages (requires flatpak)
cargo run -p fluxara-cli -- list
```

### Daemon
```bash
cargo run -p fluxara-daemon
```

### GTK UI (requires GTK4 dev packages)
```bash
cargo run -p fluxara-ui-gtk
```

## Development Workflow

### Code Quality
```bash
# Format code
cargo fmt

# Run linter
cargo clippy --all-targets --all-features

# Run tests
cargo test
```

### Check Compilation (without GTK)
```bash
cargo check -p fluxara-core \
            -p fluxara-cli \
            -p fluxara-daemon \
            -p fluxara-provider-flatpak \
            -p fluxara-provider-apt \
            -p fluxara-provider-pacman
```

## Project Structure

```
Fluxara-Store/
├── crates/
│   ├── fluxara-core/          # Core types, config, traits
│   ├── fluxara-ui-gtk/        # GTK4/libadwaita UI
│   ├── fluxara-cli/           # Command-line interface
│   ├── fluxara-daemon/        # Background daemon
│   ├── fluxara-provider-*/    # Package manager providers
│   ├── fluxara-appstream/     # AppStream/ODRS integration
│   ├── fluxara-converter/     # Package conversion
│   ├── fluxara-builder/       # Source building
│   ├── fluxara-drivers/       # Driver management
│   ├── fluxara-maintenance/   # System maintenance
│   └── fluxara-polkit-agent/  # Privilege helper
├── Cargo.toml                  # Workspace config
├── README.md                   # Main documentation
├── IMPLEMENTATION.md           # Implementation summary
└── config.toml.example         # Example configuration
```

## Configuration

The application stores configuration at `~/.config/fluxara/config.toml`.

To customize:
```bash
cp config.toml.example ~/.config/fluxara/config.toml
# Edit the file as needed
```

## Testing with Flatpak

If you have Flatpak installed:

```bash
# Search
./target/release/fluxara search gimp

# List installed
./target/release/fluxara list

# Install (use with caution)
./target/release/fluxara install org.gimp.GIMP
```

## Common Tasks

### Add a new package provider
1. Create crate in `crates/fluxara-provider-<name>/`
2. Implement `PackageManager` trait from `fluxara-core`
3. Add to workspace `Cargo.toml`
4. Update CLI/UI to use new provider

### Modify configuration
1. Update structs in `fluxara-core/src/config.rs`
2. Update example in `config.toml.example`
3. Update UI settings in `fluxara-ui-gtk/src/window.rs`

### Add new UI tab
1. Add tab creation in `fluxara-ui-gtk/src/window.rs`
2. Implement content in new method (e.g., `create_<name>_page()`)

## Troubleshooting

### GTK4 not found
Install GTK4 development packages (see Prerequisites section).

### Flatpak commands fail
Install Flatpak: `sudo apt install flatpak` (or equivalent for your distro).

### Permission denied for package operations
Some operations require root. The polkit agent will handle this in the future.

## Next Steps

1. Read [README.md](README.md) for architecture overview
2. Check [IMPLEMENTATION.md](IMPLEMENTATION.md) for current status
3. Look at the roadmap in README for planned features
4. Pick an issue or feature to work on

## Contributing

See [README.md](README.md) for contribution guidelines.

## License

Dual-licensed under MIT or Apache-2.0.
