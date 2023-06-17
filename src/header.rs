#[derive(PartialEq, Debug)]
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

impl TryFrom<&[u8]> for Header {
    type Error = String;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() != 12 {
            return Err("Lenght invalid".to_string());
        }
        println!("{:?}", value);

        let id = u16::from_be_bytes([value[0], value[1]]);
        let qr = Type::try_from(value[2] & 0b1000_0000)?;
        let opcode = OpCode::try_from(value[2] & 0b0111_1000)?;
        let aa = value[2] & 0b1000_0000;
        let tc = value[2] & 0b0000_0100;
        let rd = value[2] & 0b0000_0010;
        let ra = value[2] & 0b0000_0001;
        let z = value[3] & 0b1110_0000;
        let rcode = ResponseCode::try_from(value[3] & 0b0001_1111)?;
        let qd_count = u16::from_be_bytes([value[4], value[5]]);
        let an_count = u16::from_be_bytes([value[6], value[7]]);
        let ns_count = u16::from_be_bytes([value[8], value[9]]);
        let ar_count = u16::from_be_bytes([value[10], value[11]]);

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

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Query,
    IQuery,
    Status,
}

impl TryFrom<u8> for OpCode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::IQuery),
            2 => Ok(Self::Status),
            _ => Err(String::from("Error")),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum Type {
    Query,
    Reply,
}

impl TryFrom<u8> for Type {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Query),
            1 => Ok(Self::Reply),
            _ => Err(String::from("Error")),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ResponseCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused,
}

impl TryFrom<u8> for ResponseCode {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::NoError),
            1 => Ok(Self::FormatError),
            2 => Ok(Self::ServerFailure),
            3 => Ok(Self::NameError),
            4 => Ok(Self::NotImplemented),
            5 => Ok(Self::Refused),
            _ => Err(String::from("Error")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deserialize() {
        let query = [
            144, 200, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 10, 100, 117, 99, 107, 100, 117, 99, 107, 103,
            111, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        let header = Header::try_from(&query[0..12]).unwrap();

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
        )
    }
}
