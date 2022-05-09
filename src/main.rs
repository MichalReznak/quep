use anyhow::Error;
use fehler::throws;
use tokio::main;

#[throws]
#[main]
async fn main() {
    env_logger::try_init()?;

    app::Quep::from_env().await?.run().await?;
}
