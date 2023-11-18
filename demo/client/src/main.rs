extern crate lib_rrttp;

use std::time::SystemTime;

use lib_rrttp::window::receiver::Receiver;

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
    let mut client = Receiver::new(ADDR, "127.0.0.1:12345").expect("Failed to bind socket");

    println!("Connected to {}", ADDR);

    client.read();
}
