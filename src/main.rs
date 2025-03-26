use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::str;

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

    let mut buffer = [0u8; 8];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        let slice = &buffer[..bytes_read];

        match str::from_utf8(slice) {
            Ok(text) => println!("read: {}", text),
            Err(_) => println!("read (invalid UTF-8): {:?}", slice),
        }
    }

    Ok(())
}
