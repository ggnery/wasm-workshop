use std::error::Error;

use crate::host::Host;

pub struct Bob {
    public_key: String
}

impl Bob {
    pub fn build(host: &mut Host)-> Result<Bob, Box<dyn Error>> {
        let public_key = host.get_public_key()?;

        
        println!("Key {}", public_key);    
        Ok(Bob{
            public_key
        })
    }

}