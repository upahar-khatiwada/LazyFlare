mod server;
mod utils;

use crate::server::udp::create_udp_socket;
use crate::utils::constants::{IP, PORT};
use crate::utils::response_builder::create_response;

fn main() {
    let addr: String = format!("{}:{}", IP, PORT);
    let socket: std::net::UdpSocket =
        create_udp_socket(&addr).expect("Failed to create the socket");

    let mut buf: [u8; 512] = [0u8; 512];

    // let records: Vec<String> = loadAllDNSRecords();

    loop {
        let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf: &mut [u8] = &mut buf[..number_of_bytes];

        println!(
            "Number of bytes: {}\nSource Address: {}\nfilled Buf: {:?}\n\n",
            number_of_bytes, src_addr, filled_buf
        );

        create_response(filled_buf);
    }
}
