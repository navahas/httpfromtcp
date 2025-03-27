use std::fs::File;
use std::io::{self, BufReader, Read};
use std::str;
use std::sync::mpsc::{self, Receiver};
use std::thread;

fn get_lines_channel(file: File) -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        let mut reader = BufReader::new(file);
        let mut buffer = [0u8; 8];
        let mut current_line = String::new();

        loop {
            let bytes_read = match reader.read(&mut buffer) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("Error reading: {}", e);
                    break;
                }
            };

            if bytes_read == 0 {
                break;
            }

            let slice = &buffer[..bytes_read];

            let chunk = match str::from_utf8(slice) {
                Ok(text) => text,
                Err(_) => {
                    eprintln!("Invalid UTF-8 chunk: {:?}", slice);
                    continue; 
                }
            };

            let parts = chunk.split('\n').collect::<Vec<&str>>();

            for i in 0..(parts.len() - 1) {
                current_line.push_str(parts[i]);
                if let Err(e) = tx.send(current_line.clone()) {
                    eprintln!("Error sending line: {}", e);
                    return; // If the receiver is gone, we stop.
                }
                current_line.clear();
            }

            current_line.push_str(parts[parts.len() - 1]);
        }

        if !current_line.is_empty() {
            let _ = tx.send(current_line);
        }
    });
    rx
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("Usage: <program> <file_path>");

    let file = File::open(path)?;

    let line_receiver = get_lines_channel(file);

    for line in line_receiver {
        println!("read: {}", line);
    }

    Ok(())
}
