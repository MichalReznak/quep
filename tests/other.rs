use anyhow::Error;
use app::Quep;
use fehler::throws;
use tokio::test;

#[throws]
#[test]
async fn new() {
    Quep::from_env().await?.run().await?;
}

// TODO add collect: true tests
