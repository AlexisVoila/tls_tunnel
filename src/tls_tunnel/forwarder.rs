use crate::tls_tunnel::config;
use crate::tls_tunnel::redirector;

use std::{io};
use std::net::{SocketAddr};
use tokio::net::{TcpListener, TcpStream};

pub async fn run(cfg: config::TlsTunnelConfig) ->  io::Result<()> {
    let listener = TcpListener::bind(cfg.local_ep).await?;

    loop {
        let (socket, _) = listener.accept().await?;
        let peer_addr = socket.peer_addr()?.to_string();
        tokio::spawn(async move {
            match process(socket, cfg.remote_ep).await {
                Ok(_) => {},
                Err(err) => println!("Connection from {} to {} closed [{}]", peer_addr, cfg.remote_ep, err),
            }
        });
    };
}

async fn process(local_stream: TcpStream, remote_addr: SocketAddr) -> Result<String, io::Error> {
    let remote_stream = TcpStream::connect(remote_addr).await?;
    println!("Connection from {} to {remote_addr} established", local_stream.peer_addr()?.to_string());

    redirector::run_forwarding_loop(local_stream, remote_stream).await?;
    Ok("".to_string())
}
