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
    /// Specifies whether the message should be truncated,
    /// truncated if the message is larger than 512 bytes. Always not truncated for DNS over UDP.
    pub tc: Truncated,
    /// Specifies the recursion desire, desired if the client wants the server to perform recursive resolution.
    pub rd: RecursionDesire,
    /// Specifies the recursion availablility, available if the server supports recursive resolution.
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
    /// (0) Question
    Question,
    /// (1) Reply
    Reply,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// (0) Standard query
    Query,
    /// (1) Inverse query
    IQuery,
    /// (2) Server status request
    Status,
    /// (3 - 15) reserved for future use
    Reserved(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthoritativeAnswer {
    /// (0) Non-authoritative
    NonAuthoritative,
    /// (1) Authoritative
    Authoritative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Truncated {
    /// (0) Message is not truncated
    NotTruncated,
    /// (1) Message is truncated
    Truncated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursionDesire {
    /// (0) Recursion not desired
    NotDesired,
    /// (1) Recursion desired
    Desired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecursionAvailability {
    /// (0) Recursion not available
    NotAvailable,
    /// (1) Recursion available
    Available,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Z {
    /// (0 - 7) Reserved
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCode {
    /// (0) No error condition
    NoErrorCondition,
    /// (1) The name server was unable to interpret the query.
    FormatError,
    /// (2) The name server was unable to process this query due to a problem with the name server.
    ServerFailure,
    /// (3) Meaningful only for responses from an authoritative name server,
    /// this code signifies that the domain name referenced in the query does not exist.
    NameError,
    /// (4) The name server does not support the requested kind of query.
    NotImplemented,
    /// (5) The name server refuses to perform the specified operation for policy reasons.
    /// For example, a name server may not wish to provide the information to the
    /// particular requester, or a name server may not wish to perform a particular
    /// operation (e.g., zone transfer) for particular data.
    Refused,
    // (6 - 15) reserved for future use
    Reserved(u8),
}
