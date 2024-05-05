use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use std::fs::File;

use std::io::{self, BufRead, BufReader};
use std::io::Read;
use log::*;
use std::error::Error;



use rand_core::OsRng; 
extern crate aes_gcm;


use aes_gcm::{
    aead::{generic_array::{GenericArray, typenum::U32}},
};




#[derive(Debug)]
pub struct LogShield {
    key: GenericArray<u8, U32>,
    // signing_key_slice: GenericArray<u8, U32>,
    signing_key: SigningKey,
    current_signature: Signature,
}

pub fn create_default_signing_key() -> SigningKey {
    let key = SigningKey::random(&mut OsRng);
    return key;
}

pub fn create_default_signature() -> Signature {
    let key = SigningKey::random(&mut OsRng);
    let init_data = b"Hello, World!";
    let init_sig: Signature = key.sign(init_data);
    return init_sig;
}

impl Default for LogShield {
    fn default() -> Self {
        Self {
            key: GenericArray::default(),
            signing_key: create_default_signing_key(), // replace with your function
            current_signature: create_default_signature(), // replace with your function
        }
    }
}

impl LogShield{

    pub fn init(&mut self, key: SigningKey) -> () {
       // self.key = policy.unwrap().secret.file_encryption_key.as_bytes().to_vec();
        // self.signing_key = signing_key.clone();
        self.signing_key = key;
    }

    pub fn gen_new_random_key(&mut self) -> () {
    
        self.signing_key = SigningKey::random(&mut OsRng);
     }

    pub fn sign(&mut self, data: &[u8]) -> Signature {
       


        self.current_signature = self.signing_key.sign(data);
        return self.current_signature;
    }

    pub fn show_signature(&self) -> Signature {
        print!("The signature \n {:?}", self.current_signature);
        return self.current_signature;
        
    }

    pub fn verify_signature(&self, data: &[u8], sig: Signature) -> bool {
        let verify_key = VerifyingKey::from(&self.signing_key);
        verify_key.verify(data, &sig).is_ok()
    }  

    // pub fn stream_signing (&self, src: Vec<u8>, inode_id: u64) -> Signature {
    //     let rng = OsRng;
    //     let sig = self.signing_key.sign(&rng, &src).unwrap();
    //     return sig;
    // }
    // pub fn encryptStdio (&self, src: DataBuff, inode_id: u64) -> Result<DataBuff> {  

    // }
    // pub fn stream_signing (&self, src: Vec<u8>, inode_id: u64) -> Signature {
    //     let rng = SystemRandom::new();
    //     // let sig = self.signing_key.sign(&rng, &src).unwrap();
    //     return sig;
    // }
}

