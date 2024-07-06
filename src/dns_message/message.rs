use crate::{
    dns_answer::answer::DnsAnswer, dns_header::header::DnsHeader,
    dns_question::question::DnsQuestion,
};

#[derive(Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answer: DnsAnswer,
}
