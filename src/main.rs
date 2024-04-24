use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Read;

use std::error::Error;
// use ed25519::Signature;
// use std::os::unix::ucred::impl_mac;
// use rand::{rngs::OsRng};
// use ring::hmac::sign;
use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};

use rand_core::OsRng; 
// use k256::ecdsa::{SigningKey, Signature,  signature::Signer};
// use signature::{Signer, Verifier};


extern crate aes_gcm;

// use ring::{rand::SystemRandom, signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, Signature, KeyPair}};
// use ring::signature::UnparsedPublicKey;

use aes_gcm::{
    aead::{generic_array::{GenericArray, typenum::U32}},
};



   // pub static ref EXEC_AUTH_AC:  RwLock<ExecAthentityAcChekcer> = RwLock::new(ExecAthentityAcChekcer::default());
// pub static ref STDOUT_EXEC_LOG_SHIELD:  RwLock<LogShield> = RwLock::new(LogShield::default());

pub struct LogShield {
    key: GenericArray<u8, U32>,
    // signing_key_slice: GenericArray<u8, U32>,
    signing_key: SigningKey,
    current_signature: Signature,
}

impl LogShield{

    pub fn init(&mut self, key: SigningKey) -> () {
    
    
       // self.key = policy.unwrap().secret.file_encryption_key.as_bytes().to_vec();
        // self.signing_key = signing_key.clone();
        self.signing_key = key;
    }
    // pub fn encryptStdio (&self, src: DataBuff, inode_id: u64) -> Result<DataBuff> {  

    // }
    // pub fn stream_signing (&self, src: Vec<u8>, inode_id: u64) -> Signature {
    //     let rng = SystemRandom::new();
    //     // let sig = self.signing_key.sign(&rng, &src).unwrap();
    //     return sig;
    // }
}

fn main() {
    let file = File::open("/Users/kaj/Desktop/casetest/src/log.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut data = Vec::new();
    reader.read_to_end(&mut data).unwrap();
/* 
    let rng = SystemRandom::new();
    // Generating ECDSA Key Pair
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref()).unwrap();
    println!("Private Key (PKCS#8):");

    let sig = key_pair.sign(&rng, &data).unwrap();
    println!("Signature:");
    // println!("{:?}", sig.as_ref());
    // Verifying the signature
    let peer_public_key_bytes = key_pair.public_key().as_ref();

    let peer_public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ECDSA_P256_SHA256_FIXED, peer_public_key_bytes);
    peer_public_key.verify(&data, sig.as_ref()).unwrap();
    println!("Signature verified successfully!");
 */
    let key2 = SigningKey::random(&mut OsRng);
    let init_data = b"Hello, World!";
    let init_sig: Signature = key2.sign(init_data);
    let mut log_shield = LogShield {
        key: GenericArray::default(),
        signing_key: key2.clone(),
        current_signature: key2.sign(init_data),
    };
    // let signing_key = SigningKey::random(&mut OsRng);

   
    // log_shield.init(key_pair);
      // Ok(())

}