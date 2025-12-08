use std::net::SocketAddr;

use async_http_proxy::http_connect_tokio;
use tokio::net::TcpStream;

pub async fn run(
    local_address: &SocketAddr,
    remote_address: &SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(local_address)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "Unable to bind to local address {}!\n{}",
                local_address.to_string(),
                err
            )
        });
    http_connect_tokio(
        &mut stream,
        &remote_address.ip().to_string(),
        remote_address.port(),
    )
    .await
    .expect("Error proxying request!");
    Ok(())
}
