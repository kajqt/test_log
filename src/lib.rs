mod shield;
use shield::LogShield;
use std::fs::File;
use rand_core::OsRng; 
use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sign_test() {
        let random_key = SigningKey::random(&mut OsRng);
        let init_data = b"Hello, World!";
        let mut ex_logshield = LogShield::default() ;
        ex_logshield.init(random_key);
        ex_logshield.sign(init_data);
        
        let s = ex_logshield.show_signature();
        
    }
}