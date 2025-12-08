use std::net::SocketAddr;

use futures::FutureExt;
use log::{error, info, trace};
use tokio::{io::copy_bidirectional, net::TcpListener, net::TcpStream};

pub async fn run(
    local_address: &SocketAddr,
    remote_address: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(local_address)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "Unable to bind to local address {}!\n{}",
                local_address.to_string(),
                err
            )
        });
    info!("Listening on {} for Jellyfin connections...", local_address);
    while let Ok((mut inbound, from)) = listener.accept().await {
        trace!("New request from {} on {}.", from, local_address);
        let mut outbound = TcpStream::connect(remote_address.clone())
            .await
            .unwrap_or_else(|error| panic!("Unable to connect to {}:\n{}", remote_address, error));
        tokio::spawn(async move {
            copy_bidirectional(&mut inbound, &mut outbound)
                .map(|r| {
                    if let Err(err) = r {
                        error!("Failed to transer data!\n{}", err)
                    }
                })
                .await
        });
    }
    Ok(())
}
