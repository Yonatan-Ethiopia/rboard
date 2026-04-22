use rdev::{listen, Event};
use rdev::Key;
use rdev::Button;
use rdev::EventType;
use rdev::EventType::KeyPress;
use std::sync::{Arc, Mutex};
use std::thread;
use arboard::Clipboard;
use std::time::Duration;
use rusqlite::{Connection, Result, params};
use std::hash::{Hash, Hasher};
use sha2::{Sha256, Digest};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::path::PathBuf;

fn checkClipboard(){
    
    let mut clip = Clipboard::new().unwrap();
    match clip.get_text(){
       Ok(text)=>{ println!("the current copied text is. {}", text); },
       Err(e)=>{ eprintln!("Error in {}",e); }, 
    }
}


pub fn moniter()->Result<(), Box<dyn std::error::Error>>{
    let data_dir = home::home_dir().map( |p| p.join(".rboardD")).unwrap();
    fs::create_dir_all(&data_dir)?;
    let db_path = data_dir.join("clip_data.db");
    let conn = Connection::open(&db_path)?;
    println!("Connected backend at {}", db_path.display());
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clip_history (
            id INTEGER PRIMARY KEY,
            content TEXT NOT NULL,
            timestamps DATETIME DEFAULT CURRENT_TIMESTAMP,
            content_hash TEXT UNIQUE
        )", (),
    )?;
    let mut clipboard = Clipboard::new().unwrap();
    let mut last_content = String::new();
    let mut content_hash = String::new();
    
    
    loop{
        if let Ok(content) = clipboard.get_text(){
            if content != last_content && !content.is_empty(){
                
                let mut hasher = Sha256::new();
                hasher.update(content.as_bytes());
                let result = hasher.finalize();
                content_hash = hex::encode(result);
                conn.execute( "DELETE  FROM clip_history WHERE content_hash = (?1)", params![content_hash])?;
                conn.execute(
                 "INSERT INTO clip_history (content, content_hash) VALUES (?1, ?2)", (&content, &content_hash)
                )?;
                conn.execute(
                    "DELETE FROM clip_history WHERE id NOT IN (
                        SELECT id FROM clip_history 
                        ORDER BY timestamps DESC 
                        LIMIT 5
                    )",
                    (),
                )?;
                last_content = content;
                println!("Clip db updated!");
                
                //println!("-------Current Top 5 Clips--------");
                //let mut stmt = conn.prepare("SELECT content FROM clip_history ORDER BY timestamps DESC LIMIT 5")?;
                
                //let content_iter = stmt.query_map([], |row|{
                    //let text: String = row.get(0)?;
                    //Ok(text) 
                //})?;
                //for (i, text) in content_iter.enumerate(){
                    //println!("{}. {}", i + 1, text?);
                //}
                //println!("-----------------------------");
                
            }
        }
        thread::sleep(Duration::from_millis(1000)); 
    } 
}

fn main() {
    //let is_ctrl = Arc::new(Mutex::new(false));
    //let is_ctrl_clone = is_ctrl.clone();
    //let handle = thread::spawn( move ||{
        //listen( move |event: Event| {
            //let mut key = is_ctrl_clone.lock().unwrap();
            //let _ = checkClipboard();
            //match event.event_type{
                //EventType::KeyPress(Key::ControlLeft)=>{ *key = true; },
                //EventType::KeyPress(Key::ControlRight)=>{ *key = true; },
                //EventType::KeyRelease(Key::ControlRight)=>{ *key = false; },
                //EventType::KeyRelease(Key::ControlLeft)=>{ *key = false; },
                //EventType::KeyPress(Key::KeyC)=>{ if *key { println!("Paste action pressed!");} },
                //_ => {}
                
            //} 
        //}).expect("Failed at listening!");
    //});
    //handle.join().unwrap();
    //moniter().unwrap();
}
