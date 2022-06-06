use tokio::time::Duration;
use typed_builder::TypedBuilder;

pub mod lang_schema;
pub mod circuit_generator;

#[derive(TypedBuilder)]
pub struct MetaInfo {
    pub time: Duration,
}
