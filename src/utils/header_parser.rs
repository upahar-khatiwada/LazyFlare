#[allow(warnings)]
pub fn createResponse(buf: &[u8]) {
    let id_raw: &[u8] = &buf[..2];

    let id: String = id_raw.iter().map(|i: &u8| format!("{:02X}", i)).collect();

    let flags: String = createFlags(&buf[2..4]);
}

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
