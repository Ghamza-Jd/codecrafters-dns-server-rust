use super::question::DnsQuestion;
use bytes::BufMut;

impl DnsQuestion {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut labels = self
            .qname
            .iter()
            .flat_map(|x| {
                let len = x.len().try_into().ok().and_then(char::from_u32)?;
                let mut label = String::new();
                label.push(len);
                label.push_str(x);
                Some(label)
            })
            .collect::<Vec<_>>()
            .join("");
        labels.push('\0');

        let mut bytes = Vec::new();
        bytes.put(labels.as_bytes());
        bytes.put_u16(self.qtype as u16);
        bytes.put_u16(self.qclass as u16);
        bytes
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_question_as_bytes_test() {
        use crate::dns_question::question::DnsQuestion;
        use crate::resrec::{QClass, QType};

        let question = DnsQuestion {
            qname: vec!["www".to_string(), "example".to_string(), "com".to_string()],
            qtype: QType::A,
            qclass: QClass::IN,
        };

        let bytes = question.as_bytes();
        assert_eq!(
            bytes,
            vec![
                3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o',
                b'm', 0, 0, 1, 0, 1
            ]
        );
    }
}
