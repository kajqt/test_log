mod shield;
use shield::LogShield;
use std::fs::File;
use rand_core::OsRng; 
use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use std::io::{self, BufRead, BufReader};
use std::io::Read;

fn main() {
    let file = File::open("/Users/kaj/Desktop/phd_code/casetest/src/log.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut example_data = Vec::new();
    reader.read_to_end(&mut example_data).unwrap();

    let random_key = SigningKey::random(&mut OsRng);
        let init_data = b"Hello, World!";
        let mut ex_logshield = LogShield::default() ;
        ex_logshield.init(random_key);
        ex_logshield.sign(init_data);
        
        let s = ex_logshield.show_signature();

        ex_logshield.verify_signature(init_data, s);

        print!("\nValid signature \n");

}