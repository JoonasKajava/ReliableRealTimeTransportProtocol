use std::io::{Read, Write};
use std::net::{TcpListener, UdpSocket};
use std::sync::Arc;
use std::thread;
use std::time::Instant;

use csv::Writer;
use serde::Serialize;

use lib_rrttp::application_layer::connection_manager::{
    ConnectionEventType, ConnectionManager, ConnectionManagerInterface,
};

#[derive(Serialize)]
struct Row {
    cumulative_data: usize,
    timestamp: u128,
}

const ADDRESS: &str = "localhost:12345";
const SAMPLE_FILE: &str = "sample.mp4";
const OUTPUT_UDP: &str = "output_udp.mp4";
const OUTPUT_TCP: &str = "output_tcp.mp4";
const OUTPUT_RRTP: &str = "output_rrtp.mp4";

const PACKET_BUFFER_SIZE: usize = 128;

fn udp() {
    thread::scope(|s| {
        let socket = Arc::new(UdpSocket::bind(ADDRESS).expect("couldn't bind to address"));
        socket
            .connect(ADDRESS)
            .expect("couldn't connect to address");
        let socket_clone = Arc::clone(&socket);
        s.spawn(move || {
            let bytes = std::fs::read(SAMPLE_FILE).expect("couldn't read file");
            println!("Sending {} bytes", bytes.len());
            let test = Instant::now();
            for i in bytes.chunks(PACKET_BUFFER_SIZE) {
                socket_clone.send(i).expect("couldn't send data");
            }
            println!("Time taken: {:?}", test.elapsed());
        });

        let socket_clone2 = Arc::clone(&socket);
        s.spawn(move || {
            let buffer_size = std::fs::metadata(SAMPLE_FILE)
                .expect("couldn't read metadata")
                .len() as usize;

            let mut file_buffer = Vec::with_capacity(buffer_size);
            let mut buffer = [0; PACKET_BUFFER_SIZE];
            let now = Instant::now();
            let mut rows = Vec::new();
            loop {
                let (size, _) = socket_clone2
                    .recv_from(&mut buffer)
                    .expect("couldn't receive data");
                file_buffer.extend_from_slice(&buffer[..size]);
                rows.push(Row {
                    cumulative_data: file_buffer.len(),
                    timestamp: now.elapsed().as_micros(),
                });
                if size < PACKET_BUFFER_SIZE {
                    break;
                }
            }

            let mut wtr = Writer::from_path("udp.csv").expect("couldn't create csv file");
            for row in rows {
                wtr.serialize(row).expect("couldn't serialize row");
            }
            wtr.flush().expect("couldn't flush csv file");
            std::fs::write(OUTPUT_UDP, file_buffer).expect("couldn't write file");
        });
    });
}

fn tcp() {
    thread::scope(|s| {
        let listener = TcpListener::bind(ADDRESS).expect("couldn't bind to address");
        s.spawn(move || {
            let buffer_size = std::fs::metadata(SAMPLE_FILE)
                .expect("couldn't read metadata")
                .len() as usize;

            let mut file_buffer = Vec::with_capacity(buffer_size);
            let mut buffer = [0; PACKET_BUFFER_SIZE];
            let (mut stream, _) = listener.accept().expect("couldn't accept connection");
            let mut rows = Vec::new();
            let now = Instant::now();
            loop {
                let size = stream.read(&mut buffer).expect("couldn't read data");

                file_buffer.extend_from_slice(&buffer[..size]);
                rows.push(Row {
                    cumulative_data: file_buffer.len(),
                    timestamp: now.elapsed().as_micros(),
                });
                if size == 0 {
                    break;
                }
            }
            let mut wtr = Writer::from_path("tcp.csv").expect("couldn't create csv file");
            for row in rows {
                wtr.serialize(row).expect("couldn't serialize row");
            }
            wtr.flush().expect("couldn't flush csv file");
            std::fs::write(OUTPUT_TCP, file_buffer).expect("couldn't write file");
        });

        s.spawn(move || {
            let bytes = std::fs::read(SAMPLE_FILE).expect("couldn't read file");
            println!("Sending {} bytes", bytes.len());
            let mut stream =
                std::net::TcpStream::connect(ADDRESS).expect("couldn't connect to address");
            stream.write_all(&bytes).expect("couldn't write data");
            stream.flush().expect("couldn't flush stream");
            stream
                .shutdown(std::net::Shutdown::Both)
                .expect("couldn't shutdown stream");
        });
    })
}

fn rrtp() {
    let ConnectionManagerInterface {
        connection_manager,
        connection_events,
        message_sender,
    } = ConnectionManager::start(ADDRESS).expect("couldn't start connection manager");

    connection_manager.connect(ADDRESS).unwrap();

    thread::scope(|s| {
        s.spawn(|| {
            let bytes = std::fs::read(SAMPLE_FILE).expect("couldn't read file");
            println!("Sending {} bytes", bytes.len());
            message_sender.send(bytes).unwrap();
        });

        s.spawn(move || {
            let mut file_buffer = Vec::new();
            let mut rows = Vec::new();
            let now = Instant::now();
            'l: loop {
                let event = connection_events.recv().unwrap();
                match event {
                    ConnectionEventType::ReceivedFrame(r) => {
                        file_buffer.extend_from_slice(r.get_data());
                        rows.push(Row {
                            cumulative_data: file_buffer.len(),
                            timestamp: now.elapsed().as_micros(),
                        });
                    }
                    ConnectionEventType::ReceivedCompleteMessage(_bytes) => {
                        break 'l;
                    }
                    ConnectionEventType::SentMessage => {}
                    ConnectionEventType::SentFrame(_) => {}
                    ConnectionEventType::ReceivedAck(_) => {}
                    ConnectionEventType::SentAck(_) => {}
                }
            }

            let mut wtr = Writer::from_path("rrtp.csv").expect("couldn't create csv file");
            for row in rows {
                wtr.serialize(row).expect("couldn't serialize row");
            }
            wtr.flush().expect("couldn't flush csv file");
            std::fs::write(OUTPUT_RRTP, file_buffer).expect("couldn't write file");
        });
    });
}

fn main() {
    udp();
    tcp();
    rrtp();
}
