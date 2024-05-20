
use logshield::shield::LogShield;
use std::mem::size_of_val;
use aes::cipher::generic_array::{typenum::U16, GenericArray};
use std::io::{self, Write, Read};
use p384::{ecdsa::{ Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use hex_literal::hex;
fn main() {

    let shield = LogShield::default();
    let verify_key = shield.read_verify_key_from_file("../casetest/verify_key.json");
    print!("Verifying Key: {:?}\n", verify_key);
    let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

 
    io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");

    


    // let mut seq: u32 = 0;
    // let mut cnt: u32 = 0;
    let mut round = 0;

    // print!("Verifying Key: {:?}\n", verifyingKey);
    while !!!buffer.is_empty() {
        print!("Round: {}\n", round);
        round += 1;
      
        /* 
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
xa
        let buffer = stdin.fill_buf().unwrap();

        // work with buffer
        println!("{buffer:?}");xw

        // ensure the bytes we worked with aren't rxweturned again later
        let length = buffer.len();
        stdin.consume(length);
      */
         
        let mut stdin = io::stdin();
        // let x = hex!("6b9d3dad2e1b8c1c05b19875b6659f4de23c3b667bf297ba9aa47740787137d896d5724e4c70a825f872c9ea60d2edf5");
       
        // println!("The usize of x is {}", size_of_val(&x));
       
        let mut bufhash: [u8; 8] = [0; 8];
        let mut buffer2 = Vec::new();

        

        match stdin.read(&mut bufhash) {
            Ok(0) => break, 
            Ok(n) => buffer2.extend_from_slice(&bufhash[..n]), 
            Err(e) => panic!("Error reading from stdin: {}", e),
        }
        
        println!("The usize of hash is {}", size_of_val(&bufhash));
        print!("Hash: {:?}\n", bufhash);
        let mut bufsig: [u8; 96] = [0; 96];
        buffer2 = Vec::new();
        match stdin.read(&mut bufsig) {
            Ok(0) => break, 
            Ok(n) => buffer2.extend_from_slice(&bufsig[..n]), 
            Err(e) => panic!("Error reading from stdin: {}", e),
        }
        let len = buffer2.len();
        print!("Sig: {:?}\n", bufsig);
        println!("The usize of signature is {}", size_of_val(&bufsig));
        // let example_data = buffer2;
        // let bytes = GenericArray::clone_from_slice(&bufsig);
        // println!("The usize of hash stdin is {}", size_of_val(&bufhash));
        // print!("Signature: {:?}\n", bufsig);

        let sig = Signature::from_slice(&bufsig).unwrap();
        
        if shield.verify_signature_byhash(&bufhash, sig, verify_key){
            println!("Signature is valid");
        } else {
            println!("Signature is not valid");
        }
        


        // println!("line: {:?}", example_data);
        // io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");
        // reader.read_to_end(&mut example_data).unwrap();
        
        // let newOutputBlock = ex_logshield.sign(cnt, seq, &example_data);
        
        // print!("Cnt {cnt}, Seq {seq}, len {len} \n");
        // seq += len as u32; 
        // cnt += 1;
        // ex_logshield.show_signature();

        // let check = ex_logshield.verify_signature(&example_data, newOutputBlock.signature);
        
        // assert!(check, "Signature is not valid");
        // print!("\n-> Verify : OK \n");
        // print!("Done!!\n");
        // stdin = io::stdin().lock();
        // buffer2 = &stdin.fill_buf().unwrap();
    }
    

}