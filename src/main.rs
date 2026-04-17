use rdev::{listen, Event};
use rdev::Key;
use rdev::Button;
use rdev::EventType;
use rdev::EventType::KeyPress;
use std::sync::{Arc, Mutex};
use std::thread;
fn checkAction( event: Event){
    
}

fn main() {
    let is_ctrl = Arc::new(Mutex::new(false));
    let is_ctrl_clone = is_ctrl.clone();
    let handle = thread::spawn( move ||{
        listen( move |event: Event| {
            let mut key = is_ctrl_clone.lock().unwrap();
            println!("Listening for action");
            match event.event_type{
                EventType::KeyPress(Key::ControlLeft)=>{ *key = true; },
                EventType::KeyPress(Key::ControlRight)=>{ *key = true; },
                EventType::KeyRelease(Key::ControlRight)=>{ *key = false; },
                EventType::KeyRelease(Key::ControlLeft)=>{ *key = false; },
                EventType::KeyPress(Key::KeyC)=>{ if *key { println!("Paste action pressed!");} },
                _ => {}
                
            } 
        }).expect("Failed at listening!");
    });
    handle.join().unwrap();
}
