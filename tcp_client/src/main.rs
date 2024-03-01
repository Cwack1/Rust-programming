//TCP CLIENT

use std::net::TcpStream;


fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();

    println!("Connected to the server?");
}
