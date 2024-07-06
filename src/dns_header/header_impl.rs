use super::header::{
    AuthoritativeAnswer, DnsHeader, OpCode, QueryResponse, RecursionAvailability, RecursionDesire,
    ResponseCode, Truncated, Z,
};
use bytes::BufMut;

const HEADER_SIZE_IN_BYTES: usize = 2 * 6;

impl DnsHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(HEADER_SIZE_IN_BYTES);
        bytes.put_u16(self.packet_id);

        let left_meta = self.qr.as_byte()
            | self.opcode.as_byte()
            | self.aa.as_byte()
            | self.tc.as_byte()
            | self.rd.as_byte();
        bytes.put_u8(left_meta);

        let right_meta = self.ra.as_byte() | self.z.as_byte() | self.rcode.as_byte();
        bytes.put_u8(right_meta);

        bytes.put_u16(self.qdcount);
        bytes.put_u16(self.ancount);
        bytes.put_u16(self.nscount);
        bytes.put_u16(self.arcount);
        bytes
    }
}

impl QueryResponse {
    fn as_byte(&self) -> u8 {
        let val = match self {
            QueryResponse::Question => 0,
            QueryResponse::Reply => 1,
        };
        val << 7
    }
}

impl OpCode {
    fn as_byte(&self) -> u8 {
        let val = match self {
            OpCode::Query => 0,
            OpCode::IQuery => 1,
            OpCode::Status => 2,
            OpCode::Reserved(value) => *value,
        };
        val << 3
    }
}

impl AuthoritativeAnswer {
    fn as_byte(&self) -> u8 {
        let val = match self {
            AuthoritativeAnswer::NonAuthoritative => 0,
            AuthoritativeAnswer::Authoritative => 1,
        };
        val << 2
    }
}

impl Truncated {
    fn as_byte(&self) -> u8 {
        let val = match self {
            Truncated::NotTruncated => 0,
            Truncated::Truncated => 1,
        };
        val << 1
    }
}

impl RecursionDesire {
    fn as_byte(&self) -> u8 {
        let val = match self {
            RecursionDesire::NotDesired => 0,
            RecursionDesire::Desired => 1,
        };
        val
    }
}

impl RecursionAvailability {
    fn as_byte(&self) -> u8 {
        let val = match self {
            RecursionAvailability::NotAvailable => 0,
            RecursionAvailability::Available => 1,
        };
        val << 7
    }
}

impl Z {
    fn as_byte(&self) -> u8 {
        0 << 4
    }
}

impl ResponseCode {
    fn as_byte(&self) -> u8 {
        let val = match self {
            ResponseCode::NoErrorCondition => 0,
            ResponseCode::FormatError => 1,
            ResponseCode::ServerFailure => 2,
            ResponseCode::NameError => 3,
            ResponseCode::NotImplemented => 4,
            ResponseCode::Refused => 5,
            ResponseCode::Reserved(value) => *value,
        };
        val
    }
}
