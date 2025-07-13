/// Compatibility layer that implements aloe-system-tray API using tray-icon
/// This allows using the aloe API pattern while avoiding build issues on Linux

use anyhow::Result;
use tray_icon::{
    Icon, TrayIcon as TrayIconLib, TrayIconBuilder,
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem},
};
use image::{RgbaImage, ImageBuffer, Rgba};
use std::sync::{Arc, Mutex};

/// Image type matching aloe API
pub struct Image {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            data: vec![0; size],
            width,
            height,
        }
    }
    
    pub fn from_rgba(data: Vec<u8>, width: u32, height: u32) -> Self {
        Self { data, width, height }
    }
    
    pub fn as_rgba_image(&self) -> RgbaImage {
        ImageBuffer::from_raw(self.width, self.height, self.data.clone())
            .expect("Failed to create image from data")
    }
}

/// PopupMenu type matching aloe API
pub struct PopupMenu {
    menu: Menu,
    items: Vec<(i32, String)>,
}

impl PopupMenu {
    pub fn new() -> Self {
        Self {
            menu: Menu::new(),
            items: Vec::new(),
        }
    }
    
    pub fn add_item(&mut self, id: i32, text: &str, enabled: bool, _checked: bool) -> Result<()> {
        let item = MenuItem::new(text, enabled, None);
        self.menu.append(&item)?;
        self.items.push((id, text.to_string()));
        Ok(())
    }
    
    pub fn add_separator(&mut self) -> Result<()> {
        self.menu.append(&PredefinedMenuItem::separator())?;
        Ok(())
    }
    
    pub fn get_menu(&self) -> &Menu {
        &self.menu
    }
}

/// SystemTrayIconComponent matching aloe-system-tray API
pub struct SystemTrayIconComponent {
    tray: Option<TrayIconLib>,
    icon: Option<Icon>,
    tooltip: String,
    highlighted: bool,
    _menu: Option<Arc<Mutex<PopupMenu>>>,
}

impl SystemTrayIconComponent {
    pub fn new() -> Self {
        Self {
            tray: None,
            icon: None,
            tooltip: String::new(),
            highlighted: false,
            _menu: None,
        }
    }
    
    /// Set the icon image (aloe API compatibility)
    pub fn set_icon_image(&mut self, colour_image: &Image, _template_image: &Image) {
        // Convert aloe Image to tray-icon Icon
        let rgba_image = colour_image.as_rgba_image();
        let rgba_data = rgba_image.into_raw();
        
        if let Ok(icon) = Icon::from_rgba(rgba_data, colour_image.width, colour_image.height) {
            self.icon = Some(icon.clone());
            
            // Update existing tray or create new one
            if let Some(tray) = &mut self.tray {
                let _ = tray.set_icon(Some(icon));
            }
        }
    }
    
    /// Set tooltip text
    pub fn set_icon_tooltip(&mut self, tooltip: &str) {
        self.tooltip = tooltip.to_string();
        if let Some(tray) = &mut self.tray {
            let _ = tray.set_tooltip(Some(tooltip));
        }
    }
    
    /// Set highlighted state (note: not directly supported by tray-icon)
    pub fn set_highlighted(&mut self, should_highlight: bool) {
        self.highlighted = should_highlight;
        // tray-icon doesn't support highlighting, but we track the state
    }
    
    /// Show info bubble (note: not directly supported by tray-icon)
    pub fn show_info_bubble(&mut self, _title: &str, _content: &str) {
        // tray-icon doesn't support info bubbles on Linux
        // This would need platform-specific implementation
    }
    
    /// Hide info bubble
    pub fn hide_info_bubble(&mut self) {
        // No-op since info bubbles aren't supported
    }
    
    /// Show dropdown menu
    pub fn show_dropdown_menu(&mut self, menu: &PopupMenu) {
        // Menu is shown automatically on right-click with tray-icon
        // We just ensure it's set
        if self.tray.is_none() {
            self.initialize_tray(menu);
        }
    }
    
    /// Initialize the tray icon
    pub fn initialize_tray(&mut self, menu: &PopupMenu) {
        if let Some(icon) = &self.icon {
            let tray = TrayIconBuilder::new()
                .with_icon(icon.clone())
                .with_tooltip(&self.tooltip)
                .with_menu(Box::new(menu.get_menu().clone()))
                .build();
                
            if let Ok(tray) = tray {
                self.tray = Some(tray);
            }
        }
    }
    
    /// Get the current state
    pub fn is_visible(&self) -> bool {
        self.tray.is_some()
    }
}

/// Helper to create images similar to aloe-graphics
pub fn create_icon_image() -> Image {
    let mut img = RgbaImage::new(32, 32);
    
    // Fill with blue background
    let blue = Rgba([52, 152, 219, 255]);
    let white = Rgba([255, 255, 255, 255]);
    
    for pixel in img.pixels_mut() {
        *pixel = blue;
    }
    
    // Draw white circle
    let center = 16;
    let radius = 12;
    
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = x as i32 - center;
        let dy = y as i32 - center;
        if dx * dx + dy * dy <= radius * radius {
            *pixel = white;
        }
    }
    
    // Draw blue "A" in center
    for y in 8..24 {
        for x in 8..24 {
            if (x == 15 && y >= 10 && y <= 20) || // Left line
               (x == 17 && y >= 10 && y <= 20) || // Right line
               (y == 10 && x >= 15 && x <= 17) || // Top
               (y == 15 && x >= 14 && x <= 18) {  // Middle bar
                img.put_pixel(x, y, blue);
            }
        }
    }
    
    let raw = img.into_raw();
    Image::from_rgba(raw, 32, 32)
}

/// Event handling compatibility
pub struct MenuEventReceiver;

impl MenuEventReceiver {
    pub fn try_recv() -> Option<i32> {
        // Check for menu events from tray-icon
        if let Ok(_event) = MenuEvent::receiver().try_recv() {
            // Convert menu event to ID (simplified)
            Some(0) // Would need proper ID mapping
        } else {
            None
        }
    }
}