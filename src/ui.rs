use eframe::egui;
use egui::{RichText, Color32, Vec2};
use rusqlite::{Connection, Result, params};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use arboard::Clipboard;
use std::sync::Arc;
use sha2::{Sha256, Digest};
use std::sync::mpsc::{channel, Receiver};
use crate::tray::{TrayMessage, self};


struct MyApp { items: Vec<String>, 
				last_update: std::time::Instant,
				rx: Receiver<TrayMessage>,
				visible: bool, }
impl MyApp {
    pub fn new(rx: std::sync::mpsc::Receiver<crate::tray::TrayMessage>) -> Self {
        let mut app = Self {
            rx,              
            visible: true,  
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                TrayMessage::ShowWindow => {
                    self.visible = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
                    },
                TrayMessage::Quit => std::process::exit(0),
            }
        }

        
        
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.visible = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
            ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
            
            println!("the window is hidden, is it visible: {}", self.visible);
        }
		ctx.send_viewport_cmd(egui::ViewportCommand::Visible(self.visible));

    }
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.add_space(5.0);
        ui.heading(" Clipboard");
        ui.add_space(5.0);
        ui.spacing_mut().item_spacing = Vec2::new(15.0, 10.0);
        if self.last_update.elapsed().as_secs() >= 1 {
            self.refresh_data();
            self.last_update = std::time::Instant::now();
        }
        ui.ctx().request_repaint();
        
        for text in &self.items{
            ui.add_space(5.0);
            ui.horizontal( |ui| {
                ui.add_space(5.0);
                ui.allocate_ui(egui::vec2(100.0, 20.0), |ui| {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.add(
            egui::Label::new(
                RichText::new(text)
                    .color(Color32::from_rgb(100, 200, 255))
                    .strong()
            )
            .truncate()
        );
    });
});
                let button = egui::Button::new(
                    RichText::new("Copy").size(11.0).strong()
                    .color(Color32::from_gray(140))
                    );
                
ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
    ui.add_space(5.0); 
    
    ui.scope(|ui| {
       
        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = egui::Color32::from_gray(40);
        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;
        
        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(60, 60, 80); 
        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;
        
        ui.style_mut().visuals.widgets.active.weak_bg_fill = egui::Color32::from_rgb(110, 119, 128);
        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

        let button = egui::Button::new(
            RichText::new("Copy")
                .size(14.0)
                .strong()
                .color(Color32::from_gray(140))
        );
        

        if ui.add(button).clicked() { 
            let mut clipboard = Clipboard::new().unwrap();
            clipboard.set_text(text).unwrap();
        }
        ui.add_space(2.0);
       
        let del_btn = egui::Button::new(
            RichText::new("Delete") 
                .size(11.0)
                .color(egui::Color32::from_rgb(200, 100, 100)) 
        );
        if ui.add(del_btn).clicked() {
            let data_dir = home::home_dir().map( |p| p.join(".rboardD")).unwrap();
    fs::create_dir_all(&data_dir).unwrap();
    let db_path = data_dir.join("clip_data.db");
    let conn = Connection::open(&db_path).unwrap();
            
            conn.execute( "DELETE  FROM clip_history WHERE content = (?1)", params![text]).unwrap();
            println!("Content deleted.");
            
        }
    });
});
                
            });
            ui.vertical_centered(|ui| {
            
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(ui.available_width() * 0.8, 1.0), 
        egui::Sense::hover()
    );
    
    ui.painter().rect_filled(
        rect, 
        1.0, 
        egui::Color32::from_gray(60)
    );
});
            
        }

    }
}

pub fn draw()  -> eframe::Result {
	let (tx, rx) = channel();
	let _tray = tray::create_tray();
	tray::handle_tray_events(tx);
    let options = eframe::NativeOptions {
    viewport: egui::ViewportBuilder::default()
        .with_decorations(true)
        .with_window_type(egui::WindowType::Utility)     
        .with_always_on_top()        
        .with_inner_size([300.0, 400.0])
        .with_transparent(false),    
    ..Default::default()
};
    eframe::run_native(
    "Rboard",
    options,
    Box::new(|cc| {
        set_up_font(&cc.egui_ctx);
        
        Ok(Box::new(MyApp::new(rx)))
    }),
).unwrap();
        
    loop {          
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
pub fn set_up_font(ctx: &egui::Context){
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "jet".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/JetBrainsMono-VariableFont_wght.ttf"
        ))),
    );
    
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "jet".to_owned());
    ctx.set_fonts(fonts);
}
fn main() 
{

}

