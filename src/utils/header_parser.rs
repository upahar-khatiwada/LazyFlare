pub fn parseHeaders(buf: &[u8]) {
    let id_raw: &[u8] = &buf[..2];

    let id: String = id_raw.iter().map(|i: &u8| format!("{:02X}", i)).collect();

    println!("{}", id);

    let flags_raw: &[u8] = &buf[2..4];

    let flags = parseFlags(flags_raw);
}

fn parseFlags(flagsBuf: &[u8]) {}
