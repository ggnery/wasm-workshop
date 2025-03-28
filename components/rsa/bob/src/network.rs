use std::{error::Error, io::{Read, Write}, net::TcpStream};

pub fn send_public_key(stream: &mut TcpStream, public_key: &String ) -> Result<(), Box<dyn Error>> {
    let bytes = public_key.as_bytes();
    stream.write_all(bytes)?;
    stream.flush()?;

    Ok(())
}

pub fn recieve_encrypted_message(stream: &mut TcpStream) -> Result<Vec<u8>, Box<dyn Error>> {
    //Read Alice message len
    let mut buff_size: [u8; 4] = [0; 4];

    match stream.read_exact(&mut buff_size) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.into());
        }
    };
    let size = i32::from_be_bytes(buff_size) as usize;

    //Alocate a buff to recieve Alice message
    let mut buff: Vec<u8> = vec![0; size];
    match stream.read_exact(&mut buff) {
        Ok(_) => {},
        Err(e)  => {
            return Err(e.into());
        }
    };
    
    Ok(buff)
}
