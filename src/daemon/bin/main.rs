
use logshield::shield::LogShield;
// use ecdsa::VerifyingKey;

use rand_core::OsRng; 
use p384::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey}};
use std::{
    io::{BufRead},
};


use std::io::{self, Write, Read};





// #[tokio::main]
fn main() {
    let test_key_slice = [242, 112, 248, 169, 20, 121, 158, 91, 21, 132, 251, 106, 224, 192, 145, 184, 232, 194, 2, 183, 19, 246, 94, 106, 132, 8, 22, 12, 10, 206, 246, 243, 175, 197, 171, 111, 134, 78, 113, 60, 71, 143, 234, 108, 1, 1, 23, 192];
    let _test_key = SigningKey::from_slice(&test_key_slice).unwrap();
  /* 
    let file = File::open("/Users/kaj/Desktop/phd_code/casetest/src/logfile.log").unwrap();
    let mut reader = BufReader::new(file);

    let mut example_data = Vec::new();
    reader.read_to_end(&mut example_data).unwrap();
    
    */  


    let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer);
    let mut stdout = io::stdout();
    
    io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");

    

    let random_key = SigningKey::random(&mut OsRng);
    // let random_key = _test_key.clone();

    // print!("Random Key: {:?}\n", random_key.to_bytes() );
    let mut ex_logshield = LogShield::default();
    ex_logshield.init(random_key.clone());
    // ex_logshield.init(test_key.clone());

    let mut seq: u32 = 0;
    let mut cnt: u32 = 0;
   
    let runStream = 1;
    if runStream == 1{

    loop {
         
        let mut stdin = io::stdin();
       
        let mut buf = [0; 1024];
        let mut buffer2 = Vec::new();
        match stdin.read(&mut buf) {
            Ok(0) => break, 
            Ok(n) => buffer2.extend_from_slice(&buf[..n]), 
            Err(e) => panic!("Error reading from stdin: {}", e),
        }
        let len = buffer2.len();
        let example_data = buffer2;
       

        // println!("line: {:?}", example_data);
        // io::stdin().read_line(&mut buffer).expect("Error reading from STDIN");
        // reader.read_to_end(&mut example_data).unwrap();
        
        let new_output_block = ex_logshield.sign(cnt, seq, &example_data);
        // print!("{:?}", new_output_block.mac.as_slice());
        // print!("{:?}", new_output_block.signature.to_bytes());
        // let k = new_output_block.mac.clone();
        // println!("The usize of hash k is {}", size_of_val(&k));
        let _ = stdout.flush();
        let _ = stdout.write(&new_output_block.mac);
        let _ = stdout.write(&new_output_block.signature.to_bytes());
        
        // print!("Cnt {cnt}, Seq {seq}, len {len} \n");
        seq += len as u32; 
        cnt += 1;
        ex_logshield.show_signature();

        let check = ex_logshield.verify_signature(&example_data, new_output_block.signature);
        let verify_key = VerifyingKey::from(&random_key);
        // print!("Verify Key daemon: {:?}\n", verify_key); 
        let check2= ex_logshield.verify_signature_byhash(&new_output_block.mac, new_output_block.signature, verify_key);
        
        assert!(check, "Signature is not valid??");
        assert!(check2, "SignatureHASH is not valid??");
        
        // print!("{cnt} Done!!\n");
        // stdin = io::stdin().lock();
        // buffer2 = &stdin.fill_buf().unwrap();
        if cnt == 1 {
            break ;
        }
    }
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


