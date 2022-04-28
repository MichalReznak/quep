use anyhow::Error;
use fehler::throws;

#[throws]
#[tokio::test]
async fn foo() {
    quep::Quep::new().await?.run().await?;
}
