use logshield::shield::LogShield;
// use ecdsa::VerifyingKey;
use std::fs::File;
use rand_core::OsRng; 
use p384::{ecdsa::{SigningKey, Signature, signature::Signer, VerifyingKey, signature::Verifier}};
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
    let test_key_slice = [242, 112, 248, 169, 20, 121, 158, 91, 21, 132, 251, 106, 224, 192, 145, 184, 232, 194, 2, 183, 19, 246, 94, 106, 132, 8, 22, 12, 10, 206, 246, 243, 175, 197, 171, 111, 134, 78, 113, 60, 71, 143, 234, 108, 1, 1, 23, 192];
    let test_key = SigningKey::from_slice(&test_key_slice).unwrap();
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

    // print!("Random Key: {:?}\n", random_key.to_bytes() );
    let mut ex_logshield = LogShield::default();
    // ex_logshield.init(random_key);
    ex_logshield.init(test_key.clone());

    let mut seq: u32 = 0;
    let mut cnt: u32 = 0;
    let testhash1 = [169, 133, 84, 135, 179, 248, 125, 194];
        let testsig1 =  [121, 108, 222, 205, 166, 178, 88, 220, 144, 50, 104, 253, 69, 53, 114, 19, 125, 14, 129, 113, 127, 143, 65, 145, 21, 18, 83, 135, 132, 8, 171, 103, 176, 5, 135, 213, 230, 143, 176, 62, 252, 102, 129, 45, 218, 65, 127, 6, 9, 254, 247, 160, 146, 248, 142, 61, 198, 8, 253, 134, 213, 21, 239, 186, 107, 48, 220, 230, 162, 15, 254, 127, 31, 254, 225, 144, 84, 83, 102, 32, 130, 194, 129, 31, 211, 41, 93, 167, 181, 91, 157, 75, 174, 166, 92, 145];
        let testhash2 = [207, 110, 77, 196, 157, 96, 0, 204];
        let testsig2 = [87, 80, 246, 213, 109, 8, 158, 111, 217, 228, 177, 178, 250, 13, 139, 7, 193, 54, 242, 48, 189, 204, 21, 122, 194, 13, 51, 35, 55, 154, 10, 88, 88, 53, 45, 105, 240, 88, 168, 49, 155, 194, 72, 84, 172, 184, 213, 171, 68, 67, 54, 90, 69, 33, 36, 77, 15, 27, 177, 151, 162, 133, 240, 53, 79, 43, 106, 180, 55, 4, 123, 75, 92, 130, 182, 174, 167, 49, 189, 221, 161, 122, 172, 109, 61, 114, 203, 156, 130, 110, 94, 205, 120, 159, 69, 180];

        // println!("{:?}", testhash1);
        let mut stdout: io::Stdout = io::stdout();
        stdout.write(&testhash1);
        // println!("{:?}", testsig1);
        stdout.write(&testsig1);
        // println!("{:?}", testhash2);
        // println!("{:?}", testsig2);
        let sig1 = Signature::from_slice(&testsig1).unwrap();
        // print!("Test Sig1: {:?}\n", sig1);
        let verify_key = VerifyingKey::from(&test_key);
        let check1 = ex_logshield.verify_signature_byhash(&testhash1, sig1, verify_key);
        // print!("Check1: {}\n", check1);
    let a = 1;
    if a == 1{

    

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
        
        let newOutputBlock = ex_logshield.sign(cnt, seq, &example_data);
        
        // print!("Cnt {cnt}, Seq {seq}, len {len} \n");
        seq += len as u32; 
        cnt += 1;
        ex_logshield.show_signature();

        let check = ex_logshield.verify_signature(&example_data, newOutputBlock.signature);
        
        // assert!(check, "Signature is not valid");
        // print!("\n-> Verify : OK \n");
        // print!("{cnt} Done!!\n");
        // stdin = io::stdin().lock();
        // buffer2 = &stdin.fill_buf().unwrap();
        if cnt == 2 {
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


