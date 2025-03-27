#[allow(warnings)]
mod bindings;

use bindings::Guest;
use rsa::{ pkcs1::{DecodeRsaPublicKey, EncodeRsaPublicKey}, pkcs8::{DecodePublicKey, LineEnding}, rand_core::OsRng, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::{str::from_utf8, sync::LazyLock};

struct Component;

static BIT_SIZE: usize = 2048;

static PRIVATE_KEY: LazyLock<RsaPrivateKey> = LazyLock::new(|| {
    let mut rng = OsRng;
    RsaPrivateKey::new(&mut rng, BIT_SIZE).expect("Couldn't generate private key")
});

static PUBLIC_KEY: LazyLock<RsaPublicKey> = LazyLock::new(|| {
    RsaPublicKey::from(&*PRIVATE_KEY)
});


impl Guest for Component {
    fn encrypt(message: String, public_key: String) -> Vec<u8> {
        let data = message.as_bytes();
        let public_key =RsaPublicKey::from_pkcs1_pem(&public_key).expect("Couldn't get public key");
        
        let mut rng = OsRng;
        public_key.encrypt(&mut rng, Pkcs1v15Encrypt, data).expect("Couldn't encrypt messsage")
    }
    
    fn decrypt(encrypted_message: Vec<u8>) -> String {
        let result = (*PRIVATE_KEY).decrypt(Pkcs1v15Encrypt, encrypted_message.as_slice()).expect("Coulnd't decrypt message");
        from_utf8(result.as_slice()).unwrap().to_string()
    }

    fn get_public_key() -> String {
        (*PUBLIC_KEY).to_pkcs1_pem(LineEnding::CRLF).expect("Couldn't get public key")
    }
}

bindings::export!(Component with_types_in bindings);
