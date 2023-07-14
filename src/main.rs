use anyhow::Result;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    tracing::info!("port: {:?}", port);

    zero2prod::run(listener)?
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
