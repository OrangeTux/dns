/// See 4.1.2 of rfc
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Question {
    pub qname: String,
    pub qtype: QType,
    pub qclass: QClass,
}

impl TryFrom<&mut std::slice::Iter<'_, u8>> for Question {
    type Error = String;

    fn try_from(value: &mut std::slice::Iter<u8>) -> Result<Self, Self::Error> {
        let mut qname: String = String::new();
        loop {
            let length_of_label: usize = (*value.next().unwrap()).into();
            dbg!(length_of_label);
            if length_of_label == 0 {
                break;
            }
            if !qname.is_empty() {
                qname.push('.');
            }
            let mut label: Vec<u8> = vec![];

            for i in 0..length_of_label {
                let char = *value.next().unwrap();
                label.push(char);
            }
            qname.push_str(std::str::from_utf8(&label).unwrap());
        }

        Ok(Question {
            qname,
            qtype: u16::from_be_bytes([*value.next().unwrap(), *value.next().unwrap()])
                .try_into()
                .unwrap(),
            qclass: u16::from_be_bytes([*value.next().unwrap(), *value.next().unwrap()])
                .try_into()
                .unwrap(),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum QType {
    A,
    NS,
    MD,
    MF,
    CNAME,
    SOA,
    MB,
    MG,
    MR,
    Null,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT,
    AXFR,
    MAILB,
    MAILA,
}

impl TryFrom<u16> for QType {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::Null,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            252 => Self::AXFR,
            253 => Self::MAILB,
            254 => Self::MAILA,
            _ => return Err("Failed to parse QTYPE".to_string()),
        })
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum QClass {
    IN,
    CS,
    CH,
    HS,
    Any,
}

impl TryFrom<u16> for QClass {
    type Error = String;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            255 => Self::Any,
            _ => return Err("Failed to parse QClass".to_string()),
        })
    }
}
