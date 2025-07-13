// Working implementation using tray-icon while maintaining aloe API structure

use tray_icon::{
    Icon as TrayIconIcon, TrayIcon as TrayIconLib, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};
use image::RgbaImage;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::AppConfig;
use crate::error::{Result, TrayError};
use crate::menu::MenuAction;

pub struct WorkingTrayIcon {
    tray: Option<TrayIconLib>,
    config: Arc<RwLock<AppConfig>>,
    menu: Option<Menu>,
}

impl WorkingTrayIcon {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config = Arc::new(RwLock::new(config));
        
        Ok(Self {
            tray: None,
            config,
            menu: None,
        })
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        let config = self.config.read().await;
        
        // Create a default icon
        let icon = self.create_default_icon()?;
        
        // Create menu
        let menu = self.create_menu(&config).await?;
        
        // Build tray icon
        let tray = TrayIconBuilder::new()
            .with_icon(icon)
            .with_tooltip(&config.tooltip)
            .with_menu(Box::new(menu.clone()))
            .build()
            .map_err(|e| TrayError::InitializationError(e.to_string()))?;
        
        self.tray = Some(tray);
        self.menu = Some(menu);
        
        tracing::info!("System tray icon initialized successfully");
        Ok(())
    }
    
    fn create_default_icon(&self) -> Result<TrayIconIcon> {
        // Create a simple 32x32 colored icon
        let mut img = RgbaImage::new(32, 32);
        
        // Fill with a gradient
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (x * 255 / 32) as u8;
            let g = (y * 255 / 32) as u8;
            let b = 128;
            let a = 255;
            *pixel = image::Rgba([r, g, b, a]);
        }
        
        let rgba_data = img.into_raw();
        TrayIconIcon::from_rgba(rgba_data, 32, 32)
            .map_err(|e| TrayError::IconLoadError(e.to_string()))
    }
    
    async fn create_menu(&self, config: &AppConfig) -> Result<Menu> {
        let menu = Menu::new();
        
        // Add custom menu items
        for item in &config.menu_config.custom_items {
            let menu_item = MenuItem::new(&item.label, item.enabled, None);
            menu.append(&menu_item)
                .map_err(|e| TrayError::MenuError(e.to_string()))?;
            
            if item.separator_after {
                menu.append(&PredefinedMenuItem::separator())
                    .map_err(|e| TrayError::MenuError(e.to_string()))?;
            }
        }
        
        // Add default items
        if !config.menu_config.custom_items.is_empty() && 
           (config.menu_config.show_about || config.menu_config.show_settings || config.menu_config.show_quit) {
            menu.append(&PredefinedMenuItem::separator())
                .map_err(|e| TrayError::MenuError(e.to_string()))?;
        }
        
        if config.menu_config.show_about {
            let about_item = MenuItem::new("About", true, None);
            menu.append(&about_item)
                .map_err(|e| TrayError::MenuError(e.to_string()))?;
        }
        
        if config.menu_config.show_settings {
            let settings_item = MenuItem::new("Settings", true, None);
            menu.append(&settings_item)
                .map_err(|e| TrayError::MenuError(e.to_string()))?;
        }
        
        if config.menu_config.show_quit {
            if config.menu_config.show_about || config.menu_config.show_settings {
                menu.append(&PredefinedMenuItem::separator())
                    .map_err(|e| TrayError::MenuError(e.to_string()))?;
            }
            let quit_item = MenuItem::new("Quit", true, None);
            menu.append(&quit_item)
                .map_err(|e| TrayError::MenuError(e.to_string()))?;
        }
        
        Ok(menu)
    }
    
    pub async fn update_tooltip(&mut self, tooltip: &str) -> Result<()> {
        if let Some(tray) = &mut self.tray {
            tray.set_tooltip(Some(tooltip))
                .map_err(|e| TrayError::EventError(e.to_string()))?;
            
            let mut config = self.config.write().await;
            config.tooltip = tooltip.to_string();
            config.save()?;
        }
        
        Ok(())
    }
    
    pub async fn handle_menu_event(&self) -> Option<MenuAction> {
        if let Ok(event) = MenuEvent::receiver().try_recv() {
            // Map menu event to action based on menu item ID
            // This is simplified - in production you'd track menu item IDs
            return Some(MenuAction::Custom(format!("menu_event_{}", event.id)));
        }
        None
    }
    
    pub async fn handle_events(&mut self) -> Option<MenuAction> {
        self.handle_menu_event().await
    }
    
    pub fn show(&mut self) {
        if let Some(tray) = &mut self.tray {
            let _ = tray.set_visible(true);
        }
    }
    
    pub fn hide(&mut self) {
        if let Some(tray) = &mut self.tray {
            let _ = tray.set_visible(false);
        }
    }
}