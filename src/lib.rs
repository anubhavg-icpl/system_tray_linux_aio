pub mod config;
pub mod error;
pub mod menu;
pub mod tray;
pub mod contrib;
pub mod aloe_compat;

pub use config::AppConfig;
pub use error::{TrayError, Result};
pub use menu::MenuAction;
pub use tray::TrayIcon;