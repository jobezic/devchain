use ed25519_dalek::{Keypair, Signature, Signer};
use rand::rngs::OsRng;
use std::fs;

pub struct Wallet {
    pub keypair: Keypair,
}

impl Wallet {
    pub fn load_or_create(path: &str) -> Self {
        if let Ok(bytes) = fs::read(path) {
            let keypair = Keypair::from_bytes(&bytes).expect("Invalid keypair file");
            Wallet { keypair }
        } else {
            let mut csprng = OsRng {};
            let keypair: Keypair = Keypair::generate(&mut csprng);
            fs::write(path, keypair.to_bytes()).expect("Unable to write keypair");
            Wallet { keypair }
        }
    }

    pub fn address(&self) -> String {
        hex::encode(self.keypair.public.as_bytes())
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.keypair.sign(message)
    }
}
