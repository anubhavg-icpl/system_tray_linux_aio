use system_tray_linux_aio::{AppConfig, TrayIcon};

#[tokio::test]
async fn test_tray_creation() {
    let config = AppConfig::default();
    let tray = TrayIcon::new(config).await;
    assert!(tray.is_ok());
}

#[test]
fn test_config_creation() {
    let config = AppConfig {
        app_name: "Integration Test App".to_string(),
        tooltip: "Test tooltip".to_string(),
        ..Default::default()
    };
    
    assert_eq!(config.app_name, "Integration Test App");
    assert_eq!(config.tooltip, "Test tooltip");
    assert!(config.start_minimized);
}