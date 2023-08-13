//! Serialize and deserialize `Header`s.
use crate::DecodeError;
use std::iter::Peekable;
use std::slice::Iter;

/// Header of a DNS message.
///
/// The impementation is based on section [`4.1.1. Header section format`] of RFC 1035.
///
/// [`4.1.1. Header section format`]: https://www.rfc-editor.org/rfc/rfc1035#section-4.1.1
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Header {
    /// An identifier used to match query with reply.
    pub id: u16,

    /// Whether packet contains a query or a reply.
    pub message_type: Type,

    /// Field indicating the kind of query.
    pub op_code: OpCode,

    /// Indicate that the responding name server is authoritive for the domain name.
    /// May only be `true` in responses.
    pub authoritive_answer: bool,

    /// Whether the message has been truncated or not.
    pub truncated: bool,

    /// Whether name server should recursively resolve the domain name.
    pub recursion_desired: bool,

    /// Whether  name server supports recursive queries.
    /// May only be `true` in responses.
    pub recursion_available: bool,

    /// Reserved for future use. Must be 0.
    pub z: u8,

    /// Response code.
    /// May only be set in response.
    pub r_code: ResponseCode,

    /// Number of questions.
    pub qd_count: u16,

    /// Number of answers resource records.
    pub an_count: u16,

    /// Number of authority resource records.
    pub ns_count: u16,

    /// Number of additional resource records.
    pub ar_count: u16,
}

impl TryFrom<&mut Peekable<Iter<'_, u8>>> for Header {
    type Error = DecodeError;

    fn try_from(value: &mut Peekable<Iter<u8>>) -> Result<Self, Self::Error> {
        let id = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);

        let byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        let qr = Type::try_from((byte & 0b1000_0000) >> 7)?;
        let opcode = OpCode::try_from((byte & 0b0111_1000) >> 3)?;
        let aa = (byte & 0b0000_0100) >> 2;
        let tc = (byte & 0b0000_0010) >> 1;
        let rd = byte & 0b0000_0001;

        let byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        let ra = (byte & 0b1000_0000) >> 7;
        let z = (byte & 0b0111_0000) >> 4;
        let rcode = ResponseCode::try_from(byte & 0b0000_1111)?;
        let qd_count = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);
        let an_count = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);
        let ns_count = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);
        let ar_count = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);

        let header = Header {
            id,
            message_type: qr,
            op_code: opcode,
            authoritive_answer: aa == 1,
            truncated: tc == 1,
            recursion_desired: rd == 1,
            recursion_available: ra == 1,
            z,
            r_code: rcode,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        };

        Ok(header)
    }
}

impl Header {
    pub fn into_bytes(self) -> Vec<u8> {
        let mut header = Vec::with_capacity(12);
        header.append(&mut self.id.to_be_bytes().to_vec());

        let mut byte: u8 = 0;
        byte += Into::<u8>::into(self.message_type) << 7;
        byte += Into::<u8>::into(self.op_code) << 3;
        byte += Into::<u8>::into(self.authoritive_answer) << 2;
        byte += Into::<u8>::into(self.truncated) << 1;
        byte += Into::<u8>::into(self.recursion_desired);
        header.push(byte);

        let mut byte: u8 = 0;
        byte += Into::<u8>::into(self.recursion_available) << 7;
        byte += Into::<u8>::into(self.z) << 4;
        byte += Into::<u8>::into(self.r_code);
        header.push(byte);

        header.append(&mut self.qd_count.to_be_bytes().to_vec());
        header.append(&mut self.an_count.to_be_bytes().to_vec());
        header.append(&mut self.ns_count.to_be_bytes().to_vec());
        header.append(&mut self.ar_count.to_be_bytes().to_vec());

        header
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpCode {
    Query,
    IQuery,
    Status,
}

impl TryFrom<u8> for OpCode {
    type Error = DecodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::IQuery),
            2 => Ok(Self::Status),
            _ => Err(DecodeError::IllegalValue(format!(
                "failed to parse value as OpCode: {} is not a valid value",
                value
            ))),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(val: OpCode) -> Self {
        match val {
            OpCode::Query => 0,
            OpCode::IQuery => 1,
            OpCode::Status => 2,
        }
    }
}
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Type {
    Query,
    Reply,
}

impl TryFrom<u8> for Type {
    type Error = DecodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::Reply),
            _ => Err(DecodeError::IllegalValue(format!(
                "failed to parse value as Type : {} is not a valid value",
                value
            ))),
        }
    }
}

impl From<Type> for u8 {
    fn from(val: Type) -> Self {
        match val {
            Type::Query => 0,
            Type::Reply => 1,
        }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ResponseCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}

impl TryFrom<u8> for ResponseCode {
    type Error = DecodeError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::FormatError),
            2 => Ok(Self::ServerFailure),
            3 => Ok(Self::NameError),
            4 => Ok(Self::NotImplemented),
            5 => Ok(Self::Refused),
            _ => Err(DecodeError::IllegalValue(format!(
                "failed to parse value as ResponseCode: {} is not a valid value",
                value
            ))),
        }
    }
}

impl From<ResponseCode> for u8 {
    fn from(val: ResponseCode) -> Self {
        match val {
            ResponseCode::NoError => 0,
            ResponseCode::FormatError => 1,
            ResponseCode::ServerFailure => 2,
            ResponseCode::NameError => 4,
            ResponseCode::NotImplemented => 5,
            ResponseCode::Refused => 6,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let mut query = [
            144, 200, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 10, 100, 117, 99, 107, 100, 117, 99, 107, 103,
            111, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ]
        .iter()
        .peekable();
        let header = Header::try_from(&mut query).unwrap();

        assert_eq!(
            header,
            Header {
                id: 37064,
                message_type: Type::Query,
                op_code: OpCode::Query,
                authoritive_answer: false,
                truncated: false,
                recursion_desired: true,
                recursion_available: false,
                z: 0,
                r_code: ResponseCode::NoError,
                qd_count: 1,
                an_count: 0,
                ns_count: 0,
                ar_count: 0,
            }
        );

        let question = crate::sections::Question::try_from(&mut query).unwrap();
        assert_eq!(
            question,
            crate::sections::Question {
                qname: "duckduckgo.com".to_string(),
                qtype: crate::sections::QType::A,
                qclass: crate::sections::QClass::IN,
            }
        );
    }
}
