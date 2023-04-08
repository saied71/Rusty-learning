fn main() {
    addr = String::new("127.0.0.1:8080");
    let server = Server::new("127.0.0.1:8080");
    server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Server {
        Server {
            addr
        }
    }

    fn run(self) {

    }
}