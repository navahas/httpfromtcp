use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];
    let message_content = fs::read(file_path)
        .expect("Failed to read file");

    println!("Total bytes: {}", message_content.len());

    for chunk in message_content.chunks(8) {
        let text = std::str::from_utf8(chunk).unwrap_or("Invalid utf-8");
        println!("read: {}", text);
    }

    // println!("{:?}", message_content);
}
