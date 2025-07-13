use system_tray_linux_aio::{AppConfig, TrayIcon, MenuAction};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a custom configuration
    let mut config = AppConfig::default();
    config.app_name = "My Tray App".to_string();
    config.tooltip = "My awesome tray application".to_string();
    
    // Add custom menu items
    config.menu_config.custom_items = vec![
        system_tray_linux_aio::config::MenuItem {
            label: "Open Dashboard".to_string(),
            action: "open_dashboard".to_string(),
            enabled: true,
            separator_after: false,
        },
        system_tray_linux_aio::config::MenuItem {
            label: "Check Updates".to_string(),
            action: "check_updates".to_string(),
            enabled: true,
            separator_after: true,
        },
    ];
    
    // Create and run the tray icon
    let mut tray = TrayIcon::new(config).await?;
    tray.initialize().await?;
    tray.show();
    
    println!("Tray icon is running. Press Ctrl+C to exit.");
    
    // Keep the application running
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}