use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Read;
extern crate sha2;
extern crate ed25519_dalek;

use std::error::Error;
use rand::{rngs::OsRng};
use sha2::Sha512;

// use ed25519_dalek::SigningKey;
// use ed25519_dalek::{Signature, Signer};

extern crate ring;

use ring::{rand::SystemRandom, signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, Signature, KeyPair}};
// use ring::signature::UnparsedPublicKey;

fn main() -> Result<(),  Box<dyn Error>>  {
    let file = File::open("/Users/kaj/Desktop/casetest/src/log.txt")?;
    let mut reader = BufReader::new(file);

    let mut data = Vec::new();
    reader.read_to_end(&mut data)?;

    let rng = SystemRandom::new();
    // Generating ECDSA Key Pair
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng)?;
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref())?;
    println!("Private Key (PKCS#8):");

    let sig = key_pair.sign(&rng, &data)?;
    println!("Signature:");
    // println!("{:?}", sig.as_ref());
    // Verifying the signature
    let peer_public_key_bytes = key_pair.public_key().as_ref();


    let peer_public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ECDSA_P256_SHA256_FIXED, peer_public_key_bytes);
    peer_public_key.verify(&data, sig.as_ref())?;
    println!("Signature verified successfully!");

    Ok(())


}