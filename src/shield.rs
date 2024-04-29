use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use std::fs::File;

use std::io::{self, BufRead, BufReader};
use std::io::Read;
use log::*;
use std::error::Error;
// use ed25519::Signature;
// use std::os::unix::ucred::impl_mac;
// use rand::{rngs::OsRng};
// use ring::hmac::sign;


use rand_core::OsRng; 
// use k256::ecdsa::{SigningKey, Signature,  signature::Signer};
// use signature::{Signer, Verifier};


extern crate aes_gcm;

// use ring::{rand::SystemRandom, signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, Signature, KeyPair}};
// use ring::signature::UnparsedPublicKey;

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

    pub fn sign(&mut self, data: &[u8]) -> Signature {
        let file = File::open("/Users/kaj/Desktop/phd_code/casetest/src/log.txt").unwrap();
        let mut reader = BufReader::new(file);

        let mut example_data = Vec::new();
        reader.read_to_end(&mut example_data).unwrap();

        let key2 = SigningKey::random(&mut OsRng);
        let init_data = b"Hello, World!";
        let init_sig: Signature = key2.sign(init_data);
        // let mut log_shield = LogShield {
        //     key: GenericArray::default(),
        //     signing_key: key2.clone(),
        //     current_signature: key2.sign(init_data),
        // };

        self.current_signature = key2.sign(data);
        return self.current_signature;
    }

    pub fn show_signature(&self) -> Signature {
        print!("The latest signature {:?}", self.current_signature);
        return self.current_signature;
        
    }

    pub fn verify_signature(&self, data: &[u8], sig: Signature) -> bool {
        let verify_key = VerifyingKey::from(&self.signing_key);
        verify_key.verify(data, &sig).is_ok()
    }  
    // pub fn encryptStdio (&self, src: DataBuff, inode_id: u64) -> Result<DataBuff> {  

    // }
    // pub fn stream_signing (&self, src: Vec<u8>, inode_id: u64) -> Signature {
    //     let rng = SystemRandom::new();
    //     // let sig = self.signing_key.sign(&rng, &src).unwrap();
    //     return sig;
    // }
}

