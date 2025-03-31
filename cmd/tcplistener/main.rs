use httpfromtcp::tcp;

fn main() -> std::io::Result<()> {
    tcp::run_server()
}
