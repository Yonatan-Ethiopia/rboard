mod backend;
mod tray;
mod ui;
use gtk::glib;
use std::ops::ControlFlow;
use std::thread::spawn;

fn main() {
    gtk::init().expect("Failed to init GTK");
    std::thread::spawn(|| {
        backend::moniter();
    });
    ui::draw();
}
