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
            .split('.')
            .flat_map(|part| {
                let mut x = part.to_owned().into_bytes();
                x.insert(0, x.len().try_into().unwrap());
                x
            })
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

impl From<QType> for u16 {
    fn from(val: QType) -> Self {
        match val {
            QType::A => 1,
            QType::NS => 2,
            QType::MD => 3,
            QType::MF => 4,
            QType::CNAME => 5,
            QType::SOA => 6,
            QType::MB => 7,
            QType::MG => 8,
            QType::MR => 9,
            QType::Null => 10,
            QType::WKS => 11,
            QType::PTR => 12,
            QType::HINFO => 13,
            QType::MINFO => 14,
            QType::MX => 15,
            QType::TXT => 16,
            QType::AXFR => 252,
            QType::MAILB => 253,
            QType::MAILA => 254,
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

impl From<QClass> for u16 {
    fn from(val: QClass) -> Self {
        match val {
            QClass::IN => 1,
            QClass::CS => 2,
            QClass::CH => 3,
            QClass::HS => 4,
            QClass::Any => 255,
        }
    }
}
