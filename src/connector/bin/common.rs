// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0
use cryptoki::context::{CInitializeArgs, Pkcs11};
use cryptoki::session::UserType;
use cryptoki::slot::Slot;
use cryptoki::types::AuthPin;
use std::env;

use openssl;

// use openssl::pkcs11::{Pkcs11, Slot};
use openssl::nid::Nid;
// The default user pin
pub static USER_PIN: &str = "1234";
// The default SO pin
pub static SO_PIN: &str = "1234";

use std::fs;

// fn check_permissions() -> std::io::Result<()> {
//     let metadata = fs::metadata("/usr/local/lib/softhsm/libsofthsm2.so")?;
//     let permissions = metadata.permissions();

//     if permissions.readonly() {
//         println!("The application has READONLY.");
//     } else {
//         println!("The application has read and execute permissions for the file.");
//     }

//     Ok(())
// }

pub fn get_pkcs11() -> Pkcs11 {
    Pkcs11::new( "/usr/local/lib/softhsm/libsofthsm2.so")
    .unwrap()
}

pub fn init_pins() -> (Pkcs11, Slot) {

    let pkcs11client = get_pkcs11();
    // print!("Initialized PKCS11 {:?}\n", pkcs11client);
    let init_result = pkcs11client.initialize(CInitializeArgs::OsThreads);
    
    match init_result {
        Ok(_) => println!("Initialization successful"),
        Err(e) => println!("Initialization failed with CKR: {:?}", e),
    }

    let slot = pkcs11client.get_slots_with_token().unwrap().remove(0);

    let so_pin = AuthPin::new(SO_PIN.into());
    pkcs11client.init_token(slot, &so_pin, "Test Token").unwrap();

    {
        let session = pkcs11client.open_rw_session(slot).unwrap();

        session.login(UserType::So, Some(&so_pin)).unwrap();
        session.init_pin(&AuthPin::new(USER_PIN.into())).unwrap();
    }

    (pkcs11client, slot)
}