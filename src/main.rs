mod backend;
mod ui;
use std::thread;
fn main(){
    std::thread::spawn(|| {
        backend::moniter();
    });
    
    ui::draw();
}
