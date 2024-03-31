#[test]
fn checksum_hola() {
    let string = "Hola";
    let checksum = calculate_checksum(string.as_bytes());

    assert_eq!(checksum, 0b1011010011010000);
}

fn calculate_checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    for pair in data.chunks(2) {
        let mut checksum = 0u16;
        println!(
            "{:#b} & {:#b}",
            pair.first().unwrap(),
            pair.get(1).unwrap_or(&0)
        );
        checksum += u16::from(pair[0]) << 8;
        checksum += u16::from(pair.get(1).cloned().unwrap_or_default());
        sum += checksum as u32;
    }
    while sum > 0xffff {
        let carry = sum >> 16;
        sum &= 0xffff;
        sum += carry;
    }
    !sum as u16
}

#[test]
fn check_checksum() {
    let ip = 1u128.to_be_bytes();
    let port = 12345u16.to_be_bytes();
    let protocol = 0x0011u32.to_be_bytes();
    let payload = [
        0x00, 0x00, 0x00, 0x01, 0x02, 0x00, 0x0c, 0x08, 0x00, 0x68, 0x65, 0x6c, 0x6c, 0x6f, 0x20,
        0x77, 0x6f, 0x72, 0x6c, 0x64,
    ];
    let udp_packet_length = 28u16;
    let udp_packet_length = udp_packet_length.to_be_bytes();
    let mut data = vec![];
    data.extend_from_slice(&ip);
    data.extend_from_slice(&ip);
    data.extend_from_slice(&protocol);
    data.extend_from_slice(&udp_packet_length);
    data.extend_from_slice(&port);
    data.extend_from_slice(&port);
    data.extend_from_slice(&udp_packet_length);
    data.extend_from_slice(&payload);
    let checksum = calculate_checksum(&data);
    assert_eq!(checksum, 0xc2a7);
}

#[test]
fn check_other() {
    let ip = [
        0x60, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x11, 0xfd, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x01, 0xab, 0xcd, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xfd, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x60,
    ];
    let header = [0x26, 0x92, 0x26, 0x92, 0x00, 0x0c, 0x00, 0x00];
    let payload = [0x12, 0x34, 0x56, 0x78];

    let mut data = vec![];
    data.extend_from_slice(&ip);
    data.extend_from_slice(&header);
    data.extend_from_slice(&payload);

    let checksum = calculate_checksum(&data);
    println!("{:b} = {:b}", checksum, 0x7ed5);
}
