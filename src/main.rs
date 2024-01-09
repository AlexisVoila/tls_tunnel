use tls_tunnel::{tls_tunnel};

use tokio::io;
use std::{process};

#[tokio::main(worker_threads = 1)]
async fn main() -> io::Result<()> {
    let cfg = tls_tunnel::config::get_args().unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    tls_tunnel::tunnel::run(cfg).await?;
    Ok(())
}
