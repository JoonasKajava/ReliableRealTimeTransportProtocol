extern crate lib_rrttp;

use std::fs;
use std::thread::sleep;
use std::time::SystemTime;
use log::info;

use lib_rrttp::window::transmitter::Transmitter;

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
    let mut transmitter = Transmitter::new(ADDR).expect("Failed to bind socket");
    transmitter.socket.connect("127.0.0.1:54321").expect("Failed to connect");
    info!("Transmitter bound to {}", ADDR);

    let file = fs::read("test-payload.txt").expect("Failed to read file");
    transmitter.send(file.as_slice()).expect("Failed to send data");
}
