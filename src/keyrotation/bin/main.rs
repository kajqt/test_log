use openssl::rsa::{Rsa, Padding};
use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::error::ErrorStack;
use openssl::x509::{X509, X509Name};
use openssl::stack::Stack;
use std::fs::File;
use std::io::Write;


pub fn create_certificate(pkey: &PKey<openssl::pkey::Private>, cert_name: &str, cert_path: &str) -> Result<X509, ErrorStack> {
    
    let mut name = X509Name::builder().unwrap();
    name.append_entry_by_text("CN", cert_name).unwrap();
    let name = name.build();
    // builder.set_subject_name(&name).unwrap();

    let mut builder = X509::builder().unwrap();
    builder.set_version(0).unwrap();
    builder.set_pubkey(&pkey).unwrap();
    builder.set_subject_name(&name).unwrap();

    builder.sign(pkey, MessageDigest::sha256()).unwrap();

    // Build the certificate
    let cert = builder.build();

    // Save the certificate to a file
    let pem = cert.to_pem().unwrap();
    let mut file = File::create(cert_path).unwrap();
    file.write_all(&pem).unwrap();
    Ok(cert)
}

pub fn show_certificate(cert: &X509) {
    let pem = cert.to_pem().unwrap();
    println!("{}", String::from_utf8(pem).unwrap());
}

fn main() {
    // Generate the first private key
    let rsa = Rsa::generate(3072).unwrap();
    let pkey1 = PKey::from_rsa(rsa.clone()).unwrap();
    let pub_key: Vec<u8> = pkey1.public_key_to_pem().unwrap();
    
    // Generate the second private key
    let rsa2 = Rsa::generate(3072).unwrap();
    // let pkey2 = PKey::from_rsa(rsa2.clone()).unwrap();
    let cert1 = create_certificate(&pkey1, "Intermediate K2", "K2_example.crt").unwrap();

    show_certificate(&cert1)
    
  
}