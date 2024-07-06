pub mod dns_header;

use dns_header::header::{
    AuthoritativeAnswer, DnsHeader, OpCode, QueryResponse, RecursionAvailability, RecursionDesire,
    ResponseCode, Truncated, Z,
};
use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let message = DnsHeader {
                    packet_id: 1234,
                    qr: QueryResponse::Reply,
                    opcode: OpCode::IQuery,
                    aa: AuthoritativeAnswer::NonAuthoritative,
                    tc: Truncated::NotTruncated,
                    rd: RecursionDesire::NotDesired,
                    ra: RecursionAvailability::NotAvailable,
                    z: Z::Reserved,
                    rcode: ResponseCode::NoErrorCondition,
                    qdcount: 0,
                    ancount: 0,
                    nscount: 0,
                    arcount: 0,
                };
                let message: Vec<u8> = message.as_bytes();
                let response: &[u8] = &message;
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
