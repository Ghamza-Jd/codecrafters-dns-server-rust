use super::question::DnsQuestion;
use crate::resrec::{QClass, QType};
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

impl From<&[u8]> for DnsQuestion {
    fn from(val: &[u8]) -> Self {
        let mut qname = Vec::new();
        let mut i = 0;
        while i < val.len() {
            let len: usize = val[i].into();
            if len == 0 {
                break;
            }
            let Ok(label) = String::from_utf8(val[i + 1..i + 1 + len].to_vec()) else {
                break;
            };
            qname.push(label);
            i += len + 1;
        }
        DnsQuestion {
            qname,
            qtype: QType::A,
            qclass: QClass::IN,
        }
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

    #[test]
    fn simple_question_from_bytes_test() {
        use crate::dns_question::question::DnsQuestion;
        use crate::resrec::{QClass, QType};

        let bytes = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0, 0, 1, 0, 1,
        ];

        let question = DnsQuestion::from(&bytes[..]);
        assert_eq!(
            question,
            DnsQuestion {
                qname: vec!["www".to_string(), "example".to_string(), "com".to_string()],
                qtype: QType::A,
                qclass: QClass::IN,
            }
        );
    }
}
