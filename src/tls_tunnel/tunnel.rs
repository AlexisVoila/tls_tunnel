use crate::tls_tunnel::config;
use crate::tls_tunnel::tls_tunnel;
use crate::tls_tunnel::forwarder;

use std::{io};

pub async fn run(cfg: config::TlsTunnelConfig) -> io::Result<()> {
    if cfg.ca_cert.is_some() {
        println!("Working in tls tunnel mode.");
        tls_tunnel::run(cfg).await?;
    } else {
        println!("Working in tcp port forwarding mode.");
        forwarder::run(cfg).await?;
    }
    Ok(())
}
