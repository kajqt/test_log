mod shield;
mod common;
use shield::LogShield;
use std::fs::File;
use rand_core::OsRng; 
use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};

use std::sync::Mutex;
use lazy_static::lazy_static;
#[cfg(test)]
mod tests {
    use super::*;
    // lazy_static! {
        
    //     static ref SIGN_KEY: Mutex<SigningKey> = Mutex::new(SigningKey::random(&mut OsRng));
    //     static ref VERIFY_KEY: Mutex<VerifyingKey> = Mutex::new(VerifyingKey::from(&SIGN_KEY));
    //     static ref DATA: Mutex<Vec<u8>> = Mutex::new(vec![]);
    // }
  
    
    #[test]
    fn simple_signing() {
        let random_key = SigningKey::random(&mut OsRng);
        let init_data = b"Hello, World!";
        let mut ex_logshield = LogShield::default() ;
        ex_logshield.init(random_key);
        ex_logshield.sign(init_data);
        
        let s = ex_logshield.show_signature();
        
    }

    #[test]
    fn signature_verify() {
        let random_key = SigningKey::random(&mut OsRng);
        let init_data = b"Hello, World!";
        let mut ex_logshield = LogShield::default() ;
        ex_logshield.init(random_key);
        ex_logshield.sign(init_data);
        
        let s = ex_logshield.show_signature();
        assert!(ex_logshield.verify_signature(init_data, s));
        
    }

    #[test]
    fn invalid_signature_verify() {
        let random_key = SigningKey::random(&mut OsRng);
        let mut init_data = b"Hello, World!";
        let mut ex_logshield = LogShield::default() ;
        ex_logshield.init(random_key);
        ex_logshield.sign(init_data);
        
        let s
         = ex_logshield.show_signature();

        init_data = b"Hell2, World!";
        assert!(ex_logshield.verify_signature(init_data, s) == false);
    }

    
}