use crate::resrec::{QClass, QType};
use std::net::Ipv4Addr;

/// ## Answer Section
///
/// ```text
///                                 1  1  1  1  1  1
///   0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                                               |
/// /                                               /
/// /                      NAME                     /
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TYPE                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                     CLASS                     |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                      TTL                      |
/// |                                               |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// |                   RDLENGTH                    |
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--|
/// /                     RDATA                     /
/// /                                               /
/// +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
/// ```
#[derive(Debug)]
pub struct DnsAnswer {
    /// an owner name, i.e., the name of the node to which this resource record pertains.
    pub name: Vec<String>,
    /// two octets containing one of the RR TYPE codes.
    pub typ: QType,
    /// two octets containing one of the RR CLASS codes.
    pub class: QClass,
    /// a 32 bit signed integer that specifies the time interval that the resource record
    /// may be cached before the source of the information should again be consulted.
    /// Zero values are interpreted to mean that the RR can only be used for the
    /// transaction in progress, and should not be cached.  For example, SOA records are
    /// always distributed with a zero TTL to prohibit caching.  Zero values can also be
    /// used for extremely volatile data.
    pub ttl: i32,
    /// an unsigned 16 bit integer that specifies the length in octets of the RDATA field.
    pub rdlength: u16,
    /// a variable length string of octets that describes the resource. The format of
    /// this information varies according to the TYPE and CLASS of the resource record.
    pub rddata: Vec<Ipv4Addr>,
}
