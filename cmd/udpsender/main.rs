use httpfromtcp::udp;

fn main() -> std::io::Result<()> {
    udp::run_sender()
}
