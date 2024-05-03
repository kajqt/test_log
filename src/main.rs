mod shield;
use shield::LogShield;
use std::fs::File;
use rand_core::OsRng; 
use p256::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
use std::{
    io::{BufReader, BufRead},
    process::{Command, Stdio},
};

use std::io::{self, Write, Read};
use tokio::task;

use atty::Stream;

// fn load_stdin() -> io::Result<String> {
//     if atty::is(Stream::Stdin) {
//         return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
//     }
//     let mut buffer = String::new();
//     io::stdin().read_line(&mut buffer)?;
//     return Ok(buffer);
// }

// #[tokio::main]
fn main() {
    /* 
    let file = File::open("/Users/kaj/Desktop/phd_code/casetest/src/logfile.log").unwrap();
    let mut reader = BufReader::new(file);

    let mut example_data = Vec::new();
    reader.read_to_end(&mut example_data).unwrap();
    
    */  
     if atty::is(Stream::Stdin) {
        // return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }

    let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);

    let _ = io::stdout().flush();
    io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");
    let random_key = SigningKey::random(&mut OsRng);
    let mut ex_logshield = LogShield::default();
    ex_logshield.init(random_key);

    let mut counter = 0;
    while !!!buffer.is_empty() {
        // println!("line: {}", buffer);
        let example_data = buffer.as_bytes();
        // io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");
        // reader.read_to_end(&mut example_data).unwrap();
        
        let s = ex_logshield.sign(&example_data);
        
        print!("Sign {counter}\n");
        counter +=1;
        ex_logshield.show_signature();

        let check = ex_logshield.verify_signature(&example_data, s);
        assert!(check, "Signature is not valid");
        print!("\nValid signature \n");
        print!("Done!!\n");
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