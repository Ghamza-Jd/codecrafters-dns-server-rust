pub mod dns_answer;
pub mod dns_header;
pub mod dns_message;
pub mod dns_question;
pub mod resrec;

use dns_message::message::DnsMessage;
use std::net::UdpSocket;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let mut message = DnsMessage::from(&buf[..]);
                message.to_response();

                udp_socket
                    .send_to(&message.as_bytes(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
