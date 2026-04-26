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

    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.push("assets/icon.png");
    let stpath = path.to_str();
    let icon = load_icon(stpath.expect("Failed to get path"));
    
    let tray_menu = Menu::new();
    let quit_item = MenuItem::with_id(MENU_ID_QUIT,"Quit", true, None);
    let show_item = MenuItem::with_id(MENU_ID_SHOW,"Open clipboard", true, None);
    
    tray_menu.append(&show_item).unwrap();
    tray_menu.append(&quit_item).unwrap();

    TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Clipboard Manager")
        .with_icon(icon)
        .build()
        .unwrap()
}

pub fn handle_tray_events(tx: Sender<TrayMessage>) {
    let menu_channel = MenuEvent::receiver();
    let tray_channel = TrayIconEvent::receiver();
    
    std::thread::spawn(move || {
        loop {
            if let Ok(event) = menu_channel.try_recv() {
                
                match event.id.0.as_str(){
                    MENU_ID_SHOW => {  let _ = tx.send(TrayMessage::ShowWindow); }
                    MENU_ID_QUIT => {
                        let _ = tx.send(TrayMessage::Quit);
                    }
                   _ => {  tx.send(TrayMessage::ShowWindow); }
                }
            }
            if let Ok(event) = tray_channel.try_recv() {
                
    match event {
        TrayIconEvent::Click {
            button: MouseButton::Left,
            ..
        } => {
            let _ = tx.send(TrayMessage::ShowWindow);
        }
        TrayIconEvent::Click {
            button: MouseButton::Right,
            ..
        } => {
        
            let _ = tx.send(TrayMessage::ShowWindow);
        }
        _ => {  let _ = tx.send(TrayMessage::ShowWindow);}
    }
}
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

fn load_icon(path : &str) -> tray_icon::Icon {
    
    let icon_data = include_bytes!("../assets/icon.png");
    //if let Some(rpath) = path{
    //let image = image::open(path).expect("Failed to open icon").into_rgba8();
    //}
    //let (width, height) = image.dimensions();
    //tray_icon::Icon::from_rgba(image.into_raw(), width, height).unwrap()
    
    let decoder = png::Decoder::new(std::io::Cursor::new(icon_data));
let mut reader = decoder.read_info().unwrap();
let mut rgba_data = vec![0; reader.info().width as usize * reader.info().height as usize * 4];
reader.next_frame(&mut rgba_data).unwrap();
let width = reader.info().width;
let height = reader.info().height;

let icon = tray_icon::Icon::from_rgba(rgba_data, width, height).unwrap();
icon
    
}
