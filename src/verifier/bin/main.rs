
use logshield::shield::LogShield;
use std::mem::size_of_val;
use aes::cipher::generic_array::{typenum::U16, GenericArray};
use std::io::{self, Write, Read};
use p384::{ecdsa::{ Signature, signature::Signer, VerifyingKey, signature::Verifier}};
fn main() {

    let shield = LogShield::default();
    let verify_key = shield.read_verify_key_from_file("../casetest/verify_key.json");
    let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");

    


    // let mut seq: u32 = 0;
    // let mut cnt: u32 = 0;


    // print!("Verifying Key: {:?}\n", verifyingKey);
    while !!!buffer.is_empty() {
      
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
       
        let mut bufhash = [0; 8];
        let mut buffer2 = Vec::new();
        match stdin.read(&mut bufhash) {
            Ok(0) => break, 
            Ok(n) => buffer2.extend_from_slice(&bufhash[..n]), 
            Err(e) => panic!("Error reading from stdin: {}", e),
        }
        let bufsig: [u8; 384] = [0; 384];
        match stdin.read(&mut bufhash) {
            Ok(0) => break, 
            Ok(n) => buffer2.extend_from_slice(&bufhash[..n]), 
            Err(e) => panic!("Error reading from stdin: {}", e),
        }
        let len = buffer2.len();
       
        println!("The usize of signature is {}", size_of_val(&bufsig));
        let example_data = buffer2;
        // let bytes = GenericArray::clone_from_slice(&bufsig);
        // println!("The usize of hash stdin is {}", size_of_val(&bufhash));
        

        let sig = Signature::from_slice(&bufsig).unwrap();
        shield.verify_signature(&bufhash, sig);
        


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