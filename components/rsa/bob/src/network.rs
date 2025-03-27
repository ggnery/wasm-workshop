use std::{error::Error, io::Write, net::TcpStream};

pub fn send_public_key(stream: &mut TcpStream, public_key: &String ) -> Result<(), Box<dyn Error>> {
    let bytes = public_key.as_bytes();
    let bytes_len = bytes.len() as u64;
    stream.write_all(&bytes_len.to_be_bytes())?;
    stream.write_all(bytes)?;
    stream.flush()?;

    Ok(())
}