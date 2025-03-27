use std::error::Error;
use host::Host;

mod host;

fn main() -> Result<(), Box<dyn Error>> {
    let mut host = Host::build("/home/gnery/Desktop/ericsson/wasm-workshop/components/rsa/crypto/target/wasm32-wasip1/release/crypto.wasm")?;
    let public_key = host.get_public_key()?; 
    


    Ok(())
}


