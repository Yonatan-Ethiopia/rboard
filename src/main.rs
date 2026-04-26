mod backend;
mod ui;
mod tray;
use std::ops::ControlFlow;
use std::thread::spawn;
use gtk::glib;



fn main() {
    // 1. Initialize GTK but DO NOT call gtk::main()
    gtk::init().expect("Failed to init GTK");

    // 2. Start the backend
    std::thread::spawn(|| {
        backend::moniter();
    });

    // 3. Just run the UI normally
    ui::draw();
}
