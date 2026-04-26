use tray_icon::{
    menu::{Menu, MenuEvent, MenuItem},
    TrayIcon, TrayIconBuilder, 
    TrayIconEvent, MouseButton, MouseButtonState
};
use std::sync::mpsc::Sender;



const MENU_ID_SHOW: &str = "show_app";
const MENU_ID_QUIT: &str = "quit_app";

pub enum TrayMessage{
    ShowWindow,
    Quit,
}

pub fn create_tray() -> TrayIcon {
    // 1. Load your icon (reuse the load_icon helper from before)
    let icon = load_icon("/home/yonatan/rboard/src/icon(1).png");

    // 2. Create a Menu
    let tray_menu = Menu::new();
    let quit_item = MenuItem::with_id(MENU_ID_SHOW,"Open clipboard", true, None);
    let show_item = MenuItem::with_id(MENU_ID_QUIT,"Quit", true, None);
    
    tray_menu.append(&show_item).unwrap();
    tray_menu.append(&quit_item).unwrap();

    // 3. Build the Tray
    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Clipboard Manager")
        .with_icon(icon)
        .build()
        .unwrap()
}

// Separate function to handle events so it doesn't block main
pub fn handle_tray_events(tx: Sender<TrayMessage>) {
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = menu_channel.try_recv() {
                println!("Tray event: {:?}", event);
                match event.id.0.as_str(){
                    MENU_ID_SHOW => { println!("TRay clicked!!!"); let _ = tx.send(TrayMessage::ShowWindow); }
                    MENU_ID_QUIT => {
                        let _ = tx.send(TrayMessage::Quit);
                    }
                   _ => { println!("TRay clicked!!!"); tx.send(TrayMessage::ShowWindow); }
                }
            }
            if let Ok(event) = tray_channel.try_recv() {
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            ..
        } => {
            // This triggers when the Left button is clicked
            let _ = tx.send(TrayMessage::ShowWindow);
        }
        TrayIconEvent::Click {
            button: MouseButton::Right,
            ..
        } => {
            // This triggers when the Left button is clicked
            let _ = tx.send(TrayMessage::ShowWindow);
        }
        _ => {let _ = tx.send(TrayMessage::ShowWindow);} // Ignore moves, right-clicks, etc.
    }
}
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

fn load_icon(path: &str) -> tray_icon::Icon {
    let image = image::open(path).expect("Failed to open icon").into_rgba8();
    let (width, height) = image.dimensions();
    tray_icon::Icon::from_rgba(image.into_raw(), width, height).unwrap()
}
