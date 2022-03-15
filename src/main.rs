use fehler::throws;

#[throws(anyhow::Error)]
#[tokio::main]
async fn main() {
    env_logger::init();

    let hello = quep::hello()?;
    println!("{hello}");
}
