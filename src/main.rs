use anyhow::Error;
use fehler::throws;
use tokio::main;

#[throws]
#[main]
async fn main() {
    env_logger::try_init()?;

    quep_core::Quep::from_env().await?.run().await?;
}
