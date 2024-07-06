/// ## Header section format
///
/// ```text
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      ID                       |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    QDCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ANCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    NSCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                    ARCOUNT                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Debug)]
pub struct DnsHeader {
    /// A random ID assigned to query packets. Response packets must reply with the same ID.
    pub packet_id: u16,
    /// Specifies whether the message is a query or a response.
    pub qr: QueryResponse,
    /// Specifies the kind of query in a message
    pub opcode: OpCode,
    pub aa: AuthoritativeAnswer,
    /// Truncated message, true if the message is larger than 512 bytes. Always false for DNS over UDP.
    pub tc: Truncated,
    /// Recursion desired, true if the client wants the server to perform recursive resolution.
    pub rd: RecursionDesire,
    /// Recursion available, true if the server supports recursive resolution.
    pub ra: RecursionAvailability,
    /// At inception, it was reserved for future use
    pub z: Z,
    /// Response code indicating the status of the response
    pub rcode: ResponseCode,
    /// Number of questions in the Question section.
    pub qdcount: u16,
    /// Number of records in the Answer section.
    pub ancount: u16,
    /// Number of records in the Authority section.
    pub nscount: u16,
    /// Number of records in the Additional section.
    pub arcount: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryResponse {
    Question,
    Reply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// 0 Standard query
    Query,
    /// 1 Inverse query
    IQuery,
    /// 2 Server status request
    Status,
    // 3 - 15 reserved for future use
    Reserved(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoritativeAnswer {
    NonAuthoritative,
    Authoritative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Truncated {
    NotTruncated,
    Truncated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursionDesire {
    NotDesired,
    Desired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursionAvailability {
    NotAvailable,
    Available,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Z {
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCode {
    /// No error condition
    NoErrorCondition,
    /// The name server was unable to interpret the query.
    FormatError,
    /// The name server was unable to process this query due to a problem with the name server.
    ServerFailure,
    /// Meaningful only for responses from an authoritative name server,
    /// this code signifies that the domain name referenced in the query does not exist.
    NameError,
    /// The name server does not support the requested kind of query.
    NotImplemented,
    /// The name server refuses to perform the specified operation for policy reasons.
    /// For example, a name server may not wish to provide the information to the
    /// particular requester, or a name server may not wish to perform a particular
    /// operation (e.g., zone transfer) for particular data.
    Refused,
    // 6 - 15 reserved for future use
    Reserved(u8),
}
