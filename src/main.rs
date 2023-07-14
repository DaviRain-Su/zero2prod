use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    zero2prod::run("127.0.0.1:8080")?
        .await
        .map_err(|e| anyhow::anyhow!(e))
}
