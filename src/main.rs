mod backend;
mod ui;
mod tray;
use std::ops::ControlFlow;
use std::thread::spawn;
use gtk::glib;



fn main() {
    gtk::init().expect("Failed to init GTK");
    std::thread::spawn(|| {
        backend::moniter();
    });
    ui::draw();
}
