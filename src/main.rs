extern crate anyhow;
extern crate env_logger;
extern crate log;
extern crate time;
use discovery::query_proxied_server;

use std::env;
use std::net::SocketAddr;

mod config;
mod discovery;
mod proxy;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let config = config::load(&args);

    let remote_address = config.remote.as_ip_addr();
    let discovery_response = query_proxied_server(&remote_address);
    let local_socket_address = config
        .local
        .as_socket_addrs()
        .last()
        .expect("No address found on specified interface to bind to!");
    let remote_socket_address = SocketAddr::new(remote_address, discovery_response.get_port());
    let disocvery_service_handle = discovery::start(discovery_response, &local_socket_address);
    proxy::run(&local_socket_address, &remote_socket_address);
    disocvery_service_handle.join().unwrap();
}
