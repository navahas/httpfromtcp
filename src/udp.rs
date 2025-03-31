use std::io::{self, BufRead, Write};
use std::net::UdpSocket;

const UDP_PORT: u16 = 42069;

pub fn run_sender() -> io::Result<()> {
    let server_address = format!("127.0.0.1:{}", UDP_PORT);
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect(&server_address)?;
    println!("Sending UDP packets to {}", server_address);

    let stdin = io::stdin();
    let mut reader = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = reader.read_line(&mut input)?;

        if bytes_read == 0 {
            break; // Ctrl+D
        }

        if let Err(e) = socket.send(input.as_bytes()) {
            eprintln!("Failed to send: {}", e);
        }
    }

    Ok(())
}
