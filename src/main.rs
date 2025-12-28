mod server;
mod utils;

use crate::server::udp::create_udp_socket;
use crate::utils::constants::{IP, PORT};
use crate::utils::header_parser::parseHeaders;

#[allow(dead_code, warnings)]

fn main() {
    let addr: String = format!("{}:{}", IP, PORT);
    let socket: std::net::UdpSocket =
        create_udp_socket(&addr).expect("Failed to create the socket");

    let mut buf: [u8; 512] = [0u8; 512];

    while true {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf: &mut [u8] = &mut buf[..number_of_bytes];

        println!(
            "Number of bytes: {}\nSource Address: {}\nfilled Buf: {:?}\n\n",
            number_of_bytes, src_addr, filled_buf
        );

        parseHeaders(filled_buf);
    }
}
