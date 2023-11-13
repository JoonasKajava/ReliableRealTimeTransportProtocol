extern crate lib_rrttp;

use std::thread::sleep;
use lib_rrttp::socket::Socket;

const ADDR: &str = "127.0.0.1:54321";
fn main() {
    let client = Socket::bind(ADDR).expect("Failed to bind socket");
    client.connect("127.0.0.1:12345").expect("Failed to connect");
    println!("Connected to {}", ADDR);
    loop {
        println!("Sending data");
        client.send(b"Hello, world! jotain").expect("Failed to send");
        sleep(std::time::Duration::from_secs(10));
    }
}
