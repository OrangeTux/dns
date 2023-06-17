mod header;
fn main() -> std::io::Result<()> {
    {
        let query = [
            144, 200, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 10, 100, 117, 99, 107, 100, 117, 99, 107, 103,
            111, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        //println!("{:?}", header_as_bytes[0..size]);
        let header = crate::header::Header::try_from(&query[0..12]).unwrap();
        dbg!(header);

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        //socket.send_to(buf, &src)?;
    } // the socket is closed here
    Ok(())
}
