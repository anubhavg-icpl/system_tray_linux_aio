use anyhow::Result;
use system_tray_linux_aio::{AppConfig, TrayIcon, MenuAction};
use tokio::signal;
use tracing::{info, error};

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
    
    // Create and initialize tray icon
    let mut tray = TrayIcon::new(config).await?;
    tray.initialize().await?;
    tray.show();
    
    info!("System tray icon created successfully");
    
    // Set up signal handlers
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate())?;
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt())?;
    
    // Main event loop
    loop {
        tokio::select! {
            _ = sigterm.recv() => {
                info!("Received SIGTERM, shutting down");
                break;
            }
            _ = sigint.recv() => {
                info!("Received SIGINT, shutting down");
                break;
            }
            _ = handle_tray_events(&mut tray) => {
                // Event handled
            }
        }
    }
    
    // Cleanup
    tray.hide();
    info!("Application shutdown complete");
    
    Ok(())
}

async fn handle_tray_events(tray: &mut TrayIcon) {
    // This would be connected to the actual event system
    // For now, we'll just handle events in a placeholder way
    tray.handle_events().await;
    
    // Simulate waiting for events
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}