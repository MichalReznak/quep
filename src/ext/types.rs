use tokio::time::Duration;
use typed_builder::TypedBuilder;

#[derive(TypedBuilder)]
pub struct MetaInfo {
    pub time: Duration,
}
