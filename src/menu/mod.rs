use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::AppConfig;
use crate::error::Result;
use crate::tray::{SystemTrayIconComponent, PopupMenu};

pub struct TrayMenu {
    config: Arc<RwLock<AppConfig>>,
    #[allow(dead_code)]
    menu: PopupMenu,
}

impl TrayMenu {
    pub async fn new(config: Arc<RwLock<AppConfig>>) -> Result<Self> {
        let menu = PopupMenu;
        
        Ok(Self {
            config,
            menu,
        })
    }
    
    pub async fn setup_menu(&mut self, _tray: &mut SystemTrayIconComponent) -> Result<()> {
        let config = self.config.read().await;
        
        tracing::info!("Setting up menu:");
        
        // Log custom menu items
        for item in &config.menu_config.custom_items {
            tracing::info!("  - {} (action: {})", item.label, item.action);
            if item.separator_after {
                tracing::info!("  ---separator---");
            }
        }
        
        // Log default items
        if config.menu_config.show_about {
            tracing::info!("  - About");
        }
        if config.menu_config.show_settings {
            tracing::info!("  - Settings");
        }
        if config.menu_config.show_quit {
            tracing::info!("  - Quit");
        }
        
        // When aloe-menus is properly implemented, we'll use:
        // tray.show_dropdown_menu(&self.menu);
        
        Ok(())
    }
    
    pub async fn handle_menu_result(&self, result: i32) -> Result<Option<MenuAction>> {
        let config = self.config.read().await;
        
        // Handle custom menu items
        if result > 0 && result <= config.menu_config.custom_items.len() as i32 {
            let item = &config.menu_config.custom_items[(result - 1) as usize];
            return Ok(Some(MenuAction::Custom(item.action.clone())));
        }
        
        // Handle default menu items
        let custom_count = config.menu_config.custom_items.len() as i32;
        let adjusted_result = result - custom_count;
        
        match adjusted_result {
            1 if config.menu_config.show_about => Ok(Some(MenuAction::About)),
            1 if !config.menu_config.show_about && config.menu_config.show_settings => Ok(Some(MenuAction::Settings)),
            2 if config.menu_config.show_about && config.menu_config.show_settings => Ok(Some(MenuAction::Settings)),
            id if id > 0 => {
                let quit_id = if config.menu_config.show_about && config.menu_config.show_settings {
                    3
                } else if config.menu_config.show_about || config.menu_config.show_settings {
                    2
                } else {
                    1
                };
                
                if id == quit_id && config.menu_config.show_quit {
                    Ok(Some(MenuAction::Quit))
                } else {
                    Ok(None)
                }
            },
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone)]
pub enum MenuAction {
    About,
    Settings,
    Quit,
    Custom(String),
}