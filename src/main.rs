use dns::header::{Header, OpCode, ResponseCode, Type as MessageType};
use dns::message::Message;
use dns::resource_record::{Class, ResourceRecord, Type};
use dns::sections::Question;
use rand::random;
use std::net::ToSocketAddrs;
use std::net::UdpSocket;

fn look_up<A: ToSocketAddrs>(query: &Question, address: A) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let message = Message {
        header: Header {
            id: random::<u16>(),
            message_type: MessageType::Query,
            op_code: OpCode::Query,
            authoritive_answer: false,
            truncated: false,
            recursion_desired: false,
            recursion_available: false,
            z: 0,
            r_code: ResponseCode::NoError,
            qd_count: 1,
            an_count: 0,
            ns_count: 0,
            ar_count: 0,
        },
        questions: vec![query.clone()],
        answers: vec![],
        authority: vec![],
        additional: vec![],
    };

    socket.send_to(&message.into_bytes(), address)?;
    let mut buf = [0; 512];
    let (amt, src) = socket.recv_from(&mut buf)?;
    dbg!(&amt);
    let message = Message::try_from(&mut buf.iter().peekable()).unwrap();

    dbg!(message);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("localhost:1337")?;

    // Maximum size of UDP packet. See section 2.3.4 of the RFC.
    let mut buf = [0; 512];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let message = Message::try_from(&mut buf.iter().peekable()).unwrap();
    if message.header.message_type == MessageType::Query {
        look_up(&message.questions[0], "198.41.0.4:53")?
    }

    let mut response_header = message.header.clone();
    response_header.message_type = MessageType::Reply;
    response_header.authoritive_answer = false;
    response_header.an_count = 1;
    response_header.r_code = ResponseCode::NoError;

    let response = Message {
        header: response_header,
        questions: message.questions.clone(),
        answers: vec![ResourceRecord {
            name: vec![0xc0, 0x0c],
            r#type: Type::A,
            class: Class::IN,
            ttl: 300,
            rdlength: 4,
            rdata: vec![185, 24, 223, 10],
        }],
        authority: vec![],
        additional: vec![],
    };

    socket.send_to(&response.into_bytes(), src)?;

    Ok(())
}
