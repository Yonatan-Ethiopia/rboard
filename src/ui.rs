use crate::tray::{self, TrayMessage};
use arboard::Clipboard;
use eframe::egui;
use egui::{Color32, RichText, Vec2};
use rusqlite::{params, Connection, Result};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver};
use std::sync::Arc;

struct MyApp {
    items: Vec<String>,
    last_update: std::time::Instant,
    rx: Receiver<TrayMessage>,
    visible: bool,
}
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
    fn refresh_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let data_dir = home::home_dir().map(|p| p.join(".rboardD")).unwrap();
        fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("clip_data.db");
        let conn = Connection::open(&db_path).expect("Failed to load the data");

        let mut stmt = conn
            .prepare("SELECT content FROM clip_history ORDER BY timestamps DESC LIMIT 5")
            .expect("SQL failed to retrive data");

        let content_iter = stmt.query_map([], |row| {
            let text: String = row.get(0)?;
            Ok(text)
        })?;
        self.items = content_iter.map(|item| item.unwrap()).collect();

        Ok(())
    }
}
impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Rgba::TRANSPARENT.to_array()
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::TRANSPARENT))
            .show(ctx, |ui| {
                let app_container = egui::Frame::none()
                    .fill(egui::Color32::from_rgb(25, 25, 35))
                    .corner_radius(12)
                    .inner_margin(egui::Margin::same(0));

                app_container.show(ui, |ui| {
                    ui.expand_to_include_rect(ui.max_rect());

                    let bar_height = 40.0;
                    let full_width = ui.max_rect().width();

                    let (bar_rect, _) = ui.allocate_exact_size(
                        egui::vec2(full_width, bar_height),
                        egui::Sense::hover(),
                    );

                    let rounding = egui::CornerRadius {
                        nw: 12,
                        ne: 12,
                        sw: 0,
                        se: 0,
                    };
                    ui.painter().rect_filled(
                        bar_rect,
                        rounding,
                        egui::Color32::from_rgb(30, 30, 40),
                    );
                    ui.visuals_mut().widgets.hovered.bg_stroke = egui::Stroke::NONE;
                    ui.visuals_mut().widgets.active.bg_stroke = egui::Stroke::NONE;
                    ui.visuals_mut().widgets.hovered.expansion = 0.0;
                    ui.visuals_mut().widgets.active.expansion = 0.0;

                    let drag_response =
                        ui.interact(bar_rect, ui.id().with("drag_bar"), egui::Sense::click());
                    if drag_response.is_pointer_button_down_on() {
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
                    }
                    let mut child_ui = ui.child_ui(
                        bar_rect,
                        egui::Layout::right_to_left(egui::Align::Center),
                        None,
                    );

                    child_ui.add_space(4.0);
                    child_ui.spacing_mut().item_spacing.x = 9.0;

                    let button_size = egui::vec2(18.0, 18.0);
                    let make_custom_btn = |text: &str, fill_color: egui::Color32| {
                        egui::Button::new(text)
                            .min_size(button_size)
                            .rounding(9.0)
                            .fill(fill_color)
                            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 85)))
                    };

                    let paint_circle_btn =
                        |ui: &mut egui::Ui, fill: egui::Color32| -> egui::Response {
                            let (rect, response) =
                                ui.allocate_exact_size(button_size, egui::Sense::click());
                            let center = rect.center();
                            let radius = 9.0;
                            // Background circle
                            ui.painter().circle_filled(center, radius, fill);
                            // Border
                            ui.painter().circle_stroke(
                                center,
                                radius,
                                egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 85)),
                            );
                            response
                        };

                    let draw_minimize = |ui: &mut egui::Ui| {
                        let (rect, response) =
                            ui.allocate_exact_size(button_size, egui::Sense::click());
                        let center = rect.center();
                        ui.painter().circle_filled(
                            center,
                            9.0,
                            egui::Color32::from_rgb(45, 45, 55),
                        );
                        ui.painter().circle_stroke(
                            center,
                            9.0,
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 85)),
                        );
                        ui.painter().line_segment(
                            [center - egui::vec2(4.0, 0.0), center + egui::vec2(4.0, 0.0)],
                            egui::Stroke::new(1.5, egui::Color32::from_gray(180)),
                        );
                        response
                    };

                    let draw_maximize = |ui: &mut egui::Ui| {
                        let (rect, response) =
                            ui.allocate_exact_size(button_size, egui::Sense::click());
                        let center = rect.center();
                        ui.painter().circle_filled(
                            center,
                            9.0,
                            egui::Color32::from_rgb(45, 45, 55),
                        );
                        ui.painter().circle_stroke(
                            center,
                            9.0,
                            egui::Stroke::new(1.0, egui::Color32::from_rgb(70, 70, 85)),
                        );
                        let sq = egui::Rect::from_center_size(center, egui::vec2(8.0, 8.0));
                        ui.painter().rect_stroke(
                            sq,
                            1.0,
                            egui::Stroke::new(1.5, egui::Color32::from_gray(180)),
                            egui::StrokeKind::Middle,
                        );
                        response
                    };

                    let close_btn = make_custom_btn("❌", egui::Color32::from_rgb(180, 50, 50));
                    if child_ui.add(close_btn).clicked() {
                        self.visible = false;
                        ctx.send_viewport_cmd(egui::ViewportCommand::Visible(false));
                    }

                    if draw_maximize(&mut child_ui).clicked() {
                        let is_maximized =
                            child_ui.input(|i| i.viewport().maximized.unwrap_or(false));
                        child_ui
                            .ctx()
                            .send_viewport_cmd(egui::ViewportCommand::Maximized(!is_maximized));
                    }

                    if draw_minimize(&mut child_ui).clicked() {
                        child_ui
                            .ctx()
                            .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                    }
                });
            });

        if let Ok(msg) = self.rx.try_recv() {
            match msg {
                TrayMessage::ShowWindow => {
                    self.visible = true;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Visible(true));
                }
                TrayMessage::Quit => std::process::exit(0),
            }
        }
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

        for text in &self.items {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add_space(5.0);
                ui.allocate_ui(egui::vec2(100.0, 20.0), |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.add(
                            egui::Label::new(
                                RichText::new(text)
                                    .color(Color32::from_rgb(100, 200, 255))
                                    .strong(),
                            )
                            .truncate(),
                        );
                    });
                });
                let button = egui::Button::new(
                    RichText::new("Copy")
                        .size(11.0)
                        .strong()
                        .color(Color32::from_gray(140)),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(5.0);

                    ui.scope(|ui| {
                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                            egui::Color32::from_gray(40);
                        ui.style_mut().visuals.widgets.inactive.bg_stroke = egui::Stroke::NONE;

                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                            egui::Color32::from_rgb(60, 60, 80);
                        ui.style_mut().visuals.widgets.hovered.bg_stroke = egui::Stroke::NONE;

                        ui.style_mut().visuals.widgets.active.weak_bg_fill =
                            egui::Color32::from_rgb(110, 119, 128);
                        ui.style_mut().visuals.widgets.active.bg_stroke = egui::Stroke::NONE;

                        let button = egui::Button::new(
                            RichText::new("Copy")
                                .size(14.0)
                                .strong()
                                .color(Color32::from_gray(140)),
                        );

                        if ui.add(button).clicked() {
                            let mut clipboard = Clipboard::new().unwrap();
                            clipboard.set_text(text).unwrap();
                        }
                        ui.add_space(2.0);

                        let del_btn = egui::Button::new(
                            RichText::new("Delete")
                                .size(11.0)
                                .color(egui::Color32::from_rgb(200, 100, 100)),
                        );
                        if ui.add(del_btn).clicked() {
                            let data_dir = home::home_dir().map(|p| p.join(".rboardD")).unwrap();
                            fs::create_dir_all(&data_dir).unwrap();
                            let db_path = data_dir.join("clip_data.db");
                            let conn = Connection::open(&db_path).unwrap();

                            conn.execute(
                                "DELETE  FROM clip_history WHERE content = (?1)",
                                params![text],
                            )
                            .unwrap();
                            println!("Content deleted.");
                        }
                    });
                });
            });
            ui.vertical_centered(|ui| {
                let (rect, _) = ui.allocate_exact_size(
                    egui::vec2(ui.available_width() * 0.8, 1.0),
                    egui::Sense::hover(),
                );

                ui.painter()
                    .rect_filled(rect, 1.0, egui::Color32::from_gray(60));
            });
        }
    }
}

pub fn draw() -> eframe::Result {
    let (tx, rx) = channel();
    let _tray = tray::create_tray();
    tray::handle_tray_events(tx);
    let icon_data = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory(icon_data).expect("Failed to load icon");
    let image_rgba = image.to_rgba8();
    let (width, height) = image_rgba.dimensions();

    let icon = egui::IconData {
        rgba: image_rgba.into_raw(),
        width,
        height,
    };
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_transparent(true)
            .with_resizable(true)
            .with_always_on_top()
            .with_icon(std::sync::Arc::new(icon))
            .with_inner_size([300.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rboard",
        options,
        Box::new(|cc| {
            set_up_font(&cc.egui_ctx);

            Ok(Box::new(MyApp::new(rx)))
        }),
    )
    .unwrap();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
pub fn set_up_font(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "jet".to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(
            "../assets/JetBrainsMono-VariableFont_wght.ttf"
        ))),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "jet".to_owned());
    ctx.set_fonts(fonts);
}
fn main() {}
