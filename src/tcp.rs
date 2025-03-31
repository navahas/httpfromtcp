use std::io;
use std::net::TcpListener;
use std::thread;

use crate::connection::get_lines_channel;

const TCP_PORT: u16 = 42069;

pub fn run_server() -> io::Result<()> {
    let server_address = format!("127.0.0.1:{}", TCP_PORT);
    let listener = TcpListener::bind(&server_address)?;
    println!("Listening on {}...", server_address);

    for incoming in listener.incoming() {
        let stream = incoming?;

        thread::spawn(move || {
            let line_rx = get_lines_channel(stream);
            for line in line_rx {
                println!("read: {}", line);
            }
            println!("Client disconnected.\n");
        });
    }

    Ok(())
}
