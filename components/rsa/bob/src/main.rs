use std::{error::Error, net::TcpListener};
use host::Host;
use network::send_public_key;

mod host;
mod network;

fn main() -> Result<(), Box<dyn Error>> {
    let mut host = Host::build("/home/gnery/Desktop/ericsson/wasm-workshop/components/rsa/crypto/target/wasm32-wasip1/release/crypto.wasm")?;
    let public_key = host.get_public_key()?; 
    
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("Bob listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
    
        let mut stream = stream?;
        println!("Alice connected in {}", stream.peer_addr()?);

        send_public_key(&mut stream, &public_key)?;        
    }

    Ok(())
}


