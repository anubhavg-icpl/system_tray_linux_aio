use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("Failed to initialize system tray: {0}")]
    InitializationError(String),
    
    #[error("Failed to load icon: {0}")]
    IconLoadError(String),
    
    #[error("Menu operation failed: {0}")]
    MenuError(String),
    
    #[error("Event handling error: {0}")]
    EventError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, TrayError>;