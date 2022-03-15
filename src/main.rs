use fehler::throws;

#[tokio::main]
#[throws(snafu::Whatever)]
async fn main() {
    // env_logger::init();

    let hello = quep::hello();
    println!("{hello}");
}
