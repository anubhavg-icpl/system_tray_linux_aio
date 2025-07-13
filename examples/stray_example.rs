/// Example demonstrating the stray crate for system tray functionality
use anyhow::Result;
use stray::{SystemTray, message::{NotifierItemMessage, NotifierItemCommand}};
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tracing::{info, debug};
use std::collections::HashMap;

#[derive(Debug)]
struct TrayApp {
    ui_tx: mpsc::Sender<NotifierItemCommand>,
    notifiers: HashMap<String, NotifierInfo>,
}

#[derive(Debug, Clone)]
struct NotifierInfo {
    address: String,
    menu_path: String,
    title: String,
    icon: Option<String>,
}

impl TrayApp {
    fn new() -> (Self, mpsc::Receiver<NotifierItemCommand>) {
        let (ui_tx, ui_rx) = mpsc::channel(32);
        (
            Self {
                ui_tx,
                notifiers: HashMap::new(),
            },
            ui_rx
        )
    }
    
    async fn send_menu_click(&self, address: &str, submenu_id: i32) -> Result<()> {
        if let Some(info) = self.notifiers.get(address) {
            self.ui_tx.send(NotifierItemCommand::MenuItemClicked {
                submenu_id,
                menu_path: info.menu_path.clone(),
                notifier_address: address.to_string(),
            }).await?;
            
            info!("Sent click command for menu item {} on {}", submenu_id, info.title);
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();
    
    info!("Starting stray system tray example");
    
    // Create app and get the receiver
    let (mut app, ui_rx) = TrayApp::new();
    
    // Create the system tray
    let mut tray = SystemTray::new(ui_rx).await;
    
    info!("System tray created, listening for tray icons...");
    info!("This will show all system tray icons on your system!");
    info!("Press Ctrl+C to exit");
    
    // Spawn a task to simulate menu clicks after a delay
    let tx_clone = app.ui_tx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        info!("Demo: You can send menu commands programmatically");
        
        // Example: click menu item 0 on the first notifier we see
        // In a real app, you'd respond to user input
    });
    
    // Main event loop
    loop {
        tokio::select! {
            // Handle system tray messages
            Some(message) = tray.next() => {
                match message {
                    NotifierItemMessage::Update { address, item, menu } => {
                        info!("═══════════════════════════════════════════");
                        info!("📱 Tray Icon Updated");
                        info!("═══════════════════════════════════════════");
                        info!("Address: {}", address);
                        
                        if let Some(item) = item {
                            let title = item.title.clone().unwrap_or_else(|| "Untitled".to_string());
                            let icon = item.icon_name.clone().unwrap_or_else(|| "no-icon".to_string());
                            let status = item.status.clone();
                            
                            info!("Title: {}", title);
                            info!("Icon: {}", icon);
                            info!("Status: {:?}", status);
                            
                            if let Some(tooltip) = &item.tooltip {
                                info!("Tooltip: {:?}", tooltip);
                            }
                            
                            if let Some(menu_path) = &item.menu {
                                info!("Menu Path: {}", menu_path);
                                
                                // Store notifier info
                                let info = NotifierInfo {
                                    address: address.clone(),
                                    menu_path: menu_path.clone(),
                                    title: title.clone(),
                                    icon: Some(icon),
                                };
                                app.notifiers.insert(address.clone(), info);
                            }
                            
                            // Show attention status
                            if let Some(attention) = &item.icon_theme_path {
                                debug!("Icon Theme Path: {}", attention);
                            }
                        }
                        
                        if let Some(menu) = menu {
                            info!("Menu Structure:");
                            // The menu is a complex structure, let's show a summary
                            debug!("Menu details: {:?}", menu);
                        }
                        
                        info!("───────────────────────────────────────────\n");
                    }
                    
                    NotifierItemMessage::Remove { address } => {
                        info!("❌ Tray Icon Removed: {}", address);
                        if let Some(info) = app.notifiers.remove(&address) {
                            info!("  Was: {}", info.title);
                        }
                        info!("");
                    }
                }
            }
            
            // Handle Ctrl+C
            _ = tokio::signal::ctrl_c() => {
                info!("\nShutting down...");
                break;
            }
        }
    }
    
    info!("Example completed");
    Ok(())
}