use std::error::Error;
use std::net::{SocketAddr};
use std::str::FromStr;

use clap::{App, Arg};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct TlsTunnelConfig {
    pub local_ep: SocketAddr,
    pub remote_ep: SocketAddr,
    pub ca_cert: Option<String>,
    pub client_private_key: Option<String>,
    pub client_cert: Option<String>,
}

fn parse_tunnel_addr_info(val: &str) -> MyResult<String> {
    let splited = val.split(':').collect::<Vec<&str>>();

    if splited.len() != 3 {
        return Err(From::from(format!("Invalid tunnel endpoints info format specified \"{}\"", val)));
    }

    Ok(val.to_string())
}

fn check_port<'a>(port: &'a str, name: &'a str) -> MyResult<&'a str> {
    match port.parse::<u16>() {
        Ok(p) if p > 0 => Ok(port),
        Ok(_) => Err(From::from(format!("Parameter {} argument \"{}\"", name, port))),
        Err(e) => Err(From::from(format!("Parameter {} argument \"{}\" -- {}", name, port, e))),
    }
}

fn check_ip_addr<'a>(ip: &'a str, name: &'a str) -> MyResult<&'a str> {
    match std::net::IpAddr::from_str(ip) {
        Ok(_) => Ok(ip),
        Err(e) => Err(From::from(format!("Parameter {} argument \"{}\" -- {}", name, ip, e))),
    }
}

fn parse_endpoints(ep_str: String) -> Result<(SocketAddr, SocketAddr), Box<dyn Error>> {
    let splited = ep_str.split(':').collect::<Vec<&str>>();

    let listen_port = check_port(splited[0], "listen_port")?;
    let remote_ip = check_ip_addr(splited[1], "remote_ip")?;
    let remote_port = check_port(splited[2], "remote_port")?;

    let local_ep = SocketAddr::from_str(["0.0.0.0", listen_port].join(":").as_str())?;
    let remote_ep = SocketAddr::from_str([remote_ip, remote_port].join(":").as_str())?;

    Ok((local_ep, remote_ep))
}

pub fn get_args() -> MyResult<TlsTunnelConfig> {
    let matches = App::new("tls_tunnel")
        .version("0.1.0")
        .author("Alexey Popov")
        .about("TLS Port Forwarder")
        .arg(
            Arg::with_name("tunnel")
                .short("l")
                .long("tunnel")
                .required(true)
                .takes_value(true)
                .value_name("LOCAL_PORT:REMOTE_IP:REMOTE_PORT")
                .help("Tunnel endpoints info string")
        )
        .arg(
            Arg::with_name("ca_cert")
                .short("c")
                .long("ca-cert")
                .value_name("FILE")
                .takes_value(true)
                .requires("client_private_key")
                .requires("client_cert")
                .help("CA certificate")
        )
        .arg(
            Arg::with_name("client_private_key")
                .short("k")
                .long("client-private-key")
                .value_name("FILE")
                .takes_value(true)
                .requires("ca_cert")
                .requires("client_cert")
                .help("Client private key")
        )
        .arg(
            Arg::with_name("client_cert")
                .short("e")
                .long("client-cert")
                .value_name("FILE")
                .takes_value(true)
                .requires("ca_cert")
                .requires("client_private_key")
                .help("Client certificate")
        )
        .get_matches();

    let tun_addr_info = matches
        .value_of("tunnel")
        .map(parse_tunnel_addr_info)
        .transpose()
        .map_err(|e| e)?.unwrap();

    let (lep, rep) = parse_endpoints(tun_addr_info)?;

    Ok(TlsTunnelConfig {
        local_ep: lep,
        remote_ep: rep,
        ca_cert: matches.value_of("ca_cert").map(|s| String::from(s)),
        client_private_key: matches.value_of("client_private_key").map(|s| String::from(s)),
        client_cert: matches.value_of("client_cert").map(|s| String::from(s)),
    })
}
