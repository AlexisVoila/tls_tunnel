use crate::tls_tunnel::config;
use crate::tls_tunnel::tls_tunnel;
use crate::tls_tunnel::forwarder;

use std::{io};

pub async fn run(cfg: config::TlsTunnelConfig) -> io::Result<()> {
    if cfg.ca_cert.is_some() {
        tls_tunnel::run(cfg).await?;
    } else {
        forwarder::run(cfg).await?;
    }
    Ok(())
}
