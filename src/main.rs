use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    zero2prod::run()?.await.map_err(|e| anyhow::anyhow!(e))
}
