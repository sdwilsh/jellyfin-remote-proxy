use std::{net::IpAddr, net::Ipv4Addr, net::SocketAddrV4};

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
    pub address: String,
    pub port: u16,
}

impl Local {
    pub fn as_ip_addr(&self) -> Ipv4Addr {
        return self
            .address
            .parse()
            .expect("Unable to parse supplied local ip address!  This should be an IP addrdes.");
    }

    pub fn as_socket_addr(&self) -> SocketAddrV4 {
        return SocketAddrV4::new(self.as_ip_addr(), self.port);
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
