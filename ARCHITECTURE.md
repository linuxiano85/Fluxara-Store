# Fluxara Store Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                          USER INTERFACES                             │
├──────────────────────┬──────────────────────┬───────────────────────┤
│                      │                      │                       │
│  fluxara-ui-gtk      │   fluxara-cli        │   fluxara-daemon      │
│  (GTK4/libadwaita)   │   (CLI Commands)     │   (Background)        │
│                      │                      │                       │
│  ┌─────────────┐     │  ┌─────────────┐     │   ┌──────────────┐   │
│  │   Home      │     │  │   search    │     │   │ Tray Icon    │   │
│  │   Updates   │     │  │   install   │     │   │ Notifications│   │
│  │   Drivers   │     │  │   remove    │     │   │ Auto-check   │   │
│  │   Maint.    │     │  │   update    │     │   └──────────────┘   │
│  │   Settings  │     │  │   list      │     │                       │
│  └─────────────┘     │  └─────────────┘     │                       │
│                      │                      │                       │
└──────────────┬───────┴──────────┬───────────┴───────────┬───────────┘
               │                  │                       │
               └──────────────────┼───────────────────────┘
                                  │
                                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                          CORE LAYER                                  │
│                        fluxara-core                                  │
│                                                                      │
│  ┌────────────────┐  ┌────────────────┐  ┌──────────────────────┐  │
│  │ Config System  │  │ Common Types   │  │ PackageManager Trait │  │
│  │ (TOML)         │  │ (Models)       │  │ (Interface)          │  │
│  └────────────────┘  └────────────────┘  └──────────────────────┘  │
│                                                                      │
└────────────────────────────────┬─────────────────────────────────────┘
                                 │
                ┌────────────────┼────────────────┐
                │                │                │
                ▼                ▼                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                      PACKAGE PROVIDERS                               │
├────────────────────┬────────────────────┬────────────────────────────┤
│                    │                    │                            │
│  fluxara-provider- │  fluxara-provider- │  fluxara-provider-         │
│  flatpak           │  apt               │  pacman                    │
│                    │                    │                            │
│  ┌──────────┐      │  ┌──────────┐      │  ┌──────────────────┐     │
│  │ Flatpak  │      │  │ APT      │      │  │ Pacman           │     │
│  │ CLI      │      │  │ apt-get  │      │  │ ┌──────────────┐ │     │
│  │ (DBus→)  │      │  │ apt-cache│      │  │ │Manjaro detect│ │     │
│  └──────────┘      │  └──────────┘      │  │ │AUR support   │ │     │
│                    │                    │  │ └──────────────┘ │     │
│                    │                    │  └──────────────────┘     │
└────────────────────┴────────────────────┴────────────────────────────┘
                                 │
                ┌────────────────┼────────────────┬──────────────┐
                │                │                │              │
                ▼                ▼                ▼              ▼
┌─────────────────────────────────────────────────────────────────────┐
│                        SERVICE LAYER                                 │
├─────────────┬──────────────┬──────────────┬──────────────┬──────────┤
│             │              │              │              │          │
│ fluxara-    │ fluxara-     │ fluxara-     │ fluxara-     │ fluxara- │
│ appstream   │ converter    │ builder      │ drivers      │ maint.   │
│             │              │              │              │          │
│ ┌─────────┐ │ ┌──────────┐ │ ┌──────────┐ │ ┌────────┐  │ ┌──────┐ │
│ │ ODRS    │ │ │ alien    │ │ │ Podman   │ │ │lspci   │  │ │Mirror│ │
│ │ Ratings │ │ │ Wrapper  │ │ │ Sandbox  │ │ │lsusb   │  │ │Speed │ │
│ │ Reviews │ │ │ Security │ │ │ Build    │ │ │modalias│  │ │Cache │ │
│ │         │ │ │ Policy   │ │ │ Systems  │ │ └────────┘  │ │Clean │ │
│ └─────────┘ │ └──────────┘ │ └──────────┘ │             │ └──────┘ │
│             │              │              │              │          │
└─────────────┴──────────────┴──────────────┴──────────────┴──────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     PRIVILEGE LAYER                                  │
│                  fluxara-polkit-agent                                │
│                                                                      │
│  ┌────────────────────────────────────────────────────────────┐    │
│  │  PolicyKit Integration                                      │    │
│  │  • Authorization checks                                     │    │
│  │  • Privileged command execution                             │    │
│  │  • Install/Remove/Update with root                          │    │
│  └────────────────────────────────────────────────────────────┘    │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

## Data Flow

### Package Installation
```
User (UI/CLI)
    ↓
fluxara-core (Config check)
    ↓
Provider (flatpak/apt/pacman)
    ↓
polkit-agent (if root needed)
    ↓
System package manager
```

### Configuration
```
~/.config/fluxara/config.toml
    ↓
fluxara-core (Load/Parse)
    ↓
UI/CLI/Daemon (Read settings)
    ↓
fluxara-core (Save changes)
    ↓
~/.config/fluxara/config.toml
```

### Update Check (Daemon)
```
Timer (1 hour)
    ↓
fluxara-daemon
    ↓
Providers (check updates)
    ↓
Notification (if updates found)
    ↓
Tray icon (if enabled)
```

## Key Design Principles

1. **Modularity**: Each crate has a single, clear responsibility
2. **Trait-based**: Common interfaces via `PackageManager` trait
3. **Security First**: PolicyKit for privilege, safe conversion policy
4. **Privacy First**: Telemetry opt-in, config transparency
5. **Distribution Aware**: Respects distro boundaries (Manjaro/Arch)
6. **Modern UI**: GTK4/libadwaita for native Linux experience

## Component Interactions

### Provider Selection
- **Flatpak**: Always available for Flatpak packages
- **APT**: Available on Debian/Ubuntu systems
- **Pacman**: Available on Arch/Manjaro systems
- **Selection**: Automatic based on package source

### Configuration Flow
1. Default config generated on first run
2. User modifies via Settings UI or config file
3. Changes saved atomically to TOML
4. All components read from same config

### Security Boundaries
- **User space**: UI, CLI, daemon (no root)
- **Privileged**: polkit-agent (root for system operations)
- **Sandbox**: builder (Podman containers for builds)
