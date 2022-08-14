use crate::entities::Transaction;
use crate::utils::hash_string_sha256;
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use std::fs::File;
use std::io::Write;
use std::{fs, io};

pub struct Wallet {
    pub id: String,
    keypair: Option<PKey<Private>>,
}

impl Wallet {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            keypair: None,
        }
    }

    pub fn create_keys(&mut self) {
        let (id, keypair) = Wallet::generate_keys();
        self.id = id;
        self.keypair = Some(keypair);
    }

    pub fn load_keys(&mut self) -> Result<(), io::Error> {
        let file = fs::read("keypair")?;
        let keypair = PKey::private_key_from_der(file.as_slice()).unwrap();
        let txt_public = String::from_utf8(keypair.public_key_to_pem().unwrap()).unwrap();
        let id = txt_public.split("-----").collect::<Vec<&str>>();
        let id = id.get(2).unwrap();
        self.id = id.to_string();
        self.keypair = Some(keypair);
        Ok(())
    }

    fn generate_keys() -> (String, PKey<Private>) {
        // rsa
        let rsa = Rsa::generate(2048).unwrap();

        // generate keypair
        let keypair = PKey::from_rsa(rsa).unwrap();

        let txt_public = String::from_utf8(keypair.public_key_to_pem().unwrap()).unwrap();
        let id = txt_public.split("-----").collect::<Vec<&str>>();
        let id = id.get(2).unwrap();

        let mut file = File::create("keypair").expect("Error");
        file.write_all(keypair.private_key_to_der().unwrap().as_slice())
            .expect("Error");

        (id.to_string(), keypair)
    }

    pub fn sign_transaction(&self, sender: &str, recipient: &str, amount: &str) -> String {
        let mut signer =
            Signer::new(MessageDigest::sha256(), self.keypair.as_ref().unwrap()).unwrap();
        let builder = format!("{}{}{}", sender, recipient, amount);
        let builder = hash_string_sha256(&builder);
        signer.update(builder.as_bytes()).unwrap();
        let mut buf = [0u8; 512];
        let signature = signer.sign_to_vec().unwrap();
        base16ct::lower::encode_str(&signature, &mut buf)
            .unwrap()
            .to_string()
    }

    pub fn verify_transaction(transaction: &Transaction) -> bool {
        let file = fs::read("keypair").unwrap();
        let keypair = PKey::private_key_from_der(file.as_slice()).unwrap();

        let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
        let builder = format!(
            "{}{}{}",
            transaction.sender, transaction.recipient, transaction.amount
        );
        let builder = hash_string_sha256(&builder);
        verifier.update(builder.as_bytes()).unwrap();
        let signature = base16ct::lower::decode_vec(&transaction.signature).unwrap();
        verifier.verify(&signature).unwrap()
    }
}

