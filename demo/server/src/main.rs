extern crate lib_rrttp;

use std::fs;
use std::time::SystemTime;
use log::info;
use lib_rrttp::window::Window;

const ADDR: &str = "127.0.0.1:12345";

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    setup_logger().expect("Failed to setup logger");
    const REMOTE_ADDR: &str = "127.0.0.1:54321";
    let mut client = Window::new(ADDR, REMOTE_ADDR).expect("Failed to bind socket");
    info!("Client bound to {}", ADDR);

    let listen_handle = client.read();

    let file = fs::read("test-payload.txt").expect("Failed to read file");
    info!("Sending file to {}", REMOTE_ADDR);
    client.send(file.as_slice()).expect("Failed to send data");

    listen_handle.join().unwrap()
}
