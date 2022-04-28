use fehler::throws;

#[throws(anyhow::Error)]
#[tokio::main]
async fn main() {
    env_logger::try_init()?;

    quep::Quep::new().await?.run().await?;
}
