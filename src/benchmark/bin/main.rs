use std::sync::Arc;

use std::net::TcpStream;
use std::io::{Read, Write, stdout};
use std::fs::File;
use rustls;
use webpki;
use rustls::quic::Version;
use webpki_roots;
use rustls::RootCertStore;
use rustls::{ClientConfig};
use std::path::Path;
use std::time::Instant;
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
    
    let mut config = rustls::ClientConfig::builder();
    
    let mut root_store = RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    //Loading Additional Certificates
    let file = File::open("/home/ubuntu/aprototype/certs/newcombine.crt").unwrap();
    let mut pem = std::io::BufReader::new(file);
    // let certs = rustls_pemfile::certs(&mut pem);

    // let mut pem = BufReader::new(File::open(cafile)?);
    for cert in rustls_pemfile::certs(&mut pem) {
        root_store.add(cert.unwrap()).unwrap();
    }
        // println!("Number of items in root_store: {}", root_store.len());
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
    
    // let dns_name = webpki::DnsNameRef::try_from_ascii_str("www.wannacoffee.com").unwrap();

    let mut sock = TcpStream::connect("www.wannacoffee.com:443").unwrap();

    let mut conn = rustls::ClientConnection::new(Arc::new(finalconfig), server_name).unwrap();

    // let mut client = rustls::ClientSession::new(&arc, dns_name);
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    let  num_requests = 0;
    let start = Instant::now();
    //Sending HTTP Request
    tls.write(
        concat!(git 
            "GET / HTTP/1.1\r\n",
            "Host:  www.wannacoffee.com\r\n",
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


    let duration = start.elapsed();

    println!("Total time taken for {} requests: {:?}", num_requests, duration);

    // Ok(())
}