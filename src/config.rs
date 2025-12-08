use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

use serde::Deserialize;
use tokio::{fs::File, io::AsyncReadExt};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub remote: Remote,
    pub local: Local,
}

#[derive(Debug, Deserialize)]
pub struct Remote {
    pub address: String,
}

impl Remote {
    pub fn as_ip_addr(&self) -> IpAddr {
        return self
            .address
            .parse()
            .expect("Unable to parse supplied remote address!  This should be an IP address.");
    }
}

#[derive(Debug, Deserialize)]
pub struct Local {
    pub address: Option<String>,
    pub port: u16,
}

impl Local {
    pub fn as_ip_addr(&self) -> Result<IpAddr, Box<dyn std::error::Error>> {
        match &self.address {
            None => Ok(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
            Some(maybe_addr) => match maybe_addr.parse::<Ipv4Addr>() {
                Ok(ipv4_addr) => Ok(IpAddr::V4(ipv4_addr)),
                Err(_) => match maybe_addr.parse::<Ipv6Addr>() {
                    Ok(ipv6_addr) => Ok(IpAddr::V6(ipv6_addr)),
                    Err(e) => panic!("Unable to parse ip address '{}':\n{}", maybe_addr, e),
                },
            },
        }
    }

    pub fn as_socket_addr(&self) -> SocketAddr {
        match self.as_ip_addr().unwrap() {
            IpAddr::V4(addr) => SocketAddr::V4(SocketAddrV4::new(addr, self.port)),
            IpAddr::V6(addr) => SocketAddr::V6(SocketAddrV6::new(addr, self.port, 0, 0)),
        }
    }
}

pub async fn load(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
    let config_file = &args[1];
    let mut contents = String::new();
    File::open(config_file)
        .await
        .expect("Unable to open config file!")
        .read_to_string(&mut contents)
        .await
        .unwrap_or_else(|error| panic!("Unable to read config file {}!\n{}", config_file, error));
    let deserialized: Config = serde_yml::from_str(&contents).unwrap();
    Ok(deserialized)
}
