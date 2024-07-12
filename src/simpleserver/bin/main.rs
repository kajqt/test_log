//! This is the simplest possible server using rustls that does something useful:
//! it accepts the default configuration, loads a server certificate and private key,
//! and then accepts a single client connection.
//!
//! Usage: cargo r --bin simpleserver <path/to/cert.pem> <path/to/privatekey.pem>
//!
//! Note that `unwrap()` is used to deal with networking errors; this is not something
//! that is sensible outside of example code.

use std::env;
use std::error::Error as StdError;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::Arc;

// use cryptoki::session::{SessionHandle, SessionType};
// use cryptoki::token::Token;
// use cryptoki::util::Pin;

fn main() -> Result<(), Box<dyn StdError>> {

    // let mut ck = cryptoki::CkContext::new(None)?;

    // // Connect to the SoftHSM module
    // let token_label = "token-label"; // Replace with your token label
    // let session = ck.open_session(&token_label, Pin::new("1234"))?;

    // // Retrieve the public key object
    // let public_key_object = session.get_public_key_object(1001)?; // Replace 1001 with your key ID

    // // Print the public key
    // println!("Public Key: {:?}", public_key_object);

    let mut args = env::args();
    args.next();
    // let cert_file = args
    //     .next()
    //     .expect("missing certificate file argument");
    let cert_file = "/home/ubuntu/aprototype/certs/newcombine.crt";
    let private_key_file = args
        .next()
        .expect("missing private key file argument");

    let certs = rustls_pemfile::certs(&mut BufReader::new(&mut File::open(cert_file)?))
        .collect::<Result<Vec<_>, _>>()?;
    let private_key =
        rustls_pemfile::private_key(&mut BufReader::new(&mut File::open(private_key_file)?))?
            .unwrap();
    let config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, private_key)?;

    let listener = TcpListener::bind(format!("[::]:{}", 4443)).unwrap();
    let (mut stream, _) = listener.accept()?;

    let mut conn = rustls::ServerConnection::new(Arc::new(config))?;
    conn.complete_io(&mut stream)?;

    conn.writer()
        .write_all(b"Hello from the server")?;
    conn.complete_io(&mut stream)?;
    let mut buf = [0; 64];
    let len = conn.reader().read(&mut buf)?;
    println!("Received message from client: {:?}", &buf[..len]);

    Ok(())
}