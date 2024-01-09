use std::{io};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, split};
use std::io::{Error, ErrorKind};

pub async fn run_forwarding_loop<Tl, Tr>(local_stream: Tl, remote_stream: Tr) -> Result<String, io::Error>
    where Tl: AsyncRead + AsyncWrite, Tr: AsyncRead + AsyncWrite,
{
    let (mut lrd, mut lwd) = split(local_stream);
    let (mut rrd, mut rwd) = split(remote_stream);

    let mut src_buf = vec![0; 0x8000];
    let mut dst_buf = vec![0; 0x8000];

    loop {
        tokio::select! {
            res = async {
                match lrd.read(&mut src_buf).await {
                    Ok(0) => Err(Error::new(ErrorKind::Other, "Ok")),
                    Ok(n) => rwd.write_all(&src_buf[..n]).await.map_err(|e|e),
                    Err(e) => Err(e)
                }
            } => {res?}
            res = async {
                match rrd.read(&mut dst_buf).await {
                    Ok(0) => Err(Error::new(ErrorKind::Other, "Ok")),
                    Ok(n) => lwd.write_all(&dst_buf[..n]).await.map_err(|e|e),
                    Err(e) => Err(e)
                }
            } => {res?}
            else => {}
        }
    }
}
