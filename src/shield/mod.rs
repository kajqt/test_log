// use ecdsa::VerifyingKey;
use p384::{ecdsa::{signature::{Signer, Verifier}, Signature, SigningKey, VerifyingKey}, pkcs8::der::Encode};
use std::fs::File;
use anyhow::{anyhow, Result};
use std::io::{self, BufRead, BufReader};
use std::io::Read;
use log::*;
use std::error::Error;
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::mem;
use std::mem::size_of_val;
// use crate::aes_gcm::{
//     aead::{Aead, KeyInit, OsRng, generic_array::{GenericArray, typenum::U32}, rand_core::RngCore},
//     Aes256Gcm, Nonce, // Or `Aes128Gcm`
// };

// use rand_core::OsRng; 

use aes_gcm::{
    aead::{Aead, KeyInit, OsRng, generic_array::{GenericArray, typenum::U32}, rand_core::RngCore},
    Aes256Gcm, Nonce, // Or `Aes128Gcm`
};


pub struct SignedBlock{
    pub nBlocks: u32,
    pub seq: u32,
    pub enrypted_data: Vec<u8>,
    pub signature: Signature,
    pub length: usize,
    pub mac: Vec<u8>
}

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

const NONCE_LENGTH: usize = 12;

impl LogShield{

    pub fn init(&mut self, key: SigningKey) -> () {
       // self.key = policy.unwrap().secret.file_encryption_key.as_bytes().to_vec();
        // self.signing_key = signing_key.clone();
        self.signing_key = key;
        let verify_key = VerifyingKey::from(&self.signing_key);
        let serialized = verify_key.to_sec1_bytes();
        fs::write("verify_key.json", serialized).expect("Unable to write file");
        // fs.close()
        print!("Verify Key <in daemon>: {:?}\n", verify_key);

    }

    pub fn gen_new_random_key(&mut self) -> () {
    
        self.signing_key = SigningKey::random(&mut OsRng);
    }

    pub fn gen_random_enc_key(&mut self) -> () {
    
        self.key = Aes256Gcm::generate_key(OsRng);

    }

    pub fn encrypt(&mut self, plain_txt: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
        let cipher = Aes256Gcm::new(&self.key);
    
        let mut nonce_rnd = vec![0; NONCE_LENGTH];
        OsRng.fill_bytes(&mut nonce_rnd);
        // random_bytes(&mut nonce_rnd);
        let nonce = Nonce::from_slice(&nonce_rnd);
    
        let encrypt_msg = cipher.encrypt(nonce, plain_txt).unwrap();
    
        // let mut cipher_txt = Vec::new();
        // cipher_txt.extend_from_slice(&nonce_rnd);
        // cipher_txt.extend(encrypt_msg);
        Ok((encrypt_msg, nonce_rnd.to_vec()))
    }

    pub fn decrypt(&mut self, cipher_txt: &[u8], nonce: &[u8]) -> Result<Vec<u8>> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(nonce);
        let plain_txt = cipher.decrypt(nonce, cipher_txt.as_ref());
        Ok(plain_txt.unwrap())
    }

    pub fn sign(&mut self, cnt: u32, seq: u32, data: &[u8]) -> SignedBlock {
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(&data, &mut hasher);
        let x =  hasher.finish();
        // print hash value
        // println!(" Show hash {:?}", &x.to_be_bytes());
        // println!("The usize of hash is {}", size_of_val(&x));
        self.current_signature = self.signing_key.sign(&x.to_be_bytes());
        // let verifyKey = VerifyingKey::from(&self.signing_key);
        // print!("Verifying Key: {:?}\n", verifyKey);
        // let a = verifyKey.verify(&x.to_be_bytes(), &self.current_signature ).is_ok();
        // print!("Verify???: {:?}\n", a);
        // println!(" sig??? {:?}",  &self.current_signature) ;
        let block = SignedBlock{
            nBlocks: cnt,
            seq: seq,
            enrypted_data: data.to_vec(),
            signature: self.current_signature,
            length: data.len(),
            mac: x.to_be_bytes().to_vec()
        };
        return block;
    }

    pub fn show_signature(&self) -> Signature {
        // Print out signature
        // println!("Show sig {:?}", self.current_signature.to_bytes().as_slice());
        // println!("The usize of signature is {}", size_of_val(self.current_signature.to_bytes().as_slice()));
        return self.current_signature;
        
    }

    pub fn verify_signature(&self, data: &[u8], sig: Signature) -> bool {

        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(&data, &mut hasher);
        let hash =  hasher.finish();
        let verify_key = VerifyingKey::from(&self.signing_key);
        
        
        verify_key.verify(&hash.to_be_bytes(), &sig).is_ok()
 
    }  

    pub fn verify_signature_byhash(&self, hash: &[u8], sig: Signature, vKey: VerifyingKey) -> bool {
        // let mut hasher = DefaultHasher::new();
        // Hash::hash_slice(&data, &mut hasher);
        // let hash =  hasher.finish();
        // let verify_key = VerifyingKey::from(&self.signing_key);
        
    
        vKey.verify(hash, &sig).is_ok()
 
    }  

    pub fn verify_block(&self, data: &[u8], block: SignedBlock) -> bool {
        let mut hasher = DefaultHasher::new();
        Hash::hash_slice(&data, &mut hasher);
        let hash =  hasher.finish();
        let verify_key = VerifyingKey::from(&self.signing_key);
        verify_key.verify(&hash.to_be_bytes(), &block.signature).is_ok()
    }   

    // ...

    pub fn read_verify_key_from_file(&self, file_path: &str) -> VerifyingKey {
        // Read the file into a String
        let data: Vec<u8> = fs::read(file_path).unwrap();

        // Deserialize the String into a VerifyingKey
        let verify_key: VerifyingKey = VerifyingKey::from_sec1_bytes(&data).expect("Failed to deserialize verify_key");

        verify_key
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

// AES-GCM
// HASH for each block
// Data -> divide into blocks -> encrypt each block -> hash -> sign each block -> write to disk