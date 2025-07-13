use anyhow::Result;
use system_tray_linux_aio::{AppConfig, stray_impl::StrayTrayApp};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "system_tray_linux_aio=info,stray=info".into()),
        )
        .init();
    
    info!("Starting System Tray Application");
    info!("Using stray crate for system tray functionality");
    
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
        let app = StrayTrayApp::new(config);
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