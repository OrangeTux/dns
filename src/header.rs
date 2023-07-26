use dns::DecodeError;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Header {
    /// An identifier used to match query with reply.
    pub id: u16,

    // Whether packet contains a query or a reply.
    pub qr: Type,

    pub opcode: OpCode,

    pub aa: u8,
    pub tc: u8,
    pub rd: u8,
    pub ra: u8,
    pub z: u8,
    pub rcode: ResponseCode,

    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl TryFrom<&mut std::slice::Iter<'_, u8>> for Header {
    type Error = DecodeError;

    fn try_from(value: &mut std::slice::Iter<u8>) -> Result<Self, Self::Error> {
        let id = u16::from_be_bytes([
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
            *value.next().ok_or(DecodeError::NotEnoughBytes)?,
        ]);
        let byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        let qr = Type::try_from(byte & 0b1000_0000)?;
        let opcode = OpCode::try_from(byte & 0b0111_1000)?;
        let aa = byte & 0b1000_0000;
        let tc = byte & 0b0000_0100;
        let rd = byte & 0b0000_0010;
        let ra = byte & 0b0000_0001;
        let byte = *value.next().ok_or(DecodeError::NotEnoughBytes)?;
        let z = byte & 0b1110_0000;
        let rcode = ResponseCode::try_from(byte & 0b0001_1111)?;
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
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode,
            qd_count,
            an_count,
            ns_count,
            ar_count,
        };

        Ok(header)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let mut query = [
            144, 200, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 10, 100, 117, 99, 107, 100, 117, 99, 107, 103,
            111, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ]
        .iter();
        let header = Header::try_from(&mut query).unwrap();

        assert_eq!(
            header,
            Header {
                id: 37064,
                qr: Type::Query,
                opcode: OpCode::Query,
                aa: 0,
                tc: 0,
                rd: 0,
                ra: 1,
                z: 0,
                rcode: ResponseCode::NoError,
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
