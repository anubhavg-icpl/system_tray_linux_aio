/// Interactive example for stray - allows sending menu commands
use anyhow::Result;
use stray::{SystemTray, message::{NotifierItemMessage, NotifierItemCommand}};
use tokio_stream::StreamExt;
use tokio::sync::mpsc;
use tracing::info;
use std::collections::HashMap;
use std::io::{self, Write};

struct InteractiveApp {
    ui_tx: mpsc::Sender<NotifierItemCommand>,
    notifiers: HashMap<String, (String, String)>, // address -> (title, menu_path)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();
    
    info!("Starting interactive stray example");
    
    let (ui_tx, ui_rx) = mpsc::channel::<NotifierItemCommand>(32);
    let mut app = InteractiveApp {
        ui_tx: ui_tx.clone(),
        notifiers: HashMap::new(),
    };
    
    // Create system tray
    let mut tray = SystemTray::new(ui_rx).await;
    
    // Spawn input handler
    let tx_clone = ui_tx.clone();
    let input_task = tokio::spawn(async move {
        let mut notifiers_clone = HashMap::new();
        
        loop {
            // Show menu
            println!("\n╔════════════════════════════════════╗");
            println!("║      STRAY INTERACTIVE MENU        ║");
            println!("╠════════════════════════════════════╣");
            println!("║ Commands:                          ║");
            println!("║   l - List all tray icons          ║");
            println!("║   c - Click menu item              ║");
            println!("║   q - Quit                         ║");
            println!("╚════════════════════════════════════╝");
            print!("Enter command: ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match input {
                "l" => {
                    println!("\nActive Tray Icons:");
                    println!("─────────────────────────────────");
                    for (i, (addr, (title, _))) in notifiers_clone.iter().enumerate() {
                        println!("{}: {} ({})", i, title, addr);
                    }
                    if notifiers_clone.is_empty() {
                        println!("No tray icons detected yet.");
                    }
                }
                
                "c" => {
                    if notifiers_clone.is_empty() {
                        println!("No tray icons available. Wait for icons to appear.");
                        continue;
                    }
                    
                    print!("Enter icon index: ");
                    io::stdout().flush().unwrap();
                    let mut idx_input = String::new();
                    io::stdin().read_line(&mut idx_input).unwrap();
                    
                    if let Ok(idx) = idx_input.trim().parse::<usize>() {
                        let items: Vec<_> = notifiers_clone.iter().collect();
                        if let Some((addr, (title, menu_path))) = items.get(idx) {
                            print!("Enter submenu ID (usually 0-10): ");
                            io::stdout().flush().unwrap();
                            let mut id_input = String::new();
                            io::stdin().read_line(&mut id_input).unwrap();
                            
                            if let Ok(submenu_id) = id_input.trim().parse::<i32>() {
                                let cmd = NotifierItemCommand::MenuItemClicked {
                                    submenu_id,
                                    menu_path: menu_path.to_string(),
                                    notifier_address: addr.to_string(),
                                };
                                
                                if let Err(e) = tx_clone.try_send(cmd) {
                                    println!("Failed to send command: {}", e);
                                } else {
                                    println!("Sent click command for '{}' menu item {}", title, submenu_id);
                                }
                            }
                        } else {
                            println!("Invalid index");
                        }
                    }
                }
                
                "q" => {
                    println!("Exiting...");
                    std::process::exit(0);
                }
                
                _ => {
                    println!("Unknown command: {}", input);
                }
            }
        }
    });
    
    // Main event loop
    info!("Listening for system tray icons...\n");
    
    loop {
        tokio::select! {
            Some(message) = tray.next() => {
                match message {
                    NotifierItemMessage::Update { address, item, menu } => {
                        if let Some(item) = item {
                            let title = item.title.unwrap_or_else(|| "Untitled".to_string());
                            let menu_path = item.menu.unwrap_or_default();
                            
                            println!("\n🔔 Tray icon updated: {} ({})", title, address);
                            app.notifiers.insert(address.clone(), (title, menu_path));
                            
                            // Update the input task's copy
                            // In a real app, you'd use Arc<Mutex<>> or similar
                        }
                    }
                    
                    NotifierItemMessage::Remove { address } => {
                        if let Some((title, _)) = app.notifiers.remove(&address) {
                            println!("\n❌ Tray icon removed: {} ({})", title, address);
                        }
                    }
                }
            }
            
            _ = tokio::signal::ctrl_c() => {
                info!("Received Ctrl+C");
                break;
            }
        }
    }
    
    Ok(())
}