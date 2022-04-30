#[throws]
#[tokio::test]
async fn foobar() {
    let args = CliArgs::builder()
        .provider(ProviderType::Noisy)
        .output(OutputType::Text)
        .output_ser(OutputSerType::Json)
        .circuit(CircuitType::Volume)
        .orch(OrchestratorType::Linear)
        .orch_size(4)
        .orch_size_2(4)
        .python_dir(common::get_dir()?)
        .build();
    quep::Quep::new(args).await?.run().await?;
}
