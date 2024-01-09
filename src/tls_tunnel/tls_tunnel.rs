use crate::tls_tunnel::config;
use crate::tls_tunnel::redirector;

use std::{io};
use std::pin::Pin;
use std::net::{SocketAddr};
use tokio::net::{TcpListener, TcpStream};
use tokio_openssl::{SslStream};
use openssl::ssl::{Ssl, SslConnector, SslFiletype, SslMethod, SslOptions};

pub async fn run(cfg: config::TlsTunnelConfig) -> io::Result<()> {
    let listener = TcpListener::bind(cfg.local_ep).await?;

    loop {
        let cfg_to_move = cfg.clone();
        let (socket, _) = listener.accept().await?;
        let peer_addr = socket.peer_addr()?.to_string();

        tokio::spawn(async move {
            match process(socket, cfg_to_move).await {
                Ok(_) => {},
                Err(err) => println!("Connection from {} to {} closed [{}]", peer_addr, cfg.remote_ep, err),
            }
        });
    };
}

fn prepare_ssl(ca_path: String, cert_path: String, pkey_path: String, remote_addr: SocketAddr) -> Ssl
{
    let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
    builder.set_verify_callback(openssl::ssl::SslVerifyMode::FAIL_IF_NO_PEER_CERT, |_,_| { true });
    builder.set_ca_file(ca_path).expect("CA certificate not found!");
    builder.set_certificate_file(cert_path, SslFiletype::PEM).expect("Client certificate not found!");
    builder.set_private_key_file(pkey_path, SslFiletype::PEM).expect("Client private key not found!");
    builder.set_options(SslOptions::NO_TLSV1_1 | SslOptions::NO_TLSV1_2);

    let connector = builder.build();
    let connect_config = connector.configure().unwrap().verify_hostname(false).into_ssl(remote_addr.to_string().as_str()).unwrap();
    return connect_config;
}

async fn process(local_stream: TcpStream, cfg: config::TlsTunnelConfig) -> Result<String, io::Error> {
    let ssl = prepare_ssl(cfg.ca_cert.unwrap(), cfg.client_cert.unwrap(), cfg.client_private_key.unwrap(), cfg.remote_ep);
    let remote_stream = TcpStream::connect(cfg.remote_ep).await?;

    println!("Connection from {} to {} established", local_stream.peer_addr()?.to_string(), cfg.remote_ep);

    let mut remote_stream = SslStream::new(ssl, remote_stream).unwrap();

    Pin::new(&mut remote_stream).connect().await.unwrap();
    redirector::run_forwarding_loop(local_stream, remote_stream).await?;
    Ok("".to_string())
}
