use anyhow::Result;
use system_tray_linux_aio::{AppConfig, MenuAction};
use tracing::{info, error};

#[cfg(target_os = "linux")]
mod linux_tray {
    use super::*;
    use std::process::Command;
    
    pub struct LinuxTrayApp {
        config: AppConfig,
    }
    
    impl LinuxTrayApp {
        pub fn new(config: AppConfig) -> Self {
            Self { config }
        }
        
        pub async fn run(&mut self) -> Result<()> {
            info!("Starting Linux system tray application");
            info!("App name: {}", self.config.app_name);
            info!("Tooltip: {}", self.config.tooltip);
            
            // For now, we'll create a simple implementation
            // that demonstrates the structure without the problematic dependencies
            
            info!("System tray would be initialized here with aloe-system-tray");
            info!("Menu items:");
            for item in &self.config.menu_config.custom_items {
                info!("  - {}: {}", item.label, item.action);
            }
            
            // Keep the application running
            tokio::signal::ctrl_c().await?;
            
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