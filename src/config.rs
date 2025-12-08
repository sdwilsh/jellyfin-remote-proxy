use std::{fs::File, net::IpAddr, net::Ipv4Addr, net::SocketAddrV4};

use serde::Deserialize;
use serde_yml::from_reader;

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

pub fn load(args: &[String]) -> Config {
    let config_file = &args[1];
    let f = File::open(config_file).expect("Unable to open config file!");
    let deserialized: Config = from_reader(f).unwrap();
    return deserialized;
}
