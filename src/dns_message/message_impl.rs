use super::message::DnsMessage;

impl DnsMessage {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.header.as_bytes());
        self.questions.iter().for_each(|q| {
            bytes.append(&mut q.as_bytes());
        });
        bytes.append(&mut self.answer.as_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dns_answer::answer::DnsAnswer,
        dns_header::header::{
            AuthoritativeAnswer, DnsHeader, OpCode, QueryResponse, RecursionAvailability,
            RecursionDesire, ResponseCode, Truncated, Z,
        },
        dns_message::message::DnsMessage,
        dns_question::question::DnsQuestion,
        resrec::{QClass, QType},
    };
    use std::net::Ipv4Addr;

    #[test]
    fn test_as_bytes() {
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
            ancount: 1,
            nscount: 0,
            arcount: 0,
        };
        let question = DnsQuestion {
            qname: vec!["codecrafters".to_string(), "io".to_string()],
            qtype: QType::A,
            qclass: QClass::IN,
        };
        let answer = DnsAnswer {
            name: vec!["codecrafters".to_string(), "io".to_string()],
            typ: QType::A,
            class: QClass::IN,
            ttl: 60,
            rdlength: 4,
            rddata: vec![Ipv4Addr::new(8, 8, 8, 8)],
        };

        let message = DnsMessage {
            header,
            questions: vec![question],
            answer,
        };
        let bytes = message.as_bytes();

        assert_eq!(
            bytes,
            [
                4, 210, 136, 0, 0, 1, 0, 1, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102,
                116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 12, 99, 111, 100, 101, 99, 114, 97,
                102, 116, 101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1, 0, 0, 0, 60, 0, 4, 8, 8, 8, 8
            ]
        );
    }
}
