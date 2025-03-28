use std::{error::Error, net::TcpListener};
use host::Host;
use network::{recieve_encrypted_message, send_public_key};

mod host;
mod network;

const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";

fn main() -> Result<(), Box<dyn Error>> {
    let mut host = Host::build("crypto.wasm")?;
    let public_key = host.get_public_key()?; 
    
    // Start server
    let address = format!("{}:{}", HOST, PORT);
    let listener = TcpListener::bind(&address)?;
    println!("Bob listening on {}", address);

    //Handle each connection
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Alice connected in {}\n", stream.peer_addr()?);

        send_public_key(&mut stream, &public_key)?;      
        
        loop {
            //Recieve encrypted_message from alice
            let encrypted_message = match recieve_encrypted_message(&mut stream) {
                Ok(v) => v,
                Err(_)  => {
                    break;
                }
            };  

            let original_msg = host.decrypt(encrypted_message)?;
            print!("Alice message: {}", original_msg);    
        }    
    }

    Ok(())
}


