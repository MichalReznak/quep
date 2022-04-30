use fehler::throws;

#[throws(anyhow::Error)]
#[tokio::main]
async fn main() {
    env_logger::try_init()?;

    quep::Quep::from_env().await?.run().await?;
}
