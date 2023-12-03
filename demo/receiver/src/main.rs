extern crate lib_rrttp;

use std::fs;
use std::time::SystemTime;
use log::info;
use lib_rrttp::window::Window;


//noinspection DuplicatedCode
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


const ADDR: &str = "127.0.0.1:54321";
fn main() {
    setup_logger().expect("Failed to setup logger");
    const REMOTE_ADDR: &str = "127.0.0.1:12345";
    let mut client = Window::new(ADDR, REMOTE_ADDR).expect("Failed to bind socket");
    info!("Client bound to {}", ADDR);
    info!("Reading data from {}", REMOTE_ADDR);

    client.listen().join().unwrap();

    for message in client.incoming_messages() {
        fs::write("received.gif", message).expect("Failed to write file");
    }
}
