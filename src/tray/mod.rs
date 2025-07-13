use aloe_system_tray::SystemTrayIconComponent;
use aloe_image::Image;
use aloe_graphics::Colour;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::config::AppConfig;
use crate::error::{Result, TrayError};
use crate::menu::TrayMenu;

pub struct TrayIcon {
    component: SystemTrayIconComponent,
    config: Arc<RwLock<AppConfig>>,
    menu: TrayMenu,
}

impl TrayIcon {
    pub async fn new(config: AppConfig) -> Result<Self> {
        let config = Arc::new(RwLock::new(config));
        let menu = TrayMenu::new(config.clone()).await?;
        
        let mut component = SystemTrayIconComponent::new();
        
        Ok(Self {
            component,
            config,
            menu,
        })
    }
    
    pub async fn initialize(&mut self) -> Result<()> {
        let config = self.config.read().await;
        
        // Load icon images
        let colour_image = self.load_icon(&config.icon_path)?;
        let template_image = if let Some(dark_path) = &config.dark_icon_path {
            Some(self.load_icon(dark_path)?)
        } else {
            None
        };
        
        // Set up the icon
        self.component.set_icon_image(
            &colour_image,
            template_image.as_ref().unwrap_or(&colour_image)
        );
        
        // Set tooltip
        self.component.set_icon_tooltip(&config.tooltip);
        
        // Set up menu
        self.menu.setup_menu(&mut self.component).await?;
        
        tracing::info!("System tray icon initialized successfully");
        Ok(())
    }
    
    fn load_icon(&self, path: &std::path::Path) -> Result<Image> {
        if !path.exists() {
            return Err(TrayError::IconLoadError(format!("Icon file not found: {:?}", path)));
        }
        
        // Load image using aloe-image
        // This is a placeholder - actual implementation depends on aloe-image API
        let image = Image::new();
        // image.load_from_file(path)?;
        
        Ok(image)
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
        // Event handling will be implemented based on aloe-events integration
        // This is where we'll handle clicks, right-clicks, etc.
    }
    
    pub fn show(&mut self) {
        // Show the tray icon
    }
    
    pub fn hide(&mut self) {
        // Hide the tray icon
    }
}