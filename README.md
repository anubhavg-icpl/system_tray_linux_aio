# System Tray Linux AIO

A system tray application framework built with Rust, designed to use the `aloe-system-tray` crate. This project provides a production-ready structure for system tray applications on Linux.

> **Note**: The `aloe-system-tray` crate (v0.1.2) is currently experiencing build issues on Linux due to platform-specific dependencies. See [ALOE_IMPLEMENTATION_GUIDE.md](ALOE_IMPLEMENTATION_GUIDE.md) for details and workarounds.

## Features

- 🖼️ **Dynamic Icon Management**: Set and update tray icons at runtime
- 📋 **Customizable Menus**: Build context menus with custom actions
- 🎯 **Event Handling**: Respond to user interactions (clicks, right-clicks)
- ⚙️ **Configuration System**: TOML-based configuration with hot-reload support
- 📝 **Comprehensive Logging**: Built-in logging with configurable levels
- 🚀 **Async/Await Support**: Built on Tokio for efficient async operations
- 🛡️ **Error Handling**: Robust error handling with custom error types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
system_tray_linux_aio = "0.1.0"
```

## Quick Start

```rust
use system_tray_linux_aio::{AppConfig, TrayIcon};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = AppConfig::default();
    
    // Create and initialize tray icon
    let mut tray = TrayIcon::new(config).await?;
    tray.initialize().await?;
    tray.show();
    
    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}
```

## Configuration

The application uses a TOML configuration file located at:
- Linux: `~/.config/system_tray_linux_aio/config.toml`
- macOS: `~/Library/Application Support/system_tray_linux_aio/config.toml`
- Windows: `%APPDATA%\system_tray_linux_aio\config.toml`

Example configuration:

```toml
app_name = "My App"
tooltip = "Click for menu"
icon_path = "assets/icons/app.png"
dark_icon_path = "assets/icons/app_dark.png"
start_minimized = true
auto_start = false

[menu_config]
show_about = true
show_settings = true
show_quit = true

[[menu_config.custom_items]]
label = "Open Dashboard"
action = "open_dashboard"
enabled = true
separator_after = false
```

## Architecture

The project is organized into the following modules:

- **`config`**: Configuration management and serialization
- **`error`**: Custom error types and error handling
- **`tray`**: Core system tray functionality
- **`menu`**: Menu building and event handling

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run example
cargo run --example basic_tray
```

### Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

### Logging

Set the `RUST_LOG` environment variable to control logging:

```bash
RUST_LOG=system_tray_linux_aio=debug cargo run
```

## Examples

See the `examples/` directory for more usage examples:

- `basic_tray.rs`: Simple tray icon with menu
- More examples coming soon...

## License

This project is licensed under the GPL-3.0 license, consistent with the `aloe-system-tray` dependency.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Acknowledgments

Built on top of the [aloe-system-tray](https://crates.io/crates/aloe-system-tray) crate, which is a Rust translation of the JUCE C++ framework's system tray module.