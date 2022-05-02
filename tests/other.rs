use anyhow::Error;
use fehler::throws;
use tokio::test;

#[throws]
#[test]
async fn new() {
    quep::Quep::from_env().await?.run().await?;
}
