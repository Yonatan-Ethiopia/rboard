use eframe::egui;

use rusqlite::{Connection, Result, params};
use std::error::Error;

   


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
        let conn = Connection::open("./clip_data.db").expect("Failed to load the data");
        
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
        
        if self.last_update.elapsed().as_secs() >= 1 {
            self.refresh_data();
            self.last_update = std::time::Instant::now();
        }
        ui.ctx().request_repaint();
        
        for text in &self.items{
            ui.label(text);
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

