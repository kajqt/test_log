mod shield;
use shield::LogShield;
use std::fs::File;
use rand_core::OsRng; 
use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use std::{
    io::{BufReader, BufRead},
    process::{Command, Stdio},
};

use std::hash::{DefaultHasher, Hash, Hasher};
use std::io::{self, Write, Read};
use tokio::task;

use atty::Stream;


// #[tokio::main]
fn main() {
  /* 
    let file = File::open("/Users/kaj/Desktop/phd_code/casetest/src/logfile.log").unwrap();
    let mut reader = BufReader::new(file);

    let mut example_data = Vec::new();
    reader.read_to_end(&mut example_data).unwrap();
    
    */  


    let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");

    

    let random_key = SigningKey::random(&mut OsRng);
    let mut ex_logshield = LogShield::default();
    ex_logshield.init(random_key);

    let mut counter: u32 = 0;
    while !!!buffer.is_empty() {
      
        /* 
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let buffer = stdin.fill_buf().unwrap();

        // work with buffer
        println!("{buffer:?}");

        // ensure the bytes we worked with aren't returned again later
        let length = buffer.len();
        stdin.consume(length);
      */
         
        let mut stdin = io::stdin();
       
        let mut buf = [0; 1024];
        let mut buffer2 = Vec::new();
        match stdin.read(&mut buf) {
            Ok(0) => break, 
            Ok(n) => buffer2.extend_from_slice(&buf[..n]), 
            Err(e) => panic!("Error reading from stdin: {}", e),
        }
        let example_data = buffer2;
        
        


        // println!("line: {:?}", example_data);
        // io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");
        // reader.read_to_end(&mut example_data).unwrap();
        
        let newOutputBlock = ex_logshield.sign(counter, &example_data);
        
        print!("Cnt {counter}\n");
        counter +=1;
        ex_logshield.show_signature();

        let check = ex_logshield.verify_signature(&example_data, newOutputBlock.signature);
        
        assert!(check, "Signature is not valid");
        print!("\n-> Verify : OK \n");
        // print!("Done!!\n");
        // stdin = io::stdin().lock();
        // buffer2 = &stdin.fill_buf().unwrap();
    }
  
    // Ok(())
/* 
    let mut cmd = Command::new("cat")
        // .arg("-f")
        .arg("/Users/kaj/Desktop/phd_code/casetest/src/logfile.log")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = cmd.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);
    
            // for line in reader.lines() {
            //     if let Ok(line) = line {
            //         let msg_content = format!("{}\n", line.to_owned());
            //         print!("{}", msg_content);
            //     }
            // }
    print!("_______________________________\n");
    let mut example_data = Vec::new();
    
    
    */

}


