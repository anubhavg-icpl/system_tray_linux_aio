use anyhow::Result;
use system_tray_linux_aio::AppConfig;
use tracing::{info, error};

#[cfg(target_os = "linux")]
mod linux_tray {
    use super::*;
    use tray_icon::{
        Icon, TrayIcon as TrayIconLib, TrayIconBuilder,
        menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    };
    use image::RgbaImage;
    use std::time::Duration;
    
    pub struct LinuxTrayApp {
        config: AppConfig,
        tray: Option<TrayIconLib>,
        menu: Option<Menu>,
    }
    
    impl LinuxTrayApp {
        pub fn new(config: AppConfig) -> Self {
            Self { config, tray: None, menu: None }
        }
        
        fn create_icon() -> Result<Icon> {
            // Create a 32x32 icon with a simple design
            let mut img = RgbaImage::new(32, 32);
            
            // Fill with a blue background
            for pixel in img.pixels_mut() {
                *pixel = image::Rgba([52, 152, 219, 255]); // Nice blue color
            }
            
            // Draw a white circle in the center
            let center_x = 16;
            let center_y = 16;
            let radius = 12;
            
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                let dx = x as i32 - center_x;
                let dy = y as i32 - center_y;
                if dx * dx + dy * dy <= radius * radius {
                    *pixel = image::Rgba([255, 255, 255, 255]); // White
                }
            }
            
            // Draw a blue "A" in the center
            for y in 8..24 {
                for x in 8..24 {
                    // Simple "A" pattern
                    if (x == 15 && y >= 10 && y <= 20) || // Left line
                       (x == 17 && y >= 10 && y <= 20) || // Right line
                       (y == 10 && x >= 15 && x <= 17) || // Top
                       (y == 15 && x >= 14 && x <= 18) {  // Middle bar
                        img.put_pixel(x, y, image::Rgba([52, 152, 219, 255]));
                    }
                }
            }
            
            let rgba = img.into_raw();
            Icon::from_rgba(rgba, 32, 32)
                .map_err(|e| anyhow::anyhow!("Failed to create icon: {}", e))
        }
        
        fn create_menu(&self) -> Result<Menu> {
            let menu = Menu::new();
            
            // Add custom items from config
            for item in &self.config.menu_config.custom_items {
                let menu_item = MenuItem::new(&item.label, item.enabled, None);
                menu.append(&menu_item)?;
                
                if item.separator_after {
                    menu.append(&PredefinedMenuItem::separator())?;
                }
            }
            
            // Add default items
            if !self.config.menu_config.custom_items.is_empty() {
                menu.append(&PredefinedMenuItem::separator())?;
            }
            
            if self.config.menu_config.show_about {
                menu.append(&MenuItem::new("About", true, None))?;
            }
            
            if self.config.menu_config.show_settings {
                menu.append(&MenuItem::new("Settings", true, None))?;
            }
            
            if self.config.menu_config.show_quit {
                menu.append(&PredefinedMenuItem::separator())?;
                menu.append(&MenuItem::new("Quit", true, None))?;
            }
            
            Ok(menu)
        }
        
        pub async fn run(&mut self) -> Result<()> {
            info!("Starting Linux system tray application");
            info!("App name: {}", self.config.app_name);
            info!("Tooltip: {}", self.config.tooltip);
            
            // Initialize GTK for tray-icon
            gtk::init().map_err(|e| anyhow::anyhow!("Failed to initialize GTK: {}", e))?;
            
            // Create icon
            let icon = Self::create_icon()?;
            
            // Create menu
            let menu = self.create_menu()?;
            
            // Create tray icon
            let tray = TrayIconBuilder::new()
                .with_icon(icon)
                .with_tooltip(&self.config.tooltip)
                .with_menu(Box::new(menu.clone()))
                .build()?;
            
            self.tray = Some(tray);
            self.menu = Some(menu);
            
            info!("System tray icon created successfully!");
            info!("Right-click the tray icon to see the menu");
            
            // Event loop - keep running until quit
            let menu_channel = MenuEvent::receiver();
            let mut ctrl_c = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt())?;
            
            info!("Tray icon is running. You should see it in your system tray!");
            info!("If you don't see it:");
            info!("  - Check your system tray area (might be hidden)");
            info!("  - Make sure you have a system tray (GNOME needs extension)");
            info!("  - Try running with RUST_LOG=debug for more info");
            
            loop {
                // Check for menu events
                if let Ok(event) = menu_channel.try_recv() {
                    info!("Menu event received: {:?}", event.id);
                    
                    // For now, just log the event
                    // In a real app, you'd match on specific menu IDs
                    if let Some(_menu) = &self.menu {
                        // Check if quit was clicked (simple heuristic)
                        if self.config.menu_config.show_quit {
                            info!("Menu item clicked - check if it's quit");
                            // For demo, any click on last few items might be quit
                        }
                    }
                }
                
                // Check for Ctrl+C using poll
                tokio::select! {
                    _ = ctrl_c.recv() => {
                        info!("Received Ctrl+C, shutting down");
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(100)) => {
                        // Continue loop
                    }
                }
            }
            
            Ok(())
        }
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
    
    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => {
            info!("Loaded configuration from file");
            config
        },
        Err(e) => {
            info!("Using default configuration: {}", e);
            let default_config = AppConfig::default();
            
            // Try to save default config for future use
            if let Err(e) = default_config.save() {
                error!("Failed to save default configuration: {}", e);
            }
            
            default_config
        }
    };
    
    #[cfg(target_os = "linux")]
    {
        let mut app = linux_tray::LinuxTrayApp::new(config);
        app.run().await?;
    }
    
    #[cfg(not(target_os = "linux"))]
    {
        error!("This application currently only supports Linux");
        return Err(anyhow::anyhow!("Unsupported platform"));
    }
    
    info!("Application shutdown complete");
    Ok(())
}