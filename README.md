# System Tray Linux AIO

A production-ready system tray application framework for Linux using the modern `stray` crate, with aloe-system-tray API compatibility.

> **Note**: This project now uses the `stray` crate - a minimal and modern StatusNotifierWatcher implementation for Linux system trays.

## Features

- 🖼️ **Modern System Tray API**: Uses `stray` for reliable Linux tray functionality
- 📋 **StatusNotifierWatcher**: Full support for the freedesktop.org standard
- 🎯 **Event-Driven Architecture**: React to tray icon updates and removals
- ⚙️ **Menu Interaction**: Send menu click commands programmatically
- 📝 **Comprehensive Logging**: Built-in logging with configurable levels
- 🚀 **Async/Await Support**: Built on Tokio for efficient async operations
- 🛡️ **Error Handling**: Robust error handling with custom error types
- 🔄 **aloe-system-tray Compatibility**: Optional compatibility layer for aloe API

## Architecture

The project now uses `stray` - a minimal SystemNotifierWatcher implementation that provides:
- Monitoring of system tray icons
- Menu interaction capabilities
- Event streaming for tray updates

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
system_tray_linux_aio = "0.1.0"
```

## Quick Start - Stray API

```rust
use system_tray_linux_aio::stray_impl::StrayTrayApp;
use system_tray_linux_aio::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = AppConfig::default();
    
    // Create and run the tray app
    let app = StrayTrayApp::new(config);
    app.run().await?;
    
    Ok(())
}
```

## Full Example with Menu Interaction

```rust
use stray::{SystemTray, message::{NotifierItemMessage, NotifierItemCommand}};
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    // Channel for sending menu commands
    let (ui_tx, ui_rx) = tokio::sync::mpsc::channel(32);
    let mut tray = SystemTray::new(ui_rx).await;

    while let Some(message) = tray.next().await {
        match message {
            NotifierItemMessage::Update { address, item, menu } => {
                println!("Tray icon updated: {:?}", item);
                
                // Send a menu click command
                ui_tx.send(NotifierItemCommand::MenuItemClicked {
                    submenu_id: 0,
                    menu_path: item.unwrap().menu.unwrap(),
                    notifier_address: address,
                }).await.unwrap();
            }
            NotifierItemMessage::Remove { address } => {
                println!("Tray icon removed: {}", address);
            }
        }
    }
}
```

## How Stray Works

Unlike traditional tray icon libraries that create icons, `stray` monitors existing system tray icons:

1. **StatusNotifierWatcher**: Implements the freedesktop.org StatusNotifierWatcher specification
2. **Event Streaming**: Provides a stream of tray icon updates and removals
3. **Menu Interaction**: Allows sending menu activation commands to tray icons
4. **DBus Integration**: Uses DBus for communication with the desktop environment

## Examples

The project includes several examples:

### Basic Stray Example
```bash
# Monitor all system tray icons
cargo run --example stray_example
```

### Interactive Example
```bash
# Interactive menu to send commands to tray icons
cargo run --example stray_interactive
```

### Aloe API Compatibility
```bash
# Use the aloe-system-tray compatible API
cargo run --example aloe_api_example
```

## Configuration

The application uses a TOML configuration file located at:
- Linux: `~/.config/system_tray_linux_aio/config.toml`

Example configuration:

```toml
app_name = "My Tray Monitor"
tooltip = "System Tray Monitor"
icon_path = "assets/icons/app.png"
start_minimized = true

[menu_config]
show_about = true
show_settings = true
show_quit = true
```

## Project Structure

```
src/
├── stray_impl/     # Stray crate implementation
├── aloe_compat/    # aloe-system-tray API compatibility
├── config/         # Configuration management
├── error/          # Error types
├── menu/           # Menu structures
└── tray/           # Legacy tray implementations

examples/
├── stray_example.rs       # Basic stray usage
├── stray_interactive.rs   # Interactive menu commands
└── aloe_api_example.rs    # Aloe-compatible API
```

## Key Differences from Traditional Tray Libraries

1. **Monitor vs Create**: Stray monitors existing tray icons rather than creating new ones
2. **System-wide View**: Can see and interact with all system tray icons
3. **Event-driven**: React to tray icon changes rather than managing your own icon
4. **Menu Interaction**: Send click commands to any tray icon's menu

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Check for errors
cargo check
```

### Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

### Logging

Set the `RUST_LOG` environment variable:

```bash
# Show all logs
RUST_LOG=debug cargo run

# Show only info and above
RUST_LOG=info cargo run

# Show stray crate logs
RUST_LOG=stray=debug cargo run
```

## Troubleshooting

### No tray icons detected
- Ensure you have applications with tray icons running
- Check that your desktop environment supports StatusNotifierItem
- Verify DBus is running: `systemctl status dbus`

### Permission errors
- Stray uses DBus, ensure your user has appropriate permissions
- Check DBus configuration in `/etc/dbus-1/`

### Desktop Environment Compatibility
Stray works with desktop environments that support the StatusNotifierItem specification:
- KDE Plasma ✓
- GNOME (with extensions) ✓
- XFCE ✓
- Most modern Linux DEs ✓

## Migration from tray-icon

If migrating from `tray-icon` or similar libraries:
1. Stray doesn't create icons - it monitors them
2. Use stray for system-wide tray monitoring
3. For creating your own tray icon, combine with other libraries

## License

This project is licensed under the GPL-3.0 license.

## Contributing

Contributions are welcome! Areas of interest:
- Enhanced menu interaction capabilities
- Better cross-desktop compatibility
- Additional examples and documentation

## Acknowledgments

- Built with [stray](https://crates.io/crates/stray) - A minimal SystemNotifierWatcher implementation
- Compatible with [aloe-system-tray](https://crates.io/crates/aloe-system-tray) API patterns
- Implements [StatusNotifierItem](https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/) specification