use anyhow::Result;
use system_tray_linux_aio::{AppConfig, aloe_compat};
use tracing::{info, error};

#[cfg(target_os = "linux")]
mod app {
    use super::*;
    
    /// Run using aloe-system-tray compatible API
    pub async fn run_with_aloe_api(config: AppConfig) -> Result<()> {
        info!("Starting with aloe-system-tray compatible API");
        
        // Initialize GTK
        gtk::init()?;
        
        // Create system tray using aloe API pattern
        let mut tray = aloe_compat::SystemTrayIconComponent::new();
        
        // Create icon
        let icon = aloe_compat::create_icon_image();
        
        // Setup following aloe example pattern
        tray.set_icon_image(&icon, &icon); // colour and template
        tray.set_icon_tooltip(&config.tooltip);
        tray.set_highlighted(false);
        
        // Create menu
        let mut menu = aloe_compat::PopupMenu::new();
        
        // Add custom items
        for (idx, item) in config.menu_config.custom_items.iter().enumerate() {
            menu.add_item(idx as i32 + 1000, &item.label, item.enabled, false)?;
            if item.separator_after {
                menu.add_separator()?;
            }
        }
        
        if !config.menu_config.custom_items.is_empty() {
            menu.add_separator()?;
        }
        
        if config.menu_config.show_about {
            menu.add_item(1, "About", true, false)?;
        }
        
        if config.menu_config.show_settings {
            menu.add_item(2, "Settings", true, false)?;
        }
        
        if config.menu_config.show_quit {
            menu.add_separator()?;
            menu.add_item(3, "Quit", true, false)?;
        }
        
        // Initialize tray with menu
        tray.initialize_tray(&menu);
        tray.show_dropdown_menu(&menu);
        
        info!("System tray created using aloe-compatible API!");
        info!("App: {}", config.app_name);
        
        // Event loop
        let mut running = true;
        while running {
            // Check menu events
            if let Some(menu_id) = aloe_compat::MenuEventReceiver::try_recv() {
                match menu_id {
                    3 => {
                        info!("Quit selected");
                        running = false;
                    }
                    1 => info!("About selected"),
                    2 => info!("Settings selected"),
                    id if id >= 1000 => {
                        let idx = (id - 1000) as usize;
                        if let Some(item) = config.menu_config.custom_items.get(idx) {
                            info!("Custom item selected: {}", item.label);
                        }
                    }
                    _ => {}
                }
            }
            
            // Process GTK events
            while gtk::events_pending() {
                gtk::main_iteration();
            }
            
            // Check for Ctrl+C
            if tokio::signal::ctrl_c().await.is_ok() {
                info!("Received Ctrl+C");
                break;
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "system_tray_linux_aio=info".into()),
        )
        .init();
    
    info!("Starting System Tray Application");
    info!("Using aloe-system-tray compatible API");
    
    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => {
            info!("Loaded configuration from file");
            config
        },
        Err(e) => {
            info!("Using default configuration: {}", e);
            let default_config = AppConfig::default();
            
            // Try to save default config
            if let Err(e) = default_config.save() {
                error!("Failed to save default configuration: {}", e);
            }
            
            default_config
        }
    };
    
    #[cfg(target_os = "linux")]
    {
        app::run_with_aloe_api(config).await?;
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        error!("This application currently only supports Linux");
        return Err(anyhow::anyhow!("Unsupported platform"));
    }
    
    info!("Application shutdown complete");
    Ok(())
}