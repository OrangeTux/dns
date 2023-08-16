use crate::DecodeError;
use std::iter::Peekable;
use std::slice::Iter;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ResourceRecord {
    /// The domain name to which this record relates to.
    pub name: Vec<u8>,

    /// The type of ResourceRecord.
    pub r#type: Type,

    /// The class of ResourceRecord.
    pub class: Class,

    /// The time in seconds for how much the information in this ResourceRecord is valid for.
    pub ttl: u32,

    /// The lenght of rdata in number of bytes.
    pub rdlength: u16,

    /// The actual information describing the resource. It's format depends on type and class.
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
    pub fn into_bytes(self) -> Vec<u8> {
        let mut output = vec![];
        output.append(&mut self.name.clone());
        output.append(&mut self.r#type.into_bytes());
        output.append(&mut self.class.into_bytes());
        output.append(&mut Vec::from(self.ttl.to_be_bytes()));
        output.append(&mut Vec::from(self.rdlength.to_be_bytes()));
        output.append(&mut self.rdata.clone());

        output
    }
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for ResourceRecord {
    type Error = DecodeError;

    fn try_from(value: &mut Peekable<Iter<'_, u8>>) -> Result<Self, Self::Error> {
        let mut name: Vec<u8> = Vec::with_capacity(1);
        for _ in 0..1 {
            name.push(*value.next().ok_or(DecodeError::NotEnoughBytes)?);
        }

        let r#type: Type = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ])
        .try_into()
        .unwrap();
        //let r#type = Type::NS;
        let class: Class = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ])
        .try_into()
        .unwrap();

        let ttl = u32::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);
        let rdlength = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);

        let mut rdata: Vec<u8> = Vec::with_capacity(rdlength.into());
        for _ in 0..rdlength {
            rdata.push(*value.next().ok_or(DecodeError::NotEnoughBytes)?);
        }

        Ok(ResourceRecord {
            name,
            r#type,
            class,
            ttl,
            rdlength,
            rdata,
        })
    }
}

/// Types used in ResourceRecords.
/// See section 3.2.2 of RFC 1035.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
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
    /// IPv6 host address as defined in RFC 3596 DNS Extensions to Support IP Version 6.
    AAAA,
}

impl TryFrom<u16> for Type {
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
            28 => Self::AAAA,
            _ => {
                return Err(format!(
                    "failed to parse value as Type: {} is not a valid value",
                    value
                ))
            }
        })
    }
}

impl Type {
    fn into_bytes(self) -> Vec<u8> {
        let low_byte: u8 = match self {
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
            Self::AAAA => 28,
        };
        vec![0, low_byte]
    }
}

/// Classes used by ResourceRecords.
/// See section 3.2.4 of RFC 1035.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Class {
    // The Internet.
    IN,
    CS,
    CH,
    HS,
}

impl Class {
    fn into_bytes(self) -> Vec<u8> {
        let low_byte: u8 = match self {
            Self::IN => 1,
            Self::CS => 2,
            Self::CH => 3,
            Self::HS => 4,
        };
        vec![0, low_byte]
    }
}

impl TryFrom<u16> for Class {
    type Error = DecodeError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => {
                return Err(DecodeError::IllegalValue(format!(
                    "failed to parse value as Class: {} is not a valid value",
                    value
                )))
            }
        })
    }
}
