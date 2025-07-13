use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::AppConfig;
use crate::error::{Result, TrayError};
use crate::menu::TrayMenu;

#[cfg(target_os = "linux")]
mod linux_impl;

// Placeholder imports for aloe crates
// These will be used when the build issues are resolved
// use aloe_system_tray::SystemTrayIconComponent;
// use aloe_graphics::{Image, Colour};
// use aloe_menus::{PopupMenu};
// use aloe_events::{MouseEvent};

pub struct TrayIcon {
    component: SystemTrayIconComponent,
    config: Arc<RwLock<AppConfig>>,
    menu: TrayMenu,
}

impl TrayIcon {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config = Arc::new(RwLock::new(config));
        let menu = TrayMenu::new(config.clone()).await?;
        
        let component = SystemTrayIconComponent::new();
        
        Ok(Self {
            component,
            config,
            menu,
        })
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        let config = self.config.read().await;
        
        // Create images for the tray icon
        let colour_image = self.create_default_icon();
        let template_image = colour_image.clone();
        
        // Set up the icon
        self.component.set_icon_image(&colour_image, &template_image);
        
        // Set tooltip
        self.component.set_icon_tooltip(&config.tooltip);
        
        // Set up menu
        self.menu.setup_menu(&mut self.component).await?;
        
        tracing::info!("System tray icon initialized successfully");
        Ok(())
    }
    
    fn create_default_icon(&self) -> Image {
        // Create a simple default icon
        let mut image = Image::new(aloe_graphics::PixelFormat::ARGB, 32, 32, true);
        
        // Fill with a solid color for now
        let bounds = image.get_bounds();
        let g = aloe_graphics::Graphics::new(&mut image);
        g.fill_all(Colour::from_rgb(100, 150, 200)); // Light blue color
        
        image
    }
    
    pub async fn update_tooltip(&mut self, tooltip: &str) -> Result<()> {
        self.component.set_icon_tooltip(tooltip);
        
        let mut config = self.config.write().await;
        config.tooltip = tooltip.to_string();
        config.save()?;
        
        Ok(())
    }
    
    pub async fn set_highlighted(&mut self, highlighted: bool) {
        self.component.set_highlighted(highlighted);
    }
    
    pub async fn handle_events(&mut self) {
        // The aloe-system-tray component handles events internally
        // We can check for specific states or implement custom event handling
        // based on the component's state
    }
    
    pub fn show(&mut self) {
        // The SystemTrayIconComponent is shown by default when created
        // This method is here for API consistency
    }
    
    pub fn hide(&mut self) {
        // To hide, we would need to destroy and recreate the component
        // For now, we'll keep it visible
    }
    
    pub fn get_position(&self) -> (i32, i32) {
        let bounds = self.component.get_bounds();
        (bounds.get_x(), bounds.get_y())
    }
}