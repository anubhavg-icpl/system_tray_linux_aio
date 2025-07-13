#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::env;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app_name, "System Tray App");
        assert_eq!(config.tooltip, "Click to open menu");
        assert!(config.start_minimized);
        assert!(!config.auto_start);
    }
    
    #[test]
    fn test_menu_config_default() {
        let menu_config = MenuConfig::default();
        assert!(menu_config.show_about);
        assert!(menu_config.show_settings);
        assert!(menu_config.show_quit);
        assert!(menu_config.custom_items.is_empty());
    }
    
    #[test]
    fn test_config_serialization() {
        let mut config = AppConfig::default();
        config.app_name = "Test App".to_string();
        config.menu_config.custom_items.push(MenuItem {
            label: "Test Item".to_string(),
            action: "test_action".to_string(),
            enabled: true,
            separator_after: false,
        });
        
        let serialized = toml::to_string(&config).unwrap();
        let deserialized: AppConfig = toml::from_str(&serialized).unwrap();
        
        assert_eq!(config.app_name, deserialized.app_name);
        assert_eq!(config.menu_config.custom_items.len(), deserialized.menu_config.custom_items.len());
    }
    
    #[test]
    fn test_config_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        env::set_var("HOME", temp_dir.path());
        
        let config = AppConfig {
            app_name: "Test Save App".to_string(),
            tooltip: "Test tooltip".to_string(),
            ..Default::default()
        };
        
        // Note: This test would need mock filesystem or integration test setup
        // as it relies on dirs::config_dir() which uses system paths
    }
}