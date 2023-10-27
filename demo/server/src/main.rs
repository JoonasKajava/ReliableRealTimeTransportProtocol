extern crate lib_rrttp;

use lib_rrttp::socket::Socket;

const ADDR: &str = "127.0.0.1:12345";

fn main() {
    let mut test = Socket::bind(ADDR).expect("Failed to bind socket");

    println!("Waiting for data on {}", ADDR);
    loop {
        let (size, slice, addr) = test.receive().unwrap();
        println!("Received {} bytes from {} contents: {:?}", size, addr, std::str::from_utf8(slice).unwrap());
    }
}
