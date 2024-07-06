pub mod dns_header;
pub mod dns_message;
pub mod dns_question;
pub mod resrec;

use dns_header::header::{
    AuthoritativeAnswer, DnsHeader, OpCode, QueryResponse, RecursionAvailability, RecursionDesire,
    ResponseCode, Truncated, Z,
};
use dns_message::message::DnsMessage;
use dns_question::question::DnsQuestion;
use resrec::{QClass, QType};
use std::net::UdpSocket;

fn main() {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let header = DnsHeader {
                    packet_id: 1234,
                    qr: QueryResponse::Reply,
                    opcode: OpCode::IQuery,
                    aa: AuthoritativeAnswer::NonAuthoritative,
                    tc: Truncated::NotTruncated,
                    rd: RecursionDesire::NotDesired,
                    ra: RecursionAvailability::NotAvailable,
                    z: Z::Reserved,
                    rcode: ResponseCode::NoErrorCondition,
                    qdcount: 1,
                    ancount: 0,
                    nscount: 0,
                    arcount: 0,
                };
                let question = DnsQuestion {
                    qname: vec!["codecrafters".to_string(), "io".to_string()],
                    qtype: QType::A,
                    qclass: QClass::IN,
                };

                let message = DnsMessage {
                    header,
                    questions: vec![question],
                };
                let message: Vec<u8> = message.as_bytes();
                let response: &[u8] = &message;
                udp_socket
                    .send_to(response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
