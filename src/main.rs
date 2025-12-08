extern crate env_logger;
extern crate log;
extern crate tokio;
use discovery::query_proxied_server;

use std::env;
use std::net::SocketAddr;

mod config;
mod discovery;
mod proxy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = config::load(&args).await.expect("Unable to load config!");

    let remote_address = config.remote.as_ip_addr();
    let discovery_response = query_proxied_server(&remote_address)
        .await
        .expect("No valid response from remote server!");
    let local_socket_address = config.local.as_socket_addr();
    let remote_socket_address = SocketAddr::new(remote_address, discovery_response.get_port());
    discovery::start(discovery_response, &local_socket_address)
        .await
        .expect("Failed to setup discovery!");
    proxy::run(&local_socket_address, remote_socket_address)
        .await
        .expect("Failed to proxy!");
    Ok(())
}
