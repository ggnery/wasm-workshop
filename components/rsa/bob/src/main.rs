use std::error::Error;

use bob::Bob;
use host::{Cryptography, Host};

mod bob;
mod host;

fn main() -> Result<(), Box<dyn Error>> {
    let mut host = Host::build("/home/gnery/Desktop/ericsson/wasm-workshop/components/rsa/crypto/target/wasm32-wasip1/release/crypto.wasm")?;
    let bob = Bob::build(&mut host)?;




    Ok(())
}


