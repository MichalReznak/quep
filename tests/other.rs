use anyhow::Error;
use fehler::throws;
use tokio::test;

use crate::Quep;

#[throws]
#[test]
async fn new() {
    Quep::from_env().await?.run().await?;
}
