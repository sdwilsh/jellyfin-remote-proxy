use std::{fs::File, net::IpAddr, net::Ipv4Addr, net::SocketAddrV4};

use serde::Deserialize;
use serde_yaml::from_reader;

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
    pub interface: String,
    pub port: u16,
}

impl Local {
    pub fn as_ip_addrs(&self) -> impl Iterator<Item = Ipv4Addr> + '_ {
        return nix::ifaddrs::getifaddrs()
            .expect("Unable to get system interfaces!")
            .filter(|ifaddr| {
                ifaddr.interface_name == self.interface.as_str()
                    && ifaddr
                        .address
                        .is_some_and(|ss| ss.as_sockaddr_in().is_some())
            })
            .map(|ifaddr| Ipv4Addr::from(ifaddr.address.unwrap().as_sockaddr_in().unwrap().ip()));
    }

    pub fn as_socket_addrs(&self) -> impl Iterator<Item = SocketAddrV4> + '_ {
        return self
            .as_ip_addrs()
            .map(|ipaddr| SocketAddrV4::new(ipaddr, self.port));
    }
}

pub fn load(args: &[String]) -> Config {
    let config_file = &args[1];
    let f = File::open(config_file).expect("Unable to open config file!");
    let deserialized: Config = from_reader(f).unwrap();
    return deserialized;
}
