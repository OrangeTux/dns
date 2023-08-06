use crate::DecodeError;

/// See 4.1.2 of rfc
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Question {
    pub qname: String,
    pub qtype: QType,
    pub qclass: QClass,
}

impl TryFrom<&mut std::slice::Iter<'_, u8>> for Question {
    type Error = DecodeError;

    fn try_from(value: &mut std::slice::Iter<u8>) -> Result<Self, Self::Error> {
        let mut qname: String = String::new();
        loop {
            let length_of_label: usize = (*value.next().ok_or(DecodeError::NotEnoughBytes)?).into();
            if length_of_label == 0 {
                break;
            }
            if !qname.is_empty() {
                qname.push('.');
            }
            let mut label: Vec<u8> = vec![];

            for _ in 0..length_of_label {
                let char = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
                label.push(char);
            }
            qname.push_str(std::str::from_utf8(&label).map_err(|_| {
                DecodeError::IllegalValue(
                    "failed to parse value as qname: value not valid UTF-8".into(),
                )
            })?);
        }

        Ok(Question {
            qname,
            qtype: u16::from_be_bytes([
                *value.next().ok_or(DecodeError::NotEnoughBytes)?,
                *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            ])
            .try_into()
            .unwrap(),
            qclass: u16::from_be_bytes([
                *value.next().ok_or(DecodeError::NotEnoughBytes)?,
                *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            ])
            .try_into()
            .unwrap(),
        })
    }
}

impl Question {
    pub fn into_bytes(self) -> Vec<u8> {
        let mut name: Vec<u8> = self
            .qname
            .split(".")
            .map(|part| {
                let mut x = part.to_owned().into_bytes();
                x.insert(0, x.len().try_into().unwrap());
                x
            })
            .flatten()
            .collect();
        name.push(0);

        name.append(&mut Into::<u16>::into(self.qtype).to_be_bytes().to_vec());
        name.append(&mut Into::<u16>::into(self.qclass).to_be_bytes().to_vec());
        name
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
            _ => {
                return Err(format!(
                    "failed to parse value as QType: {} is not a valid value",
                    value
                ))
            }
        })
    }
}

impl Into<u16> for QType {
    fn into(self) -> u16 {
        match self {
            Self::A => 1,
            Self::NS => 2,
            Self::MD => 3,
            Self::MF => 4,
            Self::CNAME => 5,
            Self::SOA => 6,
            Self::MB => 7,
            Self::MG => 8,
            Self::MR => 9,
            Self::Null => 10,
            Self::WKS => 11,
            Self::PTR => 12,
            Self::HINFO => 13,
            Self::MINFO => 14,
            Self::MX => 15,
            Self::TXT => 16,
            Self::AXFR => 252,
            Self::MAILB => 253,
            Self::MAILA => 254,
        }
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
    type Error = DecodeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            255 => Self::Any,
            _ => {
                return Err(DecodeError::IllegalValue(format!(
                    "failed to parse value as QClass: {} is not a valid value",
                    value
                )))
            }
        })
    }
}

impl Into<u16> for QClass {
    fn into(self) -> u16 {
        match self {
            Self::IN => 1,
            Self::CS => 2,
            Self::CH => 3,
            Self::HS => 4,
            Self::Any => 255,
        }
    }
}
