use aloe_menus::{PopupMenu, ApplicationCommandTarget};
use aloe_system_tray::SystemTrayIconComponent;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::AppConfig;
use crate::error::{Result, TrayError};

pub struct TrayMenu {
    config: Arc<RwLock<AppConfig>>,
    menu: PopupMenu,
}

impl TrayMenu {
    pub async fn new(config: Arc<RwLock<AppConfig>>) -> Result<Self> {
        let menu = PopupMenu::new();
        
        Ok(Self {
            config,
            menu,
        })
    }
    
    pub async fn setup_menu(&mut self, tray: &mut SystemTrayIconComponent) -> Result<()> {
        let config = self.config.read().await;
        
        self.menu.clear();
        
        // Add custom menu items
        for (index, item) in config.menu_config.custom_items.iter().enumerate() {
            self.menu.add_item(
                index + 1,
                &item.label,
                item.enabled,
                false // ticked
            );
            
            if item.separator_after {
                self.menu.add_separator();
            }
        }
        
        // Add default items
        if !config.menu_config.custom_items.is_empty() && 
           (config.menu_config.show_about || config.menu_config.show_settings || config.menu_config.show_quit) {
            self.menu.add_separator();
        }
        
        let mut next_id = config.menu_config.custom_items.len() + 1;
        
        if config.menu_config.show_about {
            self.menu.add_item(next_id, "About", true, false);
            next_id += 1;
        }
        
        if config.menu_config.show_settings {
            self.menu.add_item(next_id, "Settings", true, false);
            next_id += 1;
        }
        
        if config.menu_config.show_quit {
            if config.menu_config.show_about || config.menu_config.show_settings {
                self.menu.add_separator();
            }
            self.menu.add_item(next_id, "Quit", true, false);
        }
        
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