use super::answer::DnsAnswer;
use bytes::BufMut;

impl DnsAnswer {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut labels = self
            .name
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

        let rdata = self
            .rddata
            .iter()
            .flat_map(|x| x.octets())
            .map(|x| char::from_u32(x.into()).unwrap())
            .collect::<String>();

        let mut bytes = Vec::new();
        bytes.put(labels.as_bytes());
        bytes.put_u16(self.typ as u16);
        bytes.put_u16(self.class as u16);
        bytes.put_i32(self.ttl);
        bytes.put_u16(self.rdlength);
        bytes.put(rdata.as_bytes());
        bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dns_answer::answer::DnsAnswer,
        resrec::{QClass, QType},
    };
    use std::net::Ipv4Addr;

    #[test]
    fn test_as_bytes() {
        let answer = DnsAnswer {
            name: vec!["www".to_string(), "example".to_string(), "com".to_string()],
            typ: QType::A,
            class: QClass::IN,
            ttl: 3600,
            rdlength: 4,
            rddata: vec![Ipv4Addr::new(192, 168, 1, 1)],
        };
        let bytes = answer.as_bytes();
        assert_eq!(
            bytes,
            vec![
                3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o',
                b'm', 0, 0, 1, 0, 1, 0, 0, 14, 16, 0, 4, 195, 128, 194, 168, 1, 1
            ]
        );
    }
}
