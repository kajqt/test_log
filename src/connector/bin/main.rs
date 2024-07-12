

use std::ptr;
mod common;
use common::init_pins;
use common::USER_PIN;
use cryptoki::{mechanism::Mechanism, object::Attribute};
use cryptoki::session::UserType;
use cryptoki::types::AuthPin;
fn main() {
  
    // let pkcs11_module_path = "/path/to/your/softhsm2/libsofthsm2.so";
    // let pkcs11_module = unsafe { pkcs11::load_module(pkcs11_module_path).unwrap() };
    let (pkcs11, slot) = init_pins();
    println!("Initialized, {:?}", pkcs11);
    // open a session
    let session = pkcs11.open_rw_session(slot).unwrap();


    session.login(UserType::User, Some(&AuthPin::new(USER_PIN.into())));


    let mechanism = Mechanism::RsaPkcsKeyPairGen;

    let public_exponent: Vec<u8> = vec![0x01, 0x00, 0x01];
    let modulus_bits = 1024;

    // pub key template
    let pub_key_template = vec![
        Attribute::Token(true),
        Attribute::Private(false),
        Attribute::PublicExponent(public_exponent),
        Attribute::ModulusBits(modulus_bits.into()),
    ];

    // priv key template
    let priv_key_template = vec![Attribute::Token(true)];

    // generate a key pair
    let (public, private) =
        session.generate_key_pair(&mechanism, &pub_key_template, &priv_key_template).unwrap();

    // data to sign
    let data = [0xFF, 0x55, 0xDD];

    // sign something with it
    let signature = session.sign(&Mechanism::RsaPkcs, private, &data).unwrap();
    print!("Signature: {:?}\n", signature);
    // verify the signature
    session.verify(&Mechanism::RsaPkcs, public, &data, &signature);

    // delete keys
    session.destroy_object(public);
    session.destroy_object(private);

}
