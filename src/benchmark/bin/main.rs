use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write, stdout};
use std::fs::File;
use rustls;
use webpki;
use webpki_roots;
use rustls::RootCertStore;
use rustls::{ClientConfig};
use std::path::Path;
// use rustls::HandshakeType::Certificate;
// use rustls::Certificate;
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};

use std::io::{self, BufReader}; 

fn load_certs(filename: &str) -> Vec<CertificateDer<'static>> {
    let certfile = File::open(filename).expect("cannot open certificate file");
    let mut reader = BufReader::new(certfile);
    rustls_pemfile::certs(&mut reader)
        .map(|result| result.unwrap())
        .collect()
}

fn main() {
    let mut socket = std::net::TcpStream::connect("www.google.com:443").unwrap();
    let mut config = rustls::ClientConfig::builder();
    
    let root_store = RootCertStore::empty();
    let file = File::open("/home/ubuntu/aprototype/certs/root.ca.cert.pem").unwrap();
    let mut pem = std::io::BufReader::new(file);
    let certs = rustls_pemfile::certs(&mut pem);

    // for r in certs.into_iter(){
    //     match r {
    //         Ok(cert) => {
    //             root_store.add(Certificate(cert));
    //         }
    //         Err(e) => {
               
    //         }
    //     }
    // }   
    let server_name = "www.wannacoffee.com".try_into().unwrap();

    let finalconfig = config
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    let dns_name = webpki::DnsNameRef::try_from_ascii_str("www.wannacoffee.com").unwrap();

    let mut sock = TcpStream::connect("www.wannacoffee.com:443").unwrap();

    let mut conn = rustls::ClientConnection::new(Arc::new(finalconfig), server_name).unwrap();

    // let mut client = rustls::ClientSession::new(&arc, dns_name);
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    tls.write(
        concat!(
            "GET / HTTP/1.1\r\n",
            "Host: www.rust-lang.org\r\n",
            "Connection: close\r\n",
            "Accept-Encoding: identity\r\n",
            "\r\n"
        )
        .as_bytes(),
    )
    .unwrap();
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    io::stdout().write_all(&plaintext).unwrap();
    
}