use std::io;
use std::net::TcpListener;
use std::thread;

use crate::connection::get_lines_channel;

const TCP_PORT: u16 = 42069;

pub fn run_server() -> io::Result<()> {
    let address = format!("127.0.0.1:{}", TCP_PORT);
    let listener = TcpListener::bind(&address)?;
    println!("Listening on {}...", address);

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
