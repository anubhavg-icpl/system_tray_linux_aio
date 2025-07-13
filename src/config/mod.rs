use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::Result;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app_name: String,
    pub tooltip: String,
    pub icon_path: PathBuf,
    pub dark_icon_path: Option<PathBuf>,
    pub start_minimized: bool,
    pub auto_start: bool,
    pub menu_config: MenuConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuConfig {
    pub show_about: bool,
    pub show_settings: bool,
    pub show_quit: bool,
    pub custom_items: Vec<MenuItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub label: String,
    pub action: String,
    pub enabled: bool,
    pub separator_after: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "System Tray App".to_string(),
            tooltip: "Click to open menu".to_string(),
            icon_path: PathBuf::from("assets/icons/default.png"),
            dark_icon_path: None,
            start_minimized: true,
            auto_start: false,
            menu_config: MenuConfig::default(),
        }
    }
}

impl Default for MenuConfig {
    fn default() -> Self {
        Self {
            show_about: true,
            show_settings: true,
            show_quit: true,
            custom_items: vec![],
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::error::TrayError::ConfigError("Could not find config directory".into()))?;
        
        let config_path = config_dir.join("system_tray_linux_aio").join("config.toml");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: AppConfig = toml::from_str(&content)
                .map_err(|e| crate::error::TrayError::ConfigError(e.to_string()))?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }
    
    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| crate::error::TrayError::ConfigError("Could not find config directory".into()))?;
        
        let app_config_dir = config_dir.join("system_tray_linux_aio");
        std::fs::create_dir_all(&app_config_dir)?;
        
        let config_path = app_config_dir.join("config.toml");
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::error::TrayError::ConfigError(e.to_string()))?;
        
        std::fs::write(config_path, content)?;
        Ok(())
    }
}