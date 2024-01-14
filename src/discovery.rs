use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::str;
use std::thread::{self, JoinHandle};

use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json;
use url::Url;

const ANY_PORT: u16 = 0;
const DISCOVERY_QUERY: &'static str = "Who is JellyfinServer?";
const DISCOVERY_PORT: u16 = 7359;

#[derive(Debug, Deserialize, Serialize)]
pub struct DiscoveryResponse {
    #[serde(rename(deserialize = "Address", serialize = "Address"))]
    pub address: String,
    #[serde(rename(deserialize = "Id", serialize = "Id"))]
    pub id: String,
    #[serde(rename(deserialize = "Name", serialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "EndpointAddress", serialize = "EndpointAddress"))]
    pub endpoint_address: Option<String>,
}

impl DiscoveryResponse {
    pub fn get_port(&self) -> u16 {
        return Url::parse(self.address.as_str())
            .expect("Unable to parse discovery response address from remote server!")
            .port_or_known_default()
            .expect("Unknown port for remote server!");
    }
}

pub fn query_proxied_server(remote_address: &IpAddr) -> DiscoveryResponse {
    let discovery_addr = SocketAddr::new(*remote_address, DISCOVERY_PORT);
    info!("Querying {:?} for Jellyfin information...", discovery_addr);
    let discovery_socket =
        UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), ANY_PORT))
            .expect("Could not create UDP socket!");
    discovery_socket
        .send_to(DISCOVERY_QUERY.as_bytes(), discovery_addr)
        .expect("Unable to connect to remote Jellyfin server!");

    let mut buf = [0; 1024];
    let bytes = discovery_socket
        .recv(&mut buf)
        .expect("Failed to receive data from remote Jellyfin server!");
    debug!("Received {:?} bytes from {:?}", bytes, discovery_addr);

    let deserialized: DiscoveryResponse =
        serde_json::from_slice(&buf[0..bytes]).expect("Unable to deserialize server response!");
    return deserialized;
}

pub fn start(
    discovery_response: DiscoveryResponse,
    local_socket_address: &SocketAddr,
) -> JoinHandle<()> {
    let response = serde_json::to_string(&DiscoveryResponse {
        address: format!("http://{}", local_socket_address.to_string().as_str()),
        ..discovery_response
    })
    .expect("Unable to serialize local discovery response!");

    let worker_thread_join_handle = thread::spawn(move || {
        let discovery_addr: SocketAddr = (IpAddr::V4(Ipv4Addr::UNSPECIFIED), DISCOVERY_PORT).into();
        let socket =
            UdpSocket::bind(discovery_addr).expect("Could not bind to address for auto-discovery!");

        let mut buf = [0; DISCOVERY_QUERY.as_bytes().len()];
        while let Ok((_, address)) = socket.recv_from(&mut buf) {
            debug!("Received potential discovery request from {:?}", address);
            let message = match str::from_utf8(&buf) {
                Ok(v) => v,
                Err(e) => {
                    debug!("Not responding to {:?} because :{:?}", address, e);
                    continue;
                }
            };
            if !message.eq_ignore_ascii_case(DISCOVERY_QUERY) {
                debug!(
                    "Not responding to {:?} because `{}` does not match the expected query!",
                    address, message
                );
                continue;
            }
            info!("Responding to {:?} for a discovery query.", address);
            socket
                .send_to(response.as_bytes(), address)
                .expect("Unable to connect to respond to query!");
        }
        debug!("Done receivng from socket.")
    });

    return worker_thread_join_handle;
}
