use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::io::Read;
extern crate sha2;
extern crate ed25519_dalek;

use std::error::Error;
// use std::os::unix::ucred::impl_mac;
use rand::{rngs::OsRng};
use sha2::Sha512;

// use ed25519_dalek::SigningKey;
// use ed25519_dalek::{Signature, Signer};

extern crate ring;

use ring::{rand::SystemRandom, signature::{EcdsaKeyPair, ECDSA_P256_SHA256_FIXED_SIGNING, Signature, KeyPair}};
// use ring::signature::UnparsedPublicKey;

use crate::aes_gcm::{
    aead::{OsRng, generic_array::{GenericArray, typenum::U32}, rand_core::RngCore},
};

#[derive(Debug, Default)]
pub struct LogShiled {
 
    key: GenericArray<u8, U32>,
    signing_key_slice: GenericArray<u8, U32>,
}




impl LogShiled{

    pub fn init(&mut self, key: &GenericArray<u8, U32>) -> () {
    
    
       // self.key = policy.unwrap().secret.file_encryption_key.as_bytes().to_vec();
        // self.signing_key = signing_key.clone();
        self.key = key.clone();
    }

/*
    pub fn encrypNormalIOStdouterr (&self, src: DataBuff, inode_id: u64) -> Result<DataBuff> {

        let inode_checker_locked =  INODE_TRACKER.read();
        inode_checker_locked.isInodeExist(&inode_id);
        let trackedInodeType = inode_checker_locked.getInodeType(&inode_id);

        let arg;
        match trackedInodeType {
            TrackInodeType::Stdout(args) => {
                arg = args;
            },
            TrackInodeType::Stderro (args) => {
                arg = args;
            },
            _ => {
                return Ok(src);
            },
        };



        let user_type = arg.exec_user_type;
        let stdio_type = arg.stdio_type;
    

        debug!("encryptContainerStdouterr 00000000, user_type:{:?}, stdio_type {:?}", user_type, stdio_type);
        // case 0: if this is a unprivileged exec req in single cmd mode
        if user_type.is_some() && user_type.as_ref().unwrap().eq(&UserType::Unprivileged) && stdio_type ==  StdioType::ExecProcessStdio {
            info!("case 0: if this is a unprivileged exec req in single cmd mode, user_type:{:?}, stdio_type {:?}", user_type, stdio_type);
            return Ok(src);
        }

        match stdio_type {
            // case 1: if this is subcontainer stdout / stderr
            StdioType::ContainerStdio => {
                debug!("case 1:if this is subcontainer stdout / stderr, user_type:{:?}, stdio_type {:?}", user_type, stdio_type);
                if self.policy.privileged_user_config.enable_container_logs_encryption == false {
                    return Ok(src);
                }
            },
            // case 2: if this is a privileged exec req in single cmd mode
            StdioType::ExecProcessStdio => {
                debug!("case 2:if this is subcontainer stdout / stderr, user_type:{:?}, stdio_type {:?}", user_type, stdio_type);
                if self.policy.privileged_user_config.exec_result_encryption == false {
                    return Ok(src);
                }
            },
            // case 3: if this is root container stdout / stderr
            StdioType::SandboxStdio => {
                debug!("case 3:if this is root container stdout / stderr, user_type:{:?}, stdio_type {:?}", user_type, stdio_type);
                return Ok(src);
            },
            StdioType::SessionAllocationStdio(ref s) => {
                debug!("case 4:if this is session allocation request, user_type:{:?}, stdio_type {:?}, session {:?}", user_type, stdio_type, s);

                let encoded_session: Vec<u8> = postcard::to_allocvec(s).unwrap();
                let encrypted_session = prepareEncodedIoFrame(&encoded_session[..], &self.key).unwrap();
                // write session to stdout
                let mut buf= DataBuff::New(encrypted_session.len());
                buf.buf = encrypted_session;
                return Ok(buf);
            },
            StdioType::PolicyUpdate(ref update_res) => {


                let exit;
                {
                    exit = EXEC_AUTH_AC.read().auth_session.contains_key(&update_res.session_id);
                }
                debug!("case 5:if this is PolicyUpdate request, user_type:{:?}, stdio_type {:?}, session exist {:?}", user_type, stdio_type, exit);


                let result = if update_res.result {
                    "policy update is succeed".to_string()

                } else {
                    "qkernel doesn't allow policy update".to_string()
                };

                let encodedOutBoundDate = prepareEncodedIoFrame(result.as_bytes(), &self.key).unwrap();
                let mut buf = DataBuff::New(encodedOutBoundDate.len());
                buf.buf = encodedOutBoundDate.clone();            
                return Ok(buf);
            }
        }
        debug!("case5 encryptContainerStdouterr, user_type:{:?}, stdio_type {:?}", user_type, stdio_type);
        let rawData= src.buf.clone();
        //check out here

        let str = String::from_utf8_lossy(&rawData).to_string();

        debug!("stdout is : {:?}", str);
        // main encryption here
        let encodedOutBoundDate = prepareEncodedIoFrame(rawData.as_slice(), &self.key).unwrap();
        assert!(encodedOutBoundDate.len() != 0);
        
        let signing_key = PrivateKey::from_bytes(&self.signing_key_slice).unwrap();
        let signature = signing_log(&encodedOutBoundDate, &signing_key);
        let mut res = DataBuff::New(encodedOutBoundDate.len());


        res.buf = encodedOutBoundDate.clone();

        // for (i, el) in encodedOutBoundDate.iter().enumerate(){
        //     assert!(res.buf[i] == *el);
        // }
        debug!("stdout is : {:?}", res);
        Ok(res)
    }

 */
}

fn main()   {
    let file = File::open("/Users/kaj/Desktop/casetest/src/log.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut data = Vec::new();
    reader.read_to_end(&mut data).unwrap();

    let rng = SystemRandom::new();
    // Generating ECDSA Key Pair
    let pkcs8_bytes = EcdsaKeyPair::generate_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, &rng).unwrap();
    let key_pair = EcdsaKeyPair::from_pkcs8(&ECDSA_P256_SHA256_FIXED_SIGNING, pkcs8_bytes.as_ref()).unwrap();
    println!("Private Key (PKCS#8):");

    let sig = key_pair.sign(&rng, &data).unwrap();
    println!("Signature:");
    // println!("{:?}", sig.as_ref());
    // Verifying the signature
    let peer_public_key_bytes = key_pair.public_key().as_ref();


    let peer_public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ECDSA_P256_SHA256_FIXED, peer_public_key_bytes);
    peer_public_key.verify(&data, sig.as_ref()).unwrap();
    println!("Signature verified successfully!");

    // Ok(())

}