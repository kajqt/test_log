use cryptoki::{context::{Pkcs11, CInitializeArgs}, context::Function, types, slot::Slot};
use std::env;
fn main()  {
    // Path to the BouncyHSM PKCS#11 library
    let pkcs11_lib_path = "/Users/kaj/Desktop/hsmtest/BouncyHsm/native/Linux-x64/BouncyHsm.so";
    let pkcs11 = Pkcs11::new(env::var("PKCS11_HSM_MODULE")
       .unwrap_or_else(|_| "/Users/kaj/Desktop/hsmtest/BouncyHsm/native/Linux-x64/BouncyHsm.so".to_string())
    ).unwrap();

    // // Initialize the PKCS11 library
    // let context = Context::new(InitializeArgs::OsThreads)?;

    // // Get the list of slots
    // let slots = context.function_list().get_slot_list(true)?;
}