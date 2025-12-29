#[allow(warnings)]
pub fn parseHeaders(buf: &[u8]) {
    let id_raw: &[u8] = &buf[..2];

    let id: String = id_raw.iter().map(|i: &u8| format!("{:02X}", i)).collect();

    // println!("{}", id);

    let flags_raw: &[u8] = &buf[2..4];

    let flags: String = parseFlags(flags_raw);
    println!("{flags}")


}

#[allow(warnings)]
fn parseFlags(flags_buf: &[u8]) -> String {
    // let first_byte: String = format!("{:08b}", flags_buf[0]);
    // let second_byte: String = format!("{:08b}", flags_buf[1]);

    // print!("{}, {}\n", first_byte, second_byte);

    let b1: u8 = flags_buf[0];
    let b2: u8 = flags_buf[1];

    // format!(
    //     // "{}{:04b}{}{}{}{}{:03b}{:04b}",
    //     "{}{}{}{}{}{}{}{}",
    //     (b1 >> 7) & 1,
    //     (b1 >> 3) & 0x0F,
    //     (b1 >> 2) & 1,
    //     (b1 >> 1) & 1,
    //     b1 & 1,
    //     (b2 >> 7) & 1,
    //     (b2 >> 4) & 0x07,
    //     b2 & 0x0F
    // )

    format!("{:02X}{:02X}", b1, b2)
}
