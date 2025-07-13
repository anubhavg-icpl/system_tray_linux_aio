# How to Run System Tray Linux AIO

## Prerequisites

1. **Linux Desktop Environment with System Tray Support**:
   - KDE Plasma ✅ (works out of the box)
   - XFCE ✅ (works out of the box)
   - MATE ✅ (works out of the box)
   - GNOME ⚠️ (needs AppIndicator extension)
   - Ubuntu Unity ✅ (works out of the box)

2. **For GNOME Users**:
   Install the AppIndicator extension:
   ```bash
   # Install extension
   sudo apt install gnome-shell-extension-appindicator
   
   # Enable it
   gnome-extensions enable ubuntu-appindicator@ubuntu.com
   
   # Restart GNOME Shell (Alt+F2, type 'r', press Enter)
   ```

## Building the Application

```bash
# Clone or navigate to the project directory
cd /home/anubhavg/Desktop/system_tray_linux_aio

# Build in release mode for best performance
cargo build --release
```

## Running the Application

### Method 1: Using Cargo (Development)
```bash
# Run with default logging
cargo run --release

# Run with debug logging to see more details
RUST_LOG=debug cargo run --release

# Run with trace logging for maximum verbosity
RUST_LOG=trace cargo run --release
```

### Method 2: Direct Binary (Production)
```bash
# Run the compiled binary
./target/release/system_tray_linux_aio

# Or with logging
RUST_LOG=info ./target/release/system_tray_linux_aio
```

## What to Expect

1. **Console Output**: You'll see messages like:
   ```
   2025-07-13T19:XX:XX.XXXXXX  INFO system_tray_linux_aio: Starting System Tray Application
   2025-07-13T19:XX:XX.XXXXXX  INFO system_tray_linux_aio: System tray icon created successfully!
   2025-07-13T19:XX:XX.XXXXXX  INFO system_tray_linux_aio: Right-click the tray icon to see the menu
   2025-07-13T19:XX:XX.XXXXXX  INFO system_tray_linux_aio: Tray icon is running. You should see it in your system tray!
   ```

2. **System Tray Icon**: 
   - Look for a blue circle with white background and "A" letter
   - It appears in your system tray area (usually bottom-right or top-right)
   - The icon might be in a hidden/overflow area

3. **Interacting with the Icon**:
   - **Right-click**: Opens the menu
   - **Hover**: Shows tooltip "Click to open menu"
   - **Menu Items**:
     - About
     - Settings
     - Quit (click this to exit the application)

## Troubleshooting

### Can't See the Icon?

1. **Check Hidden Icons**:
   - Click the arrow/chevron in your system tray to show hidden icons
   - The icon might be collapsed in the overflow area

2. **GNOME Specific**:
   - Make sure AppIndicator extension is installed and enabled
   - Check in GNOME Extensions app that it's active
   - Try logging out and back in

3. **General Linux**:
   - Some minimal window managers might not have system tray support
   - Try running `ps aux | grep system_tray` to confirm it's running

### Application Crashes?

Run with debug logging:
```bash
RUST_LOG=debug cargo run --release 2>&1 | tee debug.log
```

Then check `debug.log` for error messages.

### Menu Not Working?

- Make sure to RIGHT-click (not left-click) the icon
- Some desktop environments might require a single left-click
- Try both clicking methods

## Stopping the Application

You can stop the application in three ways:

1. **Via Menu**: Right-click icon → Click "Quit"
2. **Terminal**: Press `Ctrl+C` in the terminal where it's running
3. **Kill Process**: `pkill system_tray_linux_aio`

## Configuration

The app creates a config file at:
```
~/.config/system_tray_linux_aio/config.toml
```

You can edit this file to customize:
- App name
- Tooltip text
- Menu items
- Other settings

## Success Indicators

You know it's working when:
1. ✅ Console shows "System tray icon created successfully!"
2. ✅ No error messages in console
3. ✅ Icon appears in system tray
4. ✅ Right-click shows menu
5. ✅ Quit menu item closes the app

## For Developers

To see the actual aloe-system-tray API structure (currently scaffolded):
- Check `src/tray/mod.rs` for the trait definitions
- See `src/contrib/linux_impl_example.rs` for Linux implementation ideas
- The working implementation uses `tray-icon` crate as a temporary solution

Enjoy your system tray application! 🎉