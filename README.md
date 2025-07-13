# System Tray Linux AIO

A production-ready system tray application framework for Linux with aloe-system-tray API compatibility.

> **Note**: The original `aloe-system-tray` crate has build issues on Linux due to macOS dependencies. This project provides a compatibility layer that implements the aloe API using working Linux libraries.

## Features

- 🖼️ **aloe-system-tray API Compatibility**: Use the same API as aloe-system-tray
- 📋 **Cross-platform Design**: Implements the aloe pattern for future portability
- 🎯 **Event Handling**: Full mouse event and menu interaction support
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

## Quick Start - aloe-system-tray API

```rust
use system_tray_linux_aio::aloe_compat::{SystemTrayIconComponent, create_icon_image};

// Following the aloe-system-tray example pattern
impl SystemTrayIconComponent {
    fn setup_icon(&mut self) {
        let colour_image = create_icon_image();
        let template_image = colour_image.clone();
        
        self.set_icon_image(&colour_image, &template_image);
        self.set_icon_tooltip("Application running");
        self.set_highlighted(true);
    }
}
```

## Full Example

```rust
use system_tray_linux_aio::aloe_compat::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GTK (required on Linux)
    gtk::init()?;
    
    // Create system tray component
    let mut tray = SystemTrayIconComponent::new();
    
    // Create icon
    let icon = create_icon_image();
    
    // Setup icon (aloe API)
    tray.set_icon_image(&icon, &icon);
    tray.set_icon_tooltip("My Application");
    tray.set_highlighted(false);
    
    // Create menu
    let mut menu = PopupMenu::new();
    menu.add_item(1, "About", true, false)?;
    menu.add_item(2, "Settings", true, false)?;
    menu.add_separator()?;
    menu.add_item(3, "Quit", true, false)?;
    
    // Initialize and show
    tray.initialize_tray(&menu);
    
    // Event loop
    loop {
        if let Some(menu_id) = MenuEventReceiver::try_recv() {
            match menu_id {
                3 => break, // Quit
                1 => println!("About clicked"),
                2 => println!("Settings clicked"),
                _ => {}
            }
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(())
}
```

## API Compatibility

This project implements the following aloe-system-tray API methods:

- `SystemTrayIconComponent::new()` - Create a new tray component
- `set_icon_image(&mut self, colour_image: &Image, template_image: &Image)` - Set tray icon
- `set_icon_tooltip(&mut self, tooltip: &str)` - Set tooltip text
- `set_highlighted(&mut self, should_highlight: bool)` - Set highlight state
- `show_info_bubble(&mut self, title: &str, content: &str)` - Show notification (limited support)
- `hide_info_bubble(&mut self)` - Hide notification
- `show_dropdown_menu(&mut self, menu: &PopupMenu)` - Display menu

## Configuration

The application uses a TOML configuration file located at:
- Linux: `~/.config/system_tray_linux_aio/config.toml`

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

- **`aloe_compat`**: aloe-system-tray API compatibility layer
- **`config`**: Configuration management and serialization
- **`error`**: Custom error types and error handling
- **`tray`**: Core system tray functionality
- **`menu`**: Menu building and event handling

## Implementation Details

Since the original aloe-system-tray crate has build issues on Linux, this project:

1. Provides a compatibility module (`aloe_compat`) that implements the aloe API
2. Uses `tray-icon` crate underneath for actual system tray functionality
3. Translates between aloe API calls and tray-icon implementations
4. Maintains API compatibility for easy migration when aloe-system-tray is fixed

## Examples

See the `examples/` directory:

- `aloe_api_example.rs`: Using the aloe-system-tray compatible API
- `basic_tray.rs`: Direct API usage example

Run examples:

```bash
# Run aloe API example
cargo run --example aloe_api_example

# Run basic example
cargo run --example basic_tray
```

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Testing

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test
```

## Troubleshooting

If you encounter build errors related to aloe crates:
- This project already includes the necessary workarounds
- Check that you're using the compatibility layer (`aloe_compat` module)
- See [ALOE_IMPLEMENTATION_GUIDE.md](ALOE_IMPLEMENTATION_GUIDE.md) for technical details

## License

This project is licensed under the GPL-3.0 license, consistent with the aloe-system-tray licensing.

## Contributing

Contributions are welcome! Areas of interest:
- Improving aloe API compatibility
- Adding missing aloe-system-tray features
- Cross-platform support (Windows, macOS)

## Acknowledgments

- Built to be compatible with [aloe-system-tray](https://crates.io/crates/aloe-system-tray)
- Uses [tray-icon](https://crates.io/crates/tray-icon) for Linux implementation
- aloe-system-tray is a Rust translation of the JUCE C++ framework