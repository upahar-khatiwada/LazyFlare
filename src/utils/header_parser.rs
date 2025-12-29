#[allow(warnings)]
pub fn createResponse(buf: &[u8]) {
    let ID: String = buf[..2].iter().map(|i: &u8| format!("{:02X}", i)).collect();

    let FLAGS: String = createFlags(&buf[2..4]);

    let QDCOUNT: String = format!("{:02X}", 01); // question count is always 1

    // assuming 0 for these
    let NSCOUNT: String = format!("{:02X}", 00);
    let ARCOUNT: String = format!("{:02X}", 00);

    let (domain_name, domain_type): (String, String) = getDomainNameAndRecordType(&buf[12..]);
}

#[allow(warnings)]
fn getDomainNameAndRecordType(buf: &[u8]) -> (String, String) {
    let mut x: usize = 0;
    let mut domain_string: String = String::new();
    let mut domain_type: String = String::new();

    while buf[x] != 0 {
        let fixed_length: usize = buf[x] as usize;
        x += 1;

        for i in 0..fixed_length {
            domain_string.push(buf[x + i] as char);
        }

        x += fixed_length;

        if (buf[x] != 0) {
            domain_string.push('.');
        }
    }

    print!("x: {x}\n");
    for i in x + 1..x + 3 {
        domain_type.push_str(&format!("{:02X}", buf[i]));
    }

    println!("{domain_type}");
    (domain_string, domain_type)
}

#[allow(warnings)]
fn createHeaders(buf: &[u8]) {}

#[allow(warnings)]
fn createFlags(flags_buf: &[u8]) -> String {
    let b1: u8 = flags_buf[0];

    let QR: u8 = 1 << 7; // we are sending a response so its 1 always
    let OPCODE: u8 = b1 & 0x78;
    let AA: u8 = 1 << 2; // assuming our server is authorative
    let TC: u8 = 0 << 1; // we are only handling the packets less than 512 bytes for UDP protocol
    // let RD: u8 = b1 & 1;
    let RD: u8 = 0;
    let RA: i32 = 0 << 7; // not handling recursion
    let Z: i32 = 0 << 4; // reserved bits
    let RCODE: i32 = 0; // not handling error case

    let flags_byte1: u8 = QR | OPCODE | AA | TC | RD;
    let flags_byte2: i32 = RA | Z | RCODE;

    format!("{:02X}{:02X}", flags_byte1, flags_byte2)
}
