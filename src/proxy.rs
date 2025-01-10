extern crate time;

use std::net::{SocketAddr, SocketAddrV4};
use std::thread::{self};

use anyhow::Context;
use log::{debug, info};

use sozu_command_lib::{
    channel::Channel,
    config::ListenerBuilder,
    proto::command::{
        request::RequestType, AddBackend, Cluster, HttpListenerConfig, LoadBalancingAlgorithms,
        LoadBalancingParams, PathRule, RequestHttpFrontend, RulePosition, SocketAddress,
        WorkerRequest,
    },
};

const CLUSTER_ID: &'static str = "jellyfin-remote-proxy";

fn get_cluster() -> Cluster {
    return Cluster {
        cluster_id: CLUSTER_ID.to_string(),
        sticky_session: false,
        https_redirect: false,
        load_balancing: LoadBalancingAlgorithms::RoundRobin as i32,
        ..Default::default()
    };
}

fn get_backend(remote_address: &SocketAddr) -> AddBackend {
    return AddBackend {
        cluster_id: CLUSTER_ID.to_string(),
        backend_id: "jellyfin-actual".to_string(),
        address: SocketAddress::from(remote_address.clone()),
        load_balancing_parameters: Some(LoadBalancingParams::default()),
        ..Default::default()
    };
}

fn get_frontend(local_address: &SocketAddrV4) -> RequestHttpFrontend {
    return RequestHttpFrontend {
        cluster_id: Some(CLUSTER_ID.to_string()),
        address: SocketAddress::from(SocketAddr::from(local_address.clone())),
        hostname: local_address.ip().to_string(),
        path: PathRule::prefix(String::from("/")),
        position: RulePosition::Pre.into(),
        ..Default::default()
    };
}

fn get_listener(local_address: &SocketAddrV4) -> HttpListenerConfig {
    return ListenerBuilder::new_http(SocketAddress::from(SocketAddr::from(local_address.clone())))
        .to_http(None)
        .expect("Could not create a listener to proxy Jellyfin!");
}

pub fn run(local_address: &SocketAddrV4, remote_address: &SocketAddr) {
    let (mut command_channel, proxy_channel) = Channel::generate(1000, 10000)
        .with_context(|| "should create a channel!")
        .expect("Creating channels failed!");

    let listener = get_listener(local_address);
    let worker_thread_join_handle = thread::spawn(move || {
        let max_buffers = 500;
        let buffer_size = 16384;
        sozu_lib::http::testing::start_http_worker(
            listener,
            proxy_channel,
            max_buffers,
            buffer_size,
        )
        .expect("The worker could not be started, or has shut down!");
    });

    debug!("Adding cluster...");
    command_channel
        .write_message(&WorkerRequest {
            id: String::from("jellyfin-cluster"),
            content: RequestType::AddCluster(get_cluster()).into(),
        })
        .expect("Could not send AddCluster request!");
    debug!("Adding frontend...");
    command_channel
        .write_message(&WorkerRequest {
            id: String::from("jellyfin-frontend"),
            content: RequestType::AddHttpFrontend(get_frontend(local_address)).into(),
        })
        .expect("Could not send AddHttpFrontend request!");
    debug!("Adding backend...");
    command_channel
        .write_message(&WorkerRequest {
            id: String::from("jellyfin-backend"),
            content: RequestType::AddBackend(get_backend(remote_address)).into(),
        })
        .expect("Could not send AddBackend request!");
    info!(
        "Proxy up serving {:?} on {:?}.",
        remote_address, local_address
    );

    worker_thread_join_handle.join().unwrap();
}
