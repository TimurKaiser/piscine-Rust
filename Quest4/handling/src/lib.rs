use std::fs::{File, OpenOptions};
use std::io::Write;

pub fn open_or_create(file: &str, content: &str) {
    let mut file = match OpenOptions::new()
        .write(true)      
        .create(true)      
        .open(file)        
    {
        Ok(f) => f,
        Err(e) => panic!("Failed to open or create file: {}", e), 
    };

    if let Err(e) = file.write_all(content.as_bytes()) {
        panic!("Failed to write content to file: {}", e);
    }
}
