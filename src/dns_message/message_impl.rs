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
