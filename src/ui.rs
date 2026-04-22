use eframe::egui;
use egui::{RichText, Color32, Vec2};
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use arboard::Clipboard;

   


// Minimal example: define struct, implement Default and eframe::App
struct MyApp { items: Vec<String>, last_update: std::time::Instant, }
impl Default for MyApp {
    fn default() -> Self { let mut app = Self{
        items: Vec::new(),
        last_update: std::time::Instant::now(), 
        };
        app.refresh_data();
        app
        }
}
impl MyApp {
    fn refresh_data(&mut self)  -> Result<(), Box<dyn std::error::Error>> {
        let data_dir = home::home_dir().map( |p| p.join(".rboardD")).unwrap();
    fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join("clip_data.db");
    let conn = Connection::open(&db_path).expect("Failed to load the data");
    
        
        let mut stmt = conn.prepare("SELECT content FROM clip_history ORDER BY timestamps DESC LIMIT 5").expect("SQL failed to retrive data");
        
        let content_iter = stmt.query_map([], |row|{
                    let text: String = row.get(0)?;
                    Ok(text) 
                })?;
        self.items = content_iter.map(|item| item.unwrap()).collect();
        
        Ok(())
    }
}
impl eframe::App for MyApp {
    // This is the new required method for drawing the UI
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // You no longer need to call egui::CentralPanel::default().show(...) 
        // if you want to draw directly in the provided 'ui' area.
        ui.heading("Clipboard");
        ui.spacing_mut().item_spacing = Vec2::new(15.0, 10.0);
        if self.last_update.elapsed().as_secs() >= 1 {
            self.refresh_data();
            self.last_update = std::time::Instant::now();
        }
        ui.ctx().request_repaint();
        
        for text in &self.items{
            ui.add_space(20.0);
            ui.horizontal( |ui| {
                ui.add_space(5.0);
                ui.label(
                    RichText::new(text)
                    .color(Color32::from_rgb(100, 200, 255))
                    .strong()
                );
                let button = egui::Button::new(
                    RichText::new("Copy")
                    .color(Color32::GREEN)
                    ).fill(Color32::from_gray(40));
                if ui.add(button).clicked() { 
                     let mut clipboard = Clipboard::new().unwrap();
                     clipboard.set_text(text).unwrap();
                }
            });
            
        }

    }
}

pub fn draw()  -> eframe::Result {
    let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_decorations(true)     // Removes the title bar/close buttons
        .with_always_on_top()        // Keeps it above other windows
        .with_inner_size([300.0, 400.0])
        .with_transparent(false),     // If you want a rounded/transparent look
    ..Default::default()
};
    eframe::run_native("App", options, 
        Box::new(|_| Ok(Box::new(MyApp::default()))));
        
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
fn main() //-> eframe::Result 
{
    //let options = eframe::NativeOptions {
    //viewport: egui::ViewportBuilder::default()
        //.with_decorations(true)     // Removes the title bar/close buttons
        //.with_always_on_top()        // Keeps it above other windows
        //.with_inner_size([300.0, 400.0])
        //.with_transparent(false),     // If you want a rounded/transparent look
    //..Default::default()
//};
    //eframe::run_native("App", options, 
        //Box::new(|_| Ok(Box::new(MyApp::default()))));
        
    //loop {
        //std::thread::sleep(std::time::Duration::from_secs(1));
    //}
}

