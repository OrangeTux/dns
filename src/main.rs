use dns::header::{ResponseCode, Type as MessageType};
use dns::message::Message;
use dns::resource_record::{Class, ResourceRecord, Type};
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("localhost:1337")?;

    // Maximum size of UDP packet. See section 2.3.4 of the RFC.
    let mut buf = [0; 512];
    let (amt, src) = socket.recv_from(&mut buf)?;

    let message = Message::try_from(&mut buf.iter()).unwrap();
    let mut response_header = message.header.clone();
    response_header.qr = MessageType::Reply;
    response_header.aa = 1;
    response_header.an_count = 1;
    response_header.rcode = ResponseCode::NoError;

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
