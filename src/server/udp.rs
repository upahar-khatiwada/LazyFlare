use std::io;
use std::net::UdpSocket;

pub fn create_udp_socket(addr: &str) -> io::Result<UdpSocket> {
    let socket = UdpSocket::bind(addr)?;
    Ok(socket)
}
