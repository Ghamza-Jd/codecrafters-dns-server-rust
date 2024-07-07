use super::header::{
    AuthoritativeAnswer, DnsHeader, OpCode, QueryResponse, RecursionAvailability, RecursionDesire,
    ResponseCode, Truncated, Z,
};
use bytes::{Buf, BufMut, BytesMut};

/// Six sections in the header, each 2 bytes long
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

impl From<&[u8]> for DnsHeader {
    fn from(val: &[u8]) -> Self {
        let mut header = BytesMut::with_capacity(HEADER_SIZE_IN_BYTES);
        header.put(&val[0..HEADER_SIZE_IN_BYTES]);
        let packet_id = header.get_u16();
        let left_meta = header.get_u8();
        let right_meta = header.get_u8();
        let qdcount = header.get_u16();
        let ancount = header.get_u16();
        let nscount = header.get_u16();
        let arcount = header.get_u16();

        let qr = QueryResponse::from(left_meta);
        let opcode = OpCode::from(left_meta);
        let aa = AuthoritativeAnswer::from(left_meta);
        let tc = Truncated::from(left_meta);
        let rd = RecursionDesire::from(left_meta);

        let ra = RecursionAvailability::from(right_meta);
        let z = Z::from(right_meta);

        let rcode = match opcode {
            OpCode::IQuery => ResponseCode::NotImplemented,
            OpCode::Status => ResponseCode::NotImplemented,
            OpCode::Reserved(_) => ResponseCode::NotImplemented,
            _ => ResponseCode::from(right_meta),
        };

        DnsHeader {
            packet_id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
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

impl From<u8> for QueryResponse {
    fn from(val: u8) -> Self {
        let val = (val & 0b1000_0000) >> 7;
        match val {
            0 => QueryResponse::Question,
            _ => QueryResponse::Reply,
        }
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

impl From<u8> for OpCode {
    fn from(val: u8) -> Self {
        let val = (val & 0b0111_1000) >> 3;
        match val {
            0 => OpCode::Query,
            1 => OpCode::IQuery,
            2 => OpCode::Status,
            _ => OpCode::Reserved(val),
        }
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

impl From<u8> for AuthoritativeAnswer {
    fn from(val: u8) -> Self {
        let val = (val & 0b0100_0000) >> 2;
        match val {
            0 => AuthoritativeAnswer::NonAuthoritative,
            _ => AuthoritativeAnswer::Authoritative,
        }
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

impl From<u8> for Truncated {
    fn from(val: u8) -> Self {
        let val = (val & 0b0000_0010) >> 1;
        match val {
            0 => Truncated::NotTruncated,
            _ => Truncated::Truncated,
        }
    }
}

impl RecursionDesire {
    fn as_byte(&self) -> u8 {
        match self {
            RecursionDesire::NotDesired => 0,
            RecursionDesire::Desired => 1,
        }
    }
}

impl From<u8> for RecursionDesire {
    fn from(val: u8) -> Self {
        let val = val & 0b0000_0001;
        match val {
            0 => RecursionDesire::NotDesired,
            _ => RecursionDesire::Desired,
        }
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

impl From<u8> for RecursionAvailability {
    fn from(val: u8) -> Self {
        let val = (val & 0b1000_0000) >> 7;
        match val {
            0 => RecursionAvailability::NotAvailable,
            _ => RecursionAvailability::Available,
        }
    }
}

impl Z {
    fn as_byte(&self) -> u8 {
        0 << 4
    }
}

impl From<u8> for Z {
    fn from(_: u8) -> Self {
        Z::Reserved
    }
}

impl ResponseCode {
    fn as_byte(&self) -> u8 {
        match self {
            ResponseCode::NoErrorCondition => 0,
            ResponseCode::FormatError => 1,
            ResponseCode::ServerFailure => 2,
            ResponseCode::NameError => 3,
            ResponseCode::NotImplemented => 4,
            ResponseCode::Refused => 5,
            ResponseCode::Reserved(value) => *value,
        }
    }
}

impl From<u8> for ResponseCode {
    fn from(val: u8) -> Self {
        let val = val & 0b0000_1111;
        match val {
            0 => ResponseCode::NoErrorCondition,
            1 => ResponseCode::FormatError,
            2 => ResponseCode::ServerFailure,
            3 => ResponseCode::NameError,
            4 => ResponseCode::NotImplemented,
            5 => ResponseCode::Refused,
            _ => ResponseCode::Reserved(val),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns_header::header::{
        AuthoritativeAnswer, DnsHeader, OpCode, QueryResponse, RecursionAvailability,
        RecursionDesire, ResponseCode, Truncated, Z,
    };

    #[test]
    fn simple_header_as_bytes_test() {
        let bytes = DnsHeader {
            packet_id: 1234,
            qr: QueryResponse::Question,
            opcode: OpCode::Query,
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
        }
        .as_bytes();
        let expected_bytes = vec![
            0b0000_0100,
            0b1101_0010,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0001,
            0b0000_0000,
            0b0000_0001,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ];
        assert_eq!(bytes, expected_bytes)
    }

    #[test]
    fn query_response_as_byte_test() {
        assert_eq!(QueryResponse::Question.as_byte(), 0b0000_0000);
        assert_eq!(QueryResponse::Reply.as_byte(), 0b1000_0000);
    }

    #[test]
    fn opcode_as_byte_test() {
        assert_eq!(OpCode::Query.as_byte(), 0b0000_0000);
        assert_eq!(OpCode::IQuery.as_byte(), 0b0000_1000);
        assert_eq!(OpCode::Status.as_byte(), 0b0001_0000);
        assert_eq!(OpCode::Reserved(3).as_byte(), 0b0001_1000);
    }

    #[test]
    fn authoritative_answer_as_byte_test() {
        assert_eq!(AuthoritativeAnswer::NonAuthoritative.as_byte(), 0b0000_0000);
        assert_eq!(AuthoritativeAnswer::Authoritative.as_byte(), 0b0000_0100);
    }

    #[test]
    fn truncated_as_byte_test() {
        assert_eq!(Truncated::NotTruncated.as_byte(), 0b0000_0000);
        assert_eq!(Truncated::Truncated.as_byte(), 0b0000_0010);
    }

    #[test]
    fn recursion_desire_as_byte_test() {
        assert_eq!(RecursionDesire::NotDesired.as_byte(), 0b0000_0000);
        assert_eq!(RecursionDesire::Desired.as_byte(), 0b0000_0001);
    }

    #[test]
    fn recursion_availability_as_byte_test() {
        assert_eq!(RecursionAvailability::NotAvailable.as_byte(), 0b0000_0000);
        assert_eq!(RecursionAvailability::Available.as_byte(), 0b1000_0000);
    }

    #[test]
    fn z_as_byte_test() {
        assert_eq!(Z::Reserved.as_byte(), 0b0000_0000);
    }

    #[test]
    fn response_code_as_byte_test() {
        assert_eq!(ResponseCode::NoErrorCondition.as_byte(), 0b0000_0000);
        assert_eq!(ResponseCode::FormatError.as_byte(), 0b0000_0001);
        assert_eq!(ResponseCode::ServerFailure.as_byte(), 0b0000_0010);
        assert_eq!(ResponseCode::NameError.as_byte(), 0b0000_0011);
        assert_eq!(ResponseCode::NotImplemented.as_byte(), 0b0000_0100);
        assert_eq!(ResponseCode::Refused.as_byte(), 0b0000_0101);
        assert_eq!(ResponseCode::Reserved(6).as_byte(), 0b0000_0110);
    }
}
