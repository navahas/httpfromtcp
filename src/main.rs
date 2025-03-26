use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::str;

fn stream_reader(reader: &mut BufReader<File>) -> io::Result<()> {
    let mut buffer = [0u8; 8];
    let mut current_line = String::new();

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let slice = &buffer[..bytes_read];

        match str::from_utf8(slice) {
            Ok(text) => {
                let parts: Vec<&str> = text.split('\n').collect();

                for i in 0..(parts.len() - 1) {
                    current_line.push_str(parts[i]);
                    println!("read: {}", current_line);
                    current_line.clear();
                }
                current_line.push_str(parts[parts.len() - 1]);
            }
            Err(_) => {
                println!("read (invalid UTF-8): {:?}", slice);
            }
        }
    }

    if !current_line.is_empty() {
        println!("read: {}", current_line);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open {}: {}", file_path, err);
            return Err(err);
        }
    };

    let mut reader = BufReader::new(file);
    stream_reader(&mut reader)
}
