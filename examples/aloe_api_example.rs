/// Example demonstrating aloe-system-tray API compatibility
use anyhow::Result;
use system_tray_linux_aio::aloe_compat::{
    SystemTrayIconComponent, PopupMenu, create_icon_image, MenuEventReceiver
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();
    
    info!("Starting aloe-system-tray compatible example");
    
    // Initialize GTK (required for Linux)
    gtk::init()?;
    
    // Create system tray component following aloe API
    let mut tray_component = SystemTrayIconComponent::new();
    
    // Create icon images
    let colour_image = create_icon_image();
    let template_image = colour_image; // Same for simplicity
    
    // Setup icon following aloe API example
    setup_icon(&mut tray_component, &colour_image, &template_image);
    
    // Create menu
    let mut menu = PopupMenu::new();
    menu.add_item(1, "About", true, false)?;
    menu.add_item(2, "Settings", true, false)?;
    menu.add_separator()?;
    menu.add_item(3, "Quit", true, false)?;
    
    // Initialize with menu
    tray_component.initialize_tray(&menu);
    
    info!("System tray icon created with aloe-compatible API!");
    info!("Right-click the icon to see the menu");
    
    // Event loop
    let mut running = true;
    while running {
        // Check for menu events
        if let Some(menu_id) = MenuEventReceiver::try_recv() {
            match menu_id {
                3 => {
                    info!("Quit selected");
                    running = false;
                }
                1 => info!("About selected"),
                2 => info!("Settings selected"),
                _ => {}
            }
        }
        
        // GTK event processing
        while gtk::events_pending() {
            gtk::main_iteration();
        }
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
    
    Ok(())
}

/// Setup icon following aloe-system-tray example pattern
fn setup_icon(
    component: &mut SystemTrayIconComponent,
    colour_image: &system_tray_linux_aio::aloe_compat::Image,
    template_image: &system_tray_linux_aio::aloe_compat::Image
) {
    component.set_icon_image(&colour_image, &template_image);
    component.set_icon_tooltip("Application running");
    component.set_highlighted(true);
}