/// Implementation using the stray crate - a modern Linux system tray API
use anyhow::Result;
use stray::{SystemTray, message::{NotifierItemMessage, NotifierItemCommand}};
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use crate::AppConfig;
use tracing::{info, debug, error};
use std::collections::HashMap;

pub struct StrayTrayApp {
    config: AppConfig,
    ui_tx: mpsc::Sender<NotifierItemCommand>,
    ui_rx: mpsc::Receiver<NotifierItemCommand>,
    notifier_items: HashMap<String, NotifierInfo>,
}

#[derive(Debug, Clone)]
struct NotifierInfo {
    address: String,
    menu_path: String,
    title: String,
    icon_name: Option<String>,
}

impl StrayTrayApp {
    pub fn new(config: AppConfig) -> Self {
        let (ui_tx, ui_rx) = mpsc::channel(32);
        Self {
            config,
            ui_tx,
            ui_rx,
            notifier_items: HashMap::new(),
        }
    }
    
    pub fn get_sender(&self) -> mpsc::Sender<NotifierItemCommand> {
        self.ui_tx.clone()
    }
    
    pub async fn run(mut self) -> Result<()> {
        info!("Starting stray-based Linux system tray application");
        info!("App name: {}", self.config.app_name);
        info!("Tooltip: {}", self.config.tooltip);
        
        // Create the system tray
        let mut tray = SystemTray::new(self.ui_rx).await;
        
        info!("System tray created successfully!");
        info!("Listening for tray icon changes...");
        
        // Event loop - listen for tray updates
        loop {
            tokio::select! {
                // Handle system tray messages
                Some(message) = tray.next() => {
                    match message {
                        NotifierItemMessage::Update { address, item, menu } => {
                            info!("NotifierItem updated:");
                            info!("  Address: {}", address);
                            debug!("  Item: {:?}", item);
                            debug!("  Menu: {:?}", menu);
                            
                            // Store notifier information
                            if let Some(item) = item {
                                let info = NotifierInfo {
                                    address: address.clone(),
                                    menu_path: item.menu.clone().unwrap_or_default(),
                                    title: item.title.clone().unwrap_or_else(|| "Unknown".to_string()),
                                    icon_name: item.icon_name.clone(),
                                };
                                
                                info!("  Title: {}", info.title);
                                if let Some(icon) = &info.icon_name {
                                    info!("  Icon: {}", icon);
                                }
                                
                                self.notifier_items.insert(address, info);
                            }
                        }
                        NotifierItemMessage::Remove { address } => {
                            info!("NotifierItem removed: {}", address);
                            self.notifier_items.remove(&address);
                        }
                    }
                }
                
                // Handle Ctrl+C
                _ = tokio::signal::ctrl_c() => {
                    info!("Received Ctrl+C, shutting down");
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    /// Send a menu item click command
    pub async fn click_menu_item(&self, submenu_id: i32, notifier_address: &str) -> Result<()> {
        if let Some(info) = self.notifier_items.get(notifier_address) {
            let command = NotifierItemCommand::MenuItemClicked {
                submenu_id,
                menu_path: info.menu_path.clone(),
                notifier_address: notifier_address.to_string(),
            };
            
            self.ui_tx.send(command).await?;
            info!("Sent menu click command for item {} on {}", submenu_id, notifier_address);
        } else {
            error!("Notifier address {} not found", notifier_address);
        }
        
        Ok(())
    }
}

/// Aloe-compatible wrapper using stray
pub mod aloe_compat {
    use super::*;
    use image::RgbaImage;
    
    pub struct SystemTrayIconComponent {
        app: Option<StrayTrayApp>,
        config: AppConfig,
    }
    
    impl SystemTrayIconComponent {
        pub fn new() -> Self {
            let config = AppConfig::default();
            Self {
                app: None,
                config,
            }
        }
        
        pub fn set_icon_tooltip(&mut self, tooltip: &str) {
            self.config.tooltip = tooltip.to_string();
        }
        
        pub async fn run(mut self) -> Result<()> {
            let app = StrayTrayApp::new(self.config);
            app.run().await
        }
    }
    
    /// Helper to create icon (note: stray uses system icons, not custom images)
    pub fn create_system_icon_name() -> String {
        // Return a standard system icon name
        // You can customize this based on your app
        "application-x-executable".to_string()
    }
}